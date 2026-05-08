use std::collections::BTreeMap;
use std::fs;
use std::sync::{Mutex, OnceLock};

use clap::{Args, Subcommand};
use serde_json::{Map, Value, json};

use crate::client::{ClientAuthentication, NetBoxClient};
use crate::config::{
    ConfigContext, ConfigOverrides, NbxConfig, load_config, redact_token, resolve_connection,
    resolve_optional_connection, save_config,
};
use crate::envelope::data_envelope;
use crate::error::{NbxError, NbxResult};
use crate::generated::endpoints::{ENDPOINTS, EndpointDescriptor, HttpMethod};
use crate::output::{OutputFormat, emit_page, emit_single, print_json, print_json_line};
use crate::projection::{apply_projection, parse_field_projection};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
struct ClientCacheKey {
    url: String,
    token: String,
    timeout_seconds: u64,
}

static CLIENT_CACHE: OnceLock<Mutex<BTreeMap<ClientCacheKey, NetBoxClient>>> = OnceLock::new();

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PrefixAvailabilityEndpoint {
    AvailableIps,
    AvailablePrefixes,
}

impl PrefixAvailabilityEndpoint {
    fn path_template(self) -> &'static str {
        match self {
            Self::AvailableIps => PREFIX_AVAILABLE_IPS_PATH,
            Self::AvailablePrefixes => PREFIX_AVAILABLE_PREFIXES_PATH,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PrefixAvailabilityOperation {
    List,
    Allocate,
}

impl PrefixAvailabilityOperation {
    fn from_body(body: Option<&Value>) -> Self {
        if body.is_some() {
            Self::Allocate
        } else {
            Self::List
        }
    }

    fn http_method(self) -> HttpMethod {
        match self {
            Self::List => HttpMethod::Get,
            Self::Allocate => HttpMethod::Post,
        }
    }
}

#[derive(Debug, Clone, Args)]
pub struct GlobalOptions {
    /// `NetBox` base URL, for example `https://netbox.example`.
    #[arg(long, global = true)]
    pub url: Option<String>,

    /// `NetBox` API token. Prefer `NETBOX_TOKEN` or `nbx config set-token` for shared terminals.
    #[arg(long, global = true)]
    pub token: Option<String>,

    /// Config context to use, overriding `NBX_CONTEXT` and the default context.
    #[arg(long, global = true)]
    pub context: Option<String>,

    /// Output format. Defaults to table on a TTY and JSON when piped.
    #[arg(long, value_enum, global = true)]
    pub output: Option<OutputFormat>,

    /// Comma-separated field projection, with dot notation for nested fields.
    #[arg(long, global = true)]
    pub field: Option<String>,

    /// Shorthand field projection, for example `-o name`.
    #[arg(short = 'o', value_name = "FIELD", global = true)]
    pub output_field: Option<String>,

    /// Raw `NetBox` query string to pass through, for example `site=dc1&status=active`.
    #[arg(long, global = true)]
    pub query: Option<String>,

    /// Page size for list requests.
    #[arg(long, global = true)]
    pub limit: Option<u64>,

    /// Page offset for list requests.
    #[arg(long, global = true)]
    pub offset: Option<u64>,

    /// Stream every page as NDJSON records.
    #[arg(long, global = true)]
    pub page_all: bool,

    /// HTTP request timeout in seconds.
    #[arg(long, default_value_t = 30, global = true)]
    pub timeout: u64,

    /// Enable debug logging.
    #[arg(short = 'v', long, global = true)]
    pub verbose: bool,

    /// Print the request nbx would send without contacting `NetBox`.
    #[arg(long, global = true)]
    pub dry_run: bool,
}

// ============================================================
// Raw API requests
// ============================================================

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RawHttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}

impl RawHttpMethod {
    fn http_method(self) -> HttpMethod {
        match self {
            Self::Get => HttpMethod::Get,
            Self::Post => HttpMethod::Post,
            Self::Patch => HttpMethod::Patch,
            Self::Delete => HttpMethod::Delete,
        }
    }

    fn as_str(self) -> &'static str {
        self.http_method().as_str()
    }
}

#[derive(Debug, Args)]
pub struct RawArgs {
    /// HTTP method to use: GET, POST, PATCH, or DELETE.
    #[arg(value_parser = parse_raw_http_method)]
    pub method: RawHttpMethod,

    /// `NetBox` API path, for example `/api/plugins/example/widgets/`.
    pub path: String,

    /// Free-form JSON request body.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    /// Read the free-form JSON request body from a file.
    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

impl RawArgs {
    fn has_request_body(&self) -> bool {
        self.data.is_some() || self.data_file.is_some()
    }

    fn request_body(self) -> NbxResult<Option<Value>> {
        optional_json_body(self.data, self.data_file)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RawApiPath(String);

impl RawApiPath {
    fn parse(path_without_query: &str, original_raw_path: &str) -> NbxResult<Self> {
        if path_without_query.starts_with("/api/") {
            Ok(Self(path_without_query.to_owned()))
        } else if path_without_query.starts_with("api/") {
            Ok(Self(format!("/{path_without_query}")))
        } else {
            Err(NbxError::validation(
                "raw path must be a NetBox API path starting with /api/",
                json!({ "path": original_raw_path }),
            ))
        }
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RawRequestTarget {
    api_path: RawApiPath,
    query_parameters: Vec<(String, String)>,
}

fn parse_raw_http_method(method: &str) -> Result<RawHttpMethod, String> {
    if method.eq_ignore_ascii_case("GET") {
        Ok(RawHttpMethod::Get)
    } else if method.eq_ignore_ascii_case("POST") {
        Ok(RawHttpMethod::Post)
    } else if method.eq_ignore_ascii_case("PATCH") {
        Ok(RawHttpMethod::Patch)
    } else if method.eq_ignore_ascii_case("DELETE") {
        Ok(RawHttpMethod::Delete)
    } else {
        Err("expected one of GET, POST, PATCH, or DELETE".to_owned())
    }
}

// ============================================================
// Config
// ============================================================

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Initialize or replace the default config.
    Init(ConfigInitArgs),
    /// Set the URL on the selected or default context.
    SetUrl(ConfigValueArgs),
    /// Set the API token on the selected or default context.
    SetToken(ConfigValueArgs),
    /// Show the active config.
    Show(ConfigShowArgs),
    /// Set the default context.
    UseContext(ContextNameArgs),
    /// Print the active context name.
    CurrentContext,
    /// List configured contexts.
    GetContexts,
}

#[derive(Debug, Args)]
pub struct ConfigInitArgs {
    #[arg(long)]
    pub url: Option<String>,

    #[arg(long)]
    pub token: Option<String>,

    #[arg(long, default_value = "default")]
    pub context: String,
}

#[derive(Debug, Args)]
pub struct ConfigValueArgs {
    pub value: String,

    #[arg(long)]
    pub context: Option<String>,
}

#[derive(Debug, Args)]
pub struct ConfigShowArgs {
    #[arg(long)]
    pub show_token: bool,
}

#[derive(Debug, Args)]
pub struct ContextNameArgs {
    pub name: String,
}

// ============================================================
// Resource specs
// ============================================================

#[derive(Debug, Clone, Copy)]
pub struct ResourceSpec {
    pub app: &'static str,
    pub name: &'static str,
    pub api_path: &'static str,
    pub detail_path: &'static str,
    pub default_lookup_field: &'static str,
}

#[derive(Debug, Clone)]
struct ResolvedResourceId {
    path_segment: String,
    dry_run_id_resolution: Option<Value>,
}

impl ResolvedResourceId {
    fn known(id: impl Into<String>) -> Self {
        Self {
            path_segment: id.into(),
            dry_run_id_resolution: None,
        }
    }

    fn dry_run_lookup(resource: ResourceSpec, lookup_field: &str, lookup_value: &str) -> Self {
        let query_parameters = lookup_query_parameters(lookup_field, lookup_value);
        Self {
            path_segment: "{resolved-id}".to_owned(),
            dry_run_id_resolution: Some(json!({
                "resource": format!("{}.{}", resource.app, resource.name),
                "lookup": lookup_value,
                "lookupField": lookup_field,
                "method": HttpMethod::Get.as_str(),
                "path": resource.api_path,
                "query": query_parameters,
            })),
        }
    }

    fn path_segment(&self) -> &str {
        &self.path_segment
    }

    fn dry_run_id_resolution(&self) -> Option<Value> {
        self.dry_run_id_resolution.clone()
    }
}

pub const DEVICES: ResourceSpec = ResourceSpec {
    app: "dcim",
    name: "devices",
    api_path: "/api/dcim/devices/",
    detail_path: "/api/dcim/devices/{id}/",
    default_lookup_field: "name",
};

pub const INTERFACES: ResourceSpec = ResourceSpec {
    app: "dcim",
    name: "interfaces",
    api_path: "/api/dcim/interfaces/",
    detail_path: "/api/dcim/interfaces/{id}/",
    default_lookup_field: "name",
};

pub const INTERFACES_TRACE_PATH: &str = "/api/dcim/interfaces/{id}/trace/";
pub const PREFIX_AVAILABLE_IPS_PATH: &str = "/api/ipam/prefixes/{id}/available-ips/";
pub const PREFIX_AVAILABLE_PREFIXES_PATH: &str = "/api/ipam/prefixes/{id}/available-prefixes/";
pub const STATUS_PATH: &str = "/api/status/";
pub const AUTHENTICATION_CHECK_PATH: &str = "/api/authentication-check/";

// Sites, racks, inventory-items, IP addresses, prefixes, and VLANs each define
// their own `RESOURCE: ResourceSpec` const inside `crate::generated::resources`.
// Those generated modules are re-exported via `crate::generated::resources::all_resource_specs()`.
//
// Devices and interfaces have hand-written extensions on top of the generated
// args because their surface includes composite addressing, name resolution
// semantics, and the trace endpoint — none of which reduce to schema-shaped CRUD.

// ============================================================
// Shared lookup args (used by every resource group)
// ============================================================

#[derive(Debug, Args)]
pub struct GetArgs {
    pub id_or_lookup: String,

    #[arg(long)]
    pub lookup_field: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeleteArgs {
    pub id_or_lookup: String,

    #[arg(long)]
    pub lookup_field: Option<String>,

    #[arg(long)]
    pub confirm: bool,
}

#[derive(Debug, Args)]
pub struct BulkDeleteArgs {
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,

    #[arg(long)]
    pub confirm: bool,
}

// ============================================================
// Generic CRUD args (sites, racks, inventory-items, IPAM)
// ============================================================

#[derive(Debug, Args)]
pub struct GenericListArgs {
    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub tag: Option<String>,
}

#[derive(Debug, Args)]
pub struct GenericMutationArgs {
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

// ============================================================
// Device CRUD args (extends generated args)
// ============================================================

#[derive(Debug, Args)]
pub struct DeviceListArgs {
    /// Filter by site slug or numeric ID.
    #[arg(long)]
    pub site: Option<String>,

    /// Filter by device role slug. Repeat to OR multiple roles.
    #[arg(long)]
    pub role: Vec<String>,

    /// Filter by status (e.g. `active`, `planned`, `decommissioning`).
    #[arg(long)]
    pub status: Option<String>,

    #[arg(long)]
    pub tag: Option<String>,

    /// Filter by name (exact match).
    #[arg(long)]
    pub name: Option<String>,
}

// ============================================================
// Inventory-item list args (overrides the codegen's GenericListArgs
// via xtask/codegen LIST_OVERRIDES — see xtask/codegen/src/lib.rs)
// ============================================================

#[derive(Debug, Args)]
pub struct InventoryItemListArgs {
    /// Filter by device — slug, name, or numeric ID.
    #[arg(long)]
    pub device: Option<String>,

    /// Filter by inventory-item role slug. Repeat to OR multiple roles.
    #[arg(long)]
    pub role: Vec<String>,

    /// Filter by manufacturer slug.
    #[arg(long)]
    pub manufacturer: Option<String>,

    /// Filter by serial (exact match).
    #[arg(long)]
    pub serial: Option<String>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub tag: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeviceCreateArgs {
    #[command(flatten)]
    pub fields: crate::generated::resources::devices::DeviceCreateFields,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeviceUpdateArgs {
    /// Numeric `NetBox` ID or default-lookup-field value (`name`).
    pub id_or_lookup: String,

    #[arg(long)]
    pub lookup_field: Option<String>,

    #[command(flatten)]
    pub fields: crate::generated::resources::devices::DeviceUpdateFields,

    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum DeviceAction {
    List(DeviceListArgs),
    Get(GetArgs),
    Create(DeviceCreateArgs),
    Update(DeviceUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

// ============================================================
// Interface CRUD args (extends generated args; composite addressing, name resolution, trace)
// ============================================================

#[derive(Debug, Args)]
pub struct InterfaceListArgs {
    /// Filter by device name or numeric ID.
    #[arg(long)]
    pub device: Option<String>,

    /// Filter by interface type slug (e.g. `1000base-t`, `10gbase-x-sfpp`).
    #[arg(long = "type")]
    pub type_filter: Option<String>,

    /// Filter by enabled state.
    #[arg(long)]
    pub enabled: Option<bool>,

    /// Filter by switching mode (`access`, `tagged`, `tagged-all`).
    #[arg(long)]
    pub mode: Option<String>,

    /// Filter to management-only interfaces.
    #[arg(long = "mgmt-only")]
    pub mgmt_only: Option<bool>,

    #[arg(long)]
    pub tag: Option<String>,

    /// Filter by interface name (exact match).
    #[arg(long)]
    pub name: Option<String>,
}

#[derive(Debug, Args)]
pub struct InterfaceCreateArgs {
    /// Parent device (name or numeric ID).
    #[arg(long)]
    pub device: String,

    #[command(flatten)]
    pub fields: crate::generated::resources::interfaces::InterfaceCreateFields,

    /// Untagged VLAN (name or numeric ID); resolved against `/api/ipam/vlans/`.
    #[arg(long = "untagged-vlan")]
    pub untagged_vlan: Option<String>,

    /// Tagged VLANs as comma-separated names or numeric IDs.
    #[arg(long = "tagged-vlans")]
    pub tagged_vlans: Option<String>,

    /// LAG parent interface name on the same device.
    #[arg(long)]
    pub lag: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct InterfaceTargetArgs {
    /// Interface address: numeric ID or composite `device:interface` (e.g. `srv01:Ethernet1`).
    pub address: String,
}

#[derive(Debug, Args)]
pub struct InterfaceUpdateArgs {
    /// Interface address: numeric ID or composite `device:interface`.
    pub address: String,

    /// Reassign the interface to a different device (name or numeric ID).
    #[arg(long)]
    pub device: Option<String>,

    #[command(flatten)]
    pub fields: crate::generated::resources::interfaces::InterfaceUpdateFields,

    #[arg(long = "untagged-vlan")]
    pub untagged_vlan: Option<String>,

    #[arg(long = "tagged-vlans")]
    pub tagged_vlans: Option<String>,

    #[arg(long)]
    pub lag: Option<String>,

    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct InterfaceDeleteArgs {
    /// Interface address: numeric ID or composite `device:interface`.
    pub address: String,

    #[arg(long)]
    pub confirm: bool,
}

#[derive(Debug, Subcommand)]
pub enum InterfaceAction {
    List(InterfaceListArgs),
    Get(InterfaceTargetArgs),
    Create(InterfaceCreateArgs),
    Update(InterfaceUpdateArgs),
    Delete(InterfaceDeleteArgs),
    /// Trace the cable path attached to this interface.
    Trace(InterfaceTargetArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

// ============================================================
// Prefix CRUD args (extends generated args; available IP/prefix allocation)
// ============================================================

#[derive(Debug, Args)]
pub struct PrefixCreateArgs {
    #[command(flatten)]
    pub fields: crate::generated::resources::prefixes::PrefixCreateFields,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct PrefixUpdateArgs {
    /// Numeric `NetBox` ID or default-lookup-field value (`prefix`).
    pub id_or_lookup: String,

    #[arg(long)]
    pub lookup_field: Option<String>,

    #[command(flatten)]
    pub fields: crate::generated::resources::prefixes::PrefixUpdateFields,

    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct PrefixAvailabilityArgs {
    /// Prefix numeric ID or prefix value (e.g. `192.0.2.0/24`).
    pub id_or_lookup: String,

    #[arg(long)]
    pub lookup_field: Option<String>,

    /// Allocation payload. If omitted, nbx lists available children instead.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum PrefixAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(PrefixCreateArgs),
    Update(PrefixUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "available-ips")]
    AvailableIps(PrefixAvailabilityArgs),
    #[command(name = "available-prefixes")]
    AvailablePrefixes(PrefixAvailabilityArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

// ============================================================
// Top-level DCIM / IPAM groupings
// ============================================================

use crate::generated::resources::{
    aggregates::AggregateAction, cables::CableAction, device_roles::DeviceRoleAction,
    device_types::DeviceTypeAction, inventory_items::InventoryItemAction,
    ip_addresses::IpAddressAction, locations::LocationAction, manufacturers::ManufacturerAction,
    platforms::PlatformAction, racks::RackAction, regions::RegionAction, rirs::RirAction,
    roles::RoleAction, site_groups::SiteGroupAction, sites::SiteAction, tags::TagAction,
    tenant_groups::TenantGroupAction, tenants::TenantAction, vlan_groups::VlanGroupAction,
    vlans::VlanAction, vrfs::VrfAction,
};

#[derive(Debug, Subcommand)]
pub enum DcimCommand {
    Cables {
        #[command(subcommand)]
        action: CableAction,
    },
    #[command(name = "device-roles")]
    DeviceRoles {
        #[command(subcommand)]
        action: DeviceRoleAction,
    },
    #[command(name = "device-types")]
    DeviceTypes {
        #[command(subcommand)]
        action: DeviceTypeAction,
    },
    Devices {
        #[command(subcommand)]
        action: DeviceAction,
    },
    Interfaces {
        #[command(subcommand)]
        action: InterfaceAction,
    },
    Sites {
        #[command(subcommand)]
        action: SiteAction,
    },
    Racks {
        #[command(subcommand)]
        action: RackAction,
    },
    #[command(name = "inventory-items")]
    InventoryItems {
        #[command(subcommand)]
        action: InventoryItemAction,
    },
    Locations {
        #[command(subcommand)]
        action: LocationAction,
    },
    Manufacturers {
        #[command(subcommand)]
        action: ManufacturerAction,
    },
    Platforms {
        #[command(subcommand)]
        action: PlatformAction,
    },
    Regions {
        #[command(subcommand)]
        action: RegionAction,
    },
    #[command(name = "site-groups")]
    SiteGroups {
        #[command(subcommand)]
        action: SiteGroupAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum IpamCommand {
    Aggregates {
        #[command(subcommand)]
        action: AggregateAction,
    },
    #[command(name = "ip-addresses")]
    IpAddresses {
        #[command(subcommand)]
        action: IpAddressAction,
    },
    Prefixes {
        #[command(subcommand)]
        action: PrefixAction,
    },
    Rirs {
        #[command(subcommand)]
        action: RirAction,
    },
    Roles {
        #[command(subcommand)]
        action: RoleAction,
    },
    #[command(name = "vlan-groups")]
    VlanGroups {
        #[command(subcommand)]
        action: VlanGroupAction,
    },
    Vlans {
        #[command(subcommand)]
        action: VlanAction,
    },
    Vrfs {
        #[command(subcommand)]
        action: VrfAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum ExtrasCommand {
    Tags {
        #[command(subcommand)]
        action: TagAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum TenancyCommand {
    #[command(name = "tenant-groups")]
    TenantGroups {
        #[command(subcommand)]
        action: TenantGroupAction,
    },
    Tenants {
        #[command(subcommand)]
        action: TenantAction,
    },
}

// ============================================================
// Config dispatch
// ============================================================

pub async fn run_config_command(command: ConfigCommand) -> NbxResult<()> {
    match command {
        ConfigCommand::Init(arguments) => config_init(arguments),
        ConfigCommand::SetUrl(arguments) => config_set_url(arguments),
        ConfigCommand::SetToken(arguments) => config_set_token(arguments),
        ConfigCommand::Show(arguments) => config_show(arguments),
        ConfigCommand::UseContext(arguments) => config_use_context(arguments),
        ConfigCommand::CurrentContext => config_current_context(),
        ConfigCommand::GetContexts => config_get_contexts(),
    }
}

// ============================================================
// Raw API request dispatch
// ============================================================

pub async fn run_raw_command(arguments: RawArgs, global_options: &GlobalOptions) -> NbxResult<()> {
    let request_target = raw_request_target(&arguments.path, global_options)?;
    let method = arguments.method.http_method();

    if global_options.page_all {
        if method != HttpMethod::Get {
            return Err(NbxError::validation(
                "raw --page-all only supports GET requests",
                json!({ "method": arguments.method.as_str() }),
            ));
        }
        if arguments.has_request_body() {
            return Err(NbxError::validation(
                "raw --page-all does not support request bodies",
                json!({ "path": request_target.api_path.as_str() }),
            ));
        }
        if global_options.dry_run {
            let response = send_raw_or_dry_run(
                method,
                &request_target.api_path,
                request_target.query_parameters,
                None,
                global_options,
            )
            .await?;
            return emit_raw_response(
                project_value(response, global_options),
                selected_output_format(global_options),
            );
        }
        return stream_raw_pages(
            &request_target.api_path,
            request_target.query_parameters,
            global_options,
        )
        .await;
    }

    let response = send_raw_or_dry_run(
        method,
        &request_target.api_path,
        request_target.query_parameters,
        arguments.request_body()?,
        global_options,
    )
    .await?;
    emit_raw_response(
        project_value(response, global_options),
        selected_output_format(global_options),
    )
}

// ============================================================
// Shared helpers used by generated dispatchers
// ============================================================

pub(crate) async fn generic_list(
    resource: ResourceSpec,
    arguments: GenericListArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let mut query_parameters = base_query_parameters(global_options)?;
    push_optional_query(&mut query_parameters, "name", arguments.name);
    push_optional_query(&mut query_parameters, "tag", arguments.tag);
    list_with_query(resource, query_parameters, global_options).await
}

pub(crate) async fn generic_get(
    resource: ResourceSpec,
    arguments: GetArgs,
    validator: Option<ResponseValidator>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let value = resolve_resource_value(
        resource,
        &arguments.id_or_lookup,
        arguments.lookup_field.as_deref(),
        global_options,
    )
    .await?;
    maybe_warn_response_drift(&value, validator, resource.name);
    emit_single(
        &project_value(value, global_options),
        selected_output_format(global_options),
    )
}

pub(crate) fn generic_mutation_body(arguments: GenericMutationArgs) -> NbxResult<Value> {
    let body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    if !body.is_object() && !body.is_array() {
        return Err(NbxError::validation(
            "mutation payload must be a JSON object or array",
            json!({}),
        ));
    }
    Ok(body)
}

// ============================================================
// Device dispatch (uses generated args; the extension adds typed list filters)
// ============================================================

pub async fn run_device_action(
    action: DeviceAction,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    match action {
        DeviceAction::List(arguments) => device_list(arguments, global_options).await,
        DeviceAction::Get(arguments) => {
            generic_get(
                crate::generated::resources::devices::RESOURCE,
                arguments,
                Some(crate::generated::resources::devices::validate_response),
                global_options,
            )
            .await
        }
        DeviceAction::Create(arguments) => {
            let body = build_device_create_body(arguments, global_options).await?;
            mutate_collection_with_body(
                crate::generated::resources::devices::RESOURCE,
                HttpMethod::Post,
                body,
                Some(crate::generated::resources::devices::validate_response),
                global_options,
            )
            .await
        }
        DeviceAction::Update(arguments) => {
            let id_or_lookup = arguments.id_or_lookup.clone();
            let lookup_field = arguments.lookup_field.clone();
            let body = build_device_update_body(arguments, global_options).await?;
            update_resource_with_body(
                crate::generated::resources::devices::RESOURCE,
                &id_or_lookup,
                lookup_field.as_deref(),
                body,
                Some(crate::generated::resources::devices::validate_response),
                global_options,
            )
            .await
        }
        DeviceAction::Delete(arguments) => {
            delete_resource(
                crate::generated::resources::devices::RESOURCE,
                arguments,
                global_options,
            )
            .await
        }
        DeviceAction::BulkUpdate(arguments) => {
            mutate_collection_with_body(
                crate::generated::resources::devices::RESOURCE,
                HttpMethod::Patch,
                generic_mutation_body(arguments)?,
                Some(crate::generated::resources::devices::validate_bulk_response),
                global_options,
            )
            .await
        }
        DeviceAction::BulkDelete(arguments) => {
            bulk_delete_resource(
                crate::generated::resources::devices::RESOURCE,
                arguments,
                global_options,
            )
            .await
        }
    }
}

async fn device_list(arguments: DeviceListArgs, global_options: &GlobalOptions) -> NbxResult<()> {
    let mut query_parameters = base_query_parameters(global_options)?;
    push_optional_query(&mut query_parameters, "site", arguments.site);
    for role in arguments.role {
        query_parameters.push(("role".to_owned(), role));
    }
    push_optional_query(&mut query_parameters, "status", arguments.status);
    push_optional_query(&mut query_parameters, "tag", arguments.tag);
    push_optional_query(&mut query_parameters, "name", arguments.name);
    list_with_query(
        crate::generated::resources::devices::RESOURCE,
        query_parameters,
        global_options,
    )
    .await
}

pub async fn inventory_items_list(
    arguments: InventoryItemListArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let mut query_parameters = base_query_parameters(global_options)?;
    if let Some(device) = arguments.device {
        let device_id = resolve_device_id_for_lookup(&device, global_options).await?;
        query_parameters.push(("device_id".to_owned(), device_id.to_string()));
    }
    for role in arguments.role {
        query_parameters.push(("role".to_owned(), role));
    }
    push_optional_query(
        &mut query_parameters,
        "manufacturer",
        arguments.manufacturer,
    );
    push_optional_query(&mut query_parameters, "serial", arguments.serial);
    push_optional_query(&mut query_parameters, "name", arguments.name);
    push_optional_query(&mut query_parameters, "tag", arguments.tag);
    list_with_query(
        crate::generated::resources::inventory_items::RESOURCE,
        query_parameters,
        global_options,
    )
    .await
}

async fn build_device_create_body(
    arguments: DeviceCreateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    merge_object_into(
        &mut body,
        crate::generated::resources::devices::create_body(arguments.fields, global_options).await?,
    )?;
    Ok(body)
}

async fn build_device_update_body(
    arguments: DeviceUpdateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    merge_object_into(
        &mut body,
        crate::generated::resources::devices::update_body(arguments.fields, global_options).await?,
    )?;
    Ok(body)
}

fn merge_object_into(target: &mut Value, additions: Value) -> NbxResult<()> {
    let target_object = target
        .as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    if let Value::Object(map) = additions {
        for (key, value) in map {
            target_object.insert(key, value);
        }
    }
    Ok(())
}

// ============================================================
// Interface dispatch
// ============================================================

pub async fn run_interface_action(
    action: InterfaceAction,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    match action {
        InterfaceAction::List(arguments) => interface_list(arguments, global_options).await,
        InterfaceAction::Get(arguments) => interface_get(arguments, global_options).await,
        InterfaceAction::Create(arguments) => interface_create(arguments, global_options).await,
        InterfaceAction::Update(arguments) => interface_update(arguments, global_options).await,
        InterfaceAction::Delete(arguments) => interface_delete(arguments, global_options).await,
        InterfaceAction::Trace(arguments) => interface_trace(arguments, global_options).await,
        InterfaceAction::BulkUpdate(arguments) => {
            mutate_collection_with_body(
                INTERFACES,
                HttpMethod::Patch,
                generic_mutation_body(arguments)?,
                Some(crate::generated::resources::interfaces::validate_bulk_response),
                global_options,
            )
            .await
        }
        InterfaceAction::BulkDelete(arguments) => {
            bulk_delete_resource(INTERFACES, arguments, global_options).await
        }
    }
}

pub async fn run_prefix_action(
    action: PrefixAction,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    match action {
        PrefixAction::List(arguments) => {
            generic_list(
                crate::generated::resources::prefixes::RESOURCE,
                arguments,
                global_options,
            )
            .await
        }
        PrefixAction::Get(arguments) => {
            generic_get(
                crate::generated::resources::prefixes::RESOURCE,
                arguments,
                Some(crate::generated::resources::prefixes::validate_response),
                global_options,
            )
            .await
        }
        PrefixAction::Create(arguments) => {
            let body = build_prefix_create_body(arguments, global_options).await?;
            mutate_collection_with_body(
                crate::generated::resources::prefixes::RESOURCE,
                HttpMethod::Post,
                body,
                Some(crate::generated::resources::prefixes::validate_response),
                global_options,
            )
            .await
        }
        PrefixAction::Update(arguments) => {
            let id_or_lookup = arguments.id_or_lookup.clone();
            let lookup_field = arguments.lookup_field.clone();
            let body = build_prefix_update_body(arguments, global_options).await?;
            update_resource_with_body(
                crate::generated::resources::prefixes::RESOURCE,
                &id_or_lookup,
                lookup_field.as_deref(),
                body,
                Some(crate::generated::resources::prefixes::validate_response),
                global_options,
            )
            .await
        }
        PrefixAction::Delete(arguments) => {
            delete_resource(
                crate::generated::resources::prefixes::RESOURCE,
                arguments,
                global_options,
            )
            .await
        }
        PrefixAction::AvailableIps(arguments) => {
            prefix_availability(
                PrefixAvailabilityEndpoint::AvailableIps,
                arguments,
                global_options,
            )
            .await
        }
        PrefixAction::AvailablePrefixes(arguments) => {
            prefix_availability(
                PrefixAvailabilityEndpoint::AvailablePrefixes,
                arguments,
                global_options,
            )
            .await
        }
        PrefixAction::BulkUpdate(arguments) => {
            mutate_collection_with_body(
                crate::generated::resources::prefixes::RESOURCE,
                HttpMethod::Patch,
                generic_mutation_body(arguments)?,
                Some(crate::generated::resources::prefixes::validate_bulk_response),
                global_options,
            )
            .await
        }
        PrefixAction::BulkDelete(arguments) => {
            bulk_delete_resource(
                crate::generated::resources::prefixes::RESOURCE,
                arguments,
                global_options,
            )
            .await
        }
    }
}

pub async fn run_status_command(global_options: &GlobalOptions) -> NbxResult<()> {
    let endpoint = endpoint_for(STATUS_PATH, HttpMethod::Get)?;
    let response = send_or_dry_run_with_optional_token(
        endpoint,
        STATUS_PATH,
        Vec::new(),
        None,
        global_options,
    )
    .await?;
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

pub async fn run_authentication_check_command(global_options: &GlobalOptions) -> NbxResult<()> {
    let endpoint = endpoint_for(AUTHENTICATION_CHECK_PATH, HttpMethod::Get)?;
    let response = send_or_dry_run(
        endpoint,
        AUTHENTICATION_CHECK_PATH,
        Vec::new(),
        None,
        global_options,
    )
    .await?;
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

async fn interface_list(
    arguments: InterfaceListArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let mut query_parameters = base_query_parameters(global_options)?;
    if let Some(device) = arguments.device {
        if let Ok(device_id) = device.parse::<u64>() {
            query_parameters.push(("device_id".to_owned(), device_id.to_string()));
        } else {
            query_parameters.push(("device".to_owned(), device));
        }
    }
    push_optional_query(&mut query_parameters, "type", arguments.type_filter);
    push_optional_query(&mut query_parameters, "mode", arguments.mode);
    push_optional_query(&mut query_parameters, "tag", arguments.tag);
    push_optional_query(&mut query_parameters, "name", arguments.name);
    if let Some(enabled) = arguments.enabled {
        query_parameters.push(("enabled".to_owned(), enabled.to_string()));
    }
    if let Some(mgmt_only) = arguments.mgmt_only {
        query_parameters.push(("mgmt_only".to_owned(), mgmt_only.to_string()));
    }
    list_with_query(INTERFACES, query_parameters, global_options).await
}

async fn interface_get(
    arguments: InterfaceTargetArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let value = resolve_interface_value(&arguments.address, global_options).await?;
    maybe_warn_response_drift(
        &value,
        Some(crate::generated::resources::interfaces::validate_response),
        INTERFACES.name,
    );
    emit_single(
        &project_value(value, global_options),
        selected_output_format(global_options),
    )
}

async fn interface_create(
    arguments: InterfaceCreateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let body = build_interface_create_body(arguments, global_options).await?;
    mutate_collection_with_body(
        INTERFACES,
        HttpMethod::Post,
        body,
        Some(crate::generated::resources::interfaces::validate_response),
        global_options,
    )
    .await
}

async fn interface_update(
    arguments: InterfaceUpdateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let id = resolve_interface_id(&arguments.address, global_options).await?;
    let body = build_interface_update_body(arguments, global_options).await?;
    let path = INTERFACES.detail_path.replace("{id}", &id);
    let endpoint = endpoint_for(INTERFACES.detail_path, HttpMethod::Patch)?;
    let response = send_or_dry_run(endpoint, &path, Vec::new(), Some(body), global_options).await?;
    maybe_warn_response_drift(
        &response,
        Some(crate::generated::resources::interfaces::validate_response),
        INTERFACES.name,
    );
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

async fn interface_delete(
    arguments: InterfaceDeleteArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    if !arguments.confirm && !global_options.dry_run {
        return Err(NbxError::validation(
            "delete requires --confirm",
            json!({ "resource": INTERFACES.name }),
        ));
    }
    let id = resolve_interface_id(&arguments.address, global_options).await?;
    let path = INTERFACES.detail_path.replace("{id}", &id);
    let endpoint = endpoint_for(INTERFACES.detail_path, HttpMethod::Delete)?;
    let response = send_or_dry_run(endpoint, &path, Vec::new(), None, global_options).await?;
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

async fn interface_trace(
    arguments: InterfaceTargetArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let id = resolve_interface_id(&arguments.address, global_options).await?;
    let path = INTERFACES_TRACE_PATH.replace("{id}", &id);
    let endpoint = endpoint_for(INTERFACES_TRACE_PATH, HttpMethod::Get)?;
    let response = send_or_dry_run(endpoint, &path, Vec::new(), None, global_options).await?;
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

// ----- Interface composite address -----

#[derive(Debug, Eq, PartialEq)]
enum InterfaceAddress {
    Numeric(u64),
    Composite {
        device_lookup: String,
        interface_name: String,
    },
}

fn parse_interface_address(input: &str) -> NbxResult<InterfaceAddress> {
    if let Ok(numeric_id) = input.parse::<u64>() {
        return Ok(InterfaceAddress::Numeric(numeric_id));
    }
    if let Some((device_lookup, interface_name)) = input.split_once(':') {
        if device_lookup.is_empty() || interface_name.is_empty() {
            return Err(invalid_interface_address(input));
        }
        return Ok(InterfaceAddress::Composite {
            device_lookup: device_lookup.to_owned(),
            interface_name: interface_name.to_owned(),
        });
    }
    Err(invalid_interface_address(input))
}

fn invalid_interface_address(input: &str) -> NbxError {
    NbxError::validation(
        "interface address must be a numeric ID or composite `device:interface` (e.g. `srv01:Ethernet1`)",
        json!({ "address": input }),
    )
}

async fn resolve_interface_id(address: &str, global_options: &GlobalOptions) -> NbxResult<String> {
    if global_options.dry_run {
        return Ok(address.to_owned());
    }
    match parse_interface_address(address)? {
        InterfaceAddress::Numeric(id) => Ok(id.to_string()),
        InterfaceAddress::Composite {
            device_lookup,
            interface_name,
        } => {
            let device_id = resolve_device_id_for_lookup(&device_lookup, global_options).await?;
            let interface =
                resolve_interface_by_device(device_id, &interface_name, global_options).await?;
            interface
                .get("id")
                .and_then(Value::as_u64)
                .map(|id| id.to_string())
                .ok_or_else(|| {
                    NbxError::general("interface lookup result did not contain numeric id")
                })
        }
    }
}

async fn resolve_interface_value(
    address: &str,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    match parse_interface_address(address)? {
        InterfaceAddress::Numeric(id) => {
            let path = INTERFACES.detail_path.replace("{id}", &id.to_string());
            let endpoint = endpoint_for(INTERFACES.detail_path, HttpMethod::Get)?;
            send_or_dry_run(endpoint, &path, Vec::new(), None, global_options).await
        }
        InterfaceAddress::Composite {
            device_lookup,
            interface_name,
        } => {
            if global_options.dry_run {
                return Ok(json!({
                    "dryRun": true,
                    "lookup": "interface",
                    "device": device_lookup,
                    "name": interface_name,
                }));
            }
            let device_id = resolve_device_id_for_lookup(&device_lookup, global_options).await?;
            resolve_interface_by_device(device_id, &interface_name, global_options).await
        }
    }
}

async fn resolve_device_id_for_lookup(
    device_lookup: &str,
    global_options: &GlobalOptions,
) -> NbxResult<u64> {
    if let Ok(numeric_id) = device_lookup.parse::<u64>() {
        return Ok(numeric_id);
    }
    let value = resolve_resource_value(DEVICES, device_lookup, None, global_options).await?;
    value
        .get("id")
        .and_then(Value::as_u64)
        .ok_or_else(|| NbxError::general("device lookup result did not contain numeric id"))
}

async fn resolve_interface_by_device(
    device_id: u64,
    interface_name: &str,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let query_parameters = vec![
        ("device_id".to_owned(), device_id.to_string()),
        ("name".to_owned(), interface_name.to_owned()),
        ("limit".to_owned(), "2".to_owned()),
    ];
    let endpoint = endpoint_for(INTERFACES.api_path, HttpMethod::Get)?;
    let page = send_or_dry_run(
        endpoint,
        INTERFACES.api_path,
        query_parameters,
        None,
        global_options,
    )
    .await?;
    let results = page
        .get("results")
        .and_then(Value::as_array)
        .ok_or_else(|| NbxError::general("interface lookup response did not contain results"))?;
    match results.as_slice() {
        [single] => Ok(single.clone()),
        [] => Err(NbxError::not_found(
            format!("interface not found: device_id={device_id} name={interface_name}"),
            json!({
                "resource": "dcim.interfaces",
                "device_id": device_id,
                "name": interface_name,
            }),
        )),
        _ => Err(NbxError::validation(
            format!(
                "interface lookup matched multiple records: device_id={device_id} name={interface_name}"
            ),
            json!({
                "resource": "dcim.interfaces",
                "device_id": device_id,
                "name": interface_name,
            }),
        )),
    }
}

// ----- Interface mutation body assembly -----

async fn build_interface_create_body(
    arguments: InterfaceCreateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    merge_object_into(
        &mut body,
        crate::generated::resources::interfaces::create_body(arguments.fields, global_options)
            .await?,
    )?;
    let object = body
        .as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;

    // device is required at create time; resolve and insert.
    let resolved_device_id =
        resolve_required_device_reference(&arguments.device, object, global_options).await?;
    inject_extended_interface_refs(
        object,
        arguments.untagged_vlan,
        arguments.tagged_vlans,
        arguments.lag,
        resolved_device_id,
        global_options,
    )
    .await?;

    Ok(body)
}

async fn build_prefix_create_body(
    arguments: PrefixCreateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    merge_object_into(
        &mut body,
        crate::generated::resources::prefixes::create_body(arguments.fields, global_options)
            .await?,
    )?;
    Ok(body)
}

async fn build_prefix_update_body(
    arguments: PrefixUpdateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    merge_object_into(
        &mut body,
        crate::generated::resources::prefixes::update_body(arguments.fields, global_options)
            .await?,
    )?;
    Ok(body)
}

async fn prefix_availability(
    availability_endpoint: PrefixAvailabilityEndpoint,
    arguments: PrefixAvailabilityArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let prefix_id = resolve_resource_id(
        crate::generated::resources::prefixes::RESOURCE,
        &arguments.id_or_lookup,
        arguments.lookup_field.as_deref(),
        global_options,
    )
    .await?;
    let path_template = availability_endpoint.path_template();
    let path = path_template.replace("{id}", prefix_id.path_segment());
    let body = optional_json_body(arguments.data, arguments.data_file)?;
    let availability_operation = PrefixAvailabilityOperation::from_body(body.as_ref());
    let endpoint = endpoint_for(path_template, availability_operation.http_method())?;
    let response = send_or_dry_run_with_extra_fields(
        endpoint,
        &path,
        Vec::new(),
        body,
        prefix_id.dry_run_id_resolution(),
        global_options,
    )
    .await?;
    let projected_response = project_value(response, global_options);
    match availability_operation {
        PrefixAvailabilityOperation::List => {
            emit_page(&projected_response, selected_output_format(global_options))
        }
        PrefixAvailabilityOperation::Allocate => {
            emit_single(&projected_response, selected_output_format(global_options))
        }
    }
}

async fn build_interface_update_body(
    arguments: InterfaceUpdateArgs,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut body =
        optional_json_body(arguments.data, arguments.data_file)?.unwrap_or_else(|| json!({}));
    merge_object_into(
        &mut body,
        crate::generated::resources::interfaces::update_body(arguments.fields, global_options)
            .await?,
    )?;
    let object = body
        .as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;

    let resolved_device_id =
        resolve_optional_device_reference(arguments.device, object, global_options).await?;
    inject_extended_interface_refs(
        object,
        arguments.untagged_vlan,
        arguments.tagged_vlans,
        arguments.lag,
        resolved_device_id,
        global_options,
    )
    .await?;

    Ok(body)
}

async fn inject_extended_interface_refs(
    object: &mut Map<String, Value>,
    untagged_vlan: Option<String>,
    tagged_vlans: Option<String>,
    lag: Option<String>,
    resolved_device_id: Option<u64>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    if let Some(untagged) = untagged_vlan {
        let resolved = resolve_vlan_reference(&untagged, global_options).await?;
        object.insert("untagged_vlan".to_owned(), resolved);
    }
    if let Some(tagged_csv) = tagged_vlans {
        let resolved = resolve_tagged_vlans_reference(&tagged_csv, global_options).await?;
        object.insert("tagged_vlans".to_owned(), resolved);
    }
    if let Some(lag_lookup) = lag {
        let device_id = resolved_device_id.or_else(|| object.get("device").and_then(Value::as_u64));
        let resolved = resolve_lag_reference(&lag_lookup, device_id, global_options).await?;
        object.insert("lag".to_owned(), resolved);
    }
    Ok(())
}

async fn resolve_required_device_reference(
    device_lookup: &str,
    body_object: &mut Map<String, Value>,
    global_options: &GlobalOptions,
) -> NbxResult<Option<u64>> {
    if global_options.dry_run {
        body_object.insert("device".to_owned(), Value::String(device_lookup.to_owned()));
        return Ok(None);
    }
    let device_id = resolve_device_id_for_lookup(device_lookup, global_options).await?;
    body_object.insert("device".to_owned(), json!(device_id));
    Ok(Some(device_id))
}

async fn resolve_optional_device_reference(
    device_lookup: Option<String>,
    body_object: &mut Map<String, Value>,
    global_options: &GlobalOptions,
) -> NbxResult<Option<u64>> {
    let Some(lookup) = device_lookup else {
        return Ok(None);
    };
    if global_options.dry_run {
        body_object.insert("device".to_owned(), Value::String(lookup));
        return Ok(None);
    }
    let device_id = resolve_device_id_for_lookup(&lookup, global_options).await?;
    body_object.insert("device".to_owned(), json!(device_id));
    Ok(Some(device_id))
}

async fn resolve_vlan_reference(
    vlan_lookup: &str,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if let Ok(numeric_id) = vlan_lookup.parse::<u64>() {
        return Ok(json!(numeric_id));
    }
    if global_options.dry_run {
        return Ok(Value::String(vlan_lookup.to_owned()));
    }
    let id = resolve_unique_resource_id(
        crate::generated::resources::vlans::RESOURCE,
        "name",
        vlan_lookup,
        global_options,
    )
    .await?;
    Ok(json!(id))
}

async fn resolve_tagged_vlans_reference(
    tagged_vlans_csv: &str,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut resolved_ids: Vec<Value> = Vec::new();
    for entry in tagged_vlans_csv
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        let resolved = resolve_vlan_reference(entry, global_options).await?;
        resolved_ids.push(resolved);
    }
    Ok(Value::Array(resolved_ids))
}

async fn resolve_lag_reference(
    lag_lookup: &str,
    device_id: Option<u64>,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if let Ok(numeric_id) = lag_lookup.parse::<u64>() {
        return Ok(json!(numeric_id));
    }
    if global_options.dry_run {
        return Ok(Value::String(lag_lookup.to_owned()));
    }
    let device_id = device_id.ok_or_else(|| {
        NbxError::validation(
            "resolving --lag by name requires the parent device to be set first via --device or in --data",
            json!({ "lag": lag_lookup }),
        )
    })?;
    let lag_interface = resolve_interface_by_device(device_id, lag_lookup, global_options).await?;
    let id = lag_interface
        .get("id")
        .and_then(Value::as_u64)
        .ok_or_else(|| NbxError::general("LAG lookup result did not contain numeric id"))?;
    Ok(json!(id))
}

async fn resolve_unique_resource_id(
    resource: ResourceSpec,
    lookup_field: &str,
    lookup_value: &str,
    global_options: &GlobalOptions,
) -> NbxResult<u64> {
    let query_parameters = vec![
        (lookup_field.to_owned(), lookup_value.to_owned()),
        ("limit".to_owned(), "2".to_owned()),
    ];
    let endpoint = endpoint_for(resource.api_path, HttpMethod::Get)?;
    let page = send_or_dry_run(
        endpoint,
        resource.api_path,
        query_parameters,
        None,
        global_options,
    )
    .await?;
    let results = page
        .get("results")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            NbxError::general(format!(
                "{} lookup response did not contain results",
                resource.name
            ))
        })?;
    match results.as_slice() {
        [single] => single.get("id").and_then(Value::as_u64).ok_or_else(|| {
            NbxError::general(format!(
                "{} lookup result did not contain numeric id",
                resource.name
            ))
        }),
        [] => Err(NbxError::not_found(
            format!("{} not found: {lookup_value}", resource.name),
            json!({
                "resource": format!("{}.{}", resource.app, resource.name),
                "lookup": lookup_value,
                "lookupField": lookup_field,
            }),
        )),
        _ => Err(NbxError::validation(
            format!(
                "{} lookup matched multiple records: {lookup_value}; pass numeric ID to disambiguate",
                resource.name
            ),
            json!({
                "resource": format!("{}.{}", resource.app, resource.name),
                "lookup": lookup_value,
                "lookupField": lookup_field,
            }),
        )),
    }
}

// ============================================================
// Shared list / mutation / delete helpers
// ============================================================

pub(crate) async fn list_with_query(
    resource: ResourceSpec,
    query_parameters: Vec<(String, String)>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let endpoint = endpoint_for(resource.api_path, HttpMethod::Get)?;
    if global_options.page_all {
        stream_all_pages(resource, endpoint, query_parameters, global_options).await
    } else {
        let value = send_or_dry_run(
            endpoint,
            resource.api_path,
            query_parameters,
            None,
            global_options,
        )
        .await?;
        let projected_value = project_value(value, global_options);
        emit_page(&projected_value, selected_output_format(global_options))
    }
}

pub(crate) async fn mutate_collection_with_body(
    resource: ResourceSpec,
    method: HttpMethod,
    body: Value,
    validator: Option<ResponseValidator>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let endpoint = endpoint_for(resource.api_path, method)?;
    let response = send_or_dry_run(
        endpoint,
        resource.api_path,
        Vec::new(),
        Some(body),
        global_options,
    )
    .await?;
    maybe_warn_response_drift(&response, validator, resource.name);
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

pub(crate) async fn update_resource_with_body(
    resource: ResourceSpec,
    id_or_lookup: &str,
    lookup_field: Option<&str>,
    body: Value,
    validator: Option<ResponseValidator>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let id = resolve_resource_id(resource, id_or_lookup, lookup_field, global_options).await?;
    let path = resource.detail_path.replace("{id}", id.path_segment());
    let endpoint = endpoint_for(resource.detail_path, HttpMethod::Patch)?;
    let response = send_or_dry_run_with_extra_fields(
        endpoint,
        &path,
        Vec::new(),
        Some(body),
        id.dry_run_id_resolution(),
        global_options,
    )
    .await?;
    maybe_warn_response_drift(&response, validator, resource.name);
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

pub(crate) async fn delete_resource(
    resource: ResourceSpec,
    arguments: DeleteArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    if !arguments.confirm && !global_options.dry_run {
        return Err(NbxError::validation(
            "delete requires --confirm",
            json!({ "resource": resource.name }),
        ));
    }

    let id = resolve_resource_id(
        resource,
        &arguments.id_or_lookup,
        arguments.lookup_field.as_deref(),
        global_options,
    )
    .await?;
    let path = resource.detail_path.replace("{id}", id.path_segment());
    let endpoint = endpoint_for(resource.detail_path, HttpMethod::Delete)?;
    let response = send_or_dry_run_with_extra_fields(
        endpoint,
        &path,
        Vec::new(),
        None,
        id.dry_run_id_resolution(),
        global_options,
    )
    .await?;
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

pub(crate) async fn bulk_delete_resource(
    resource: ResourceSpec,
    arguments: BulkDeleteArgs,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    if !arguments.confirm && !global_options.dry_run {
        return Err(NbxError::validation(
            "bulk-delete requires --confirm",
            json!({ "resource": resource.name }),
        ));
    }

    let endpoint = endpoint_for(resource.api_path, HttpMethod::Delete)?;
    let body = optional_json_body(arguments.data, arguments.data_file)?;
    let response = send_or_dry_run(
        endpoint,
        resource.api_path,
        Vec::new(),
        body,
        global_options,
    )
    .await?;
    emit_single(
        &project_value(response, global_options),
        selected_output_format(global_options),
    )
}

async fn stream_all_pages(
    resource: ResourceSpec,
    endpoint: &EndpointDescriptor,
    mut query_parameters: Vec<(String, String)>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let client = netbox_client_for_options(global_options)?;

    loop {
        let page = match client
            .request(endpoint.method, resource.api_path, &query_parameters, None)
            .await
        {
            Ok(page) => page,
            Err(error) => {
                let stream_error = NbxError::stream(
                    "failed while streaming paginated results",
                    json!({
                        "resource": resource.name,
                        "source": error.message,
                        "detail": error.detail,
                    }),
                );
                print_json_line(&crate::envelope::error_envelope(&stream_error))?;
                return Err(stream_error);
            }
        };

        let projected_page = project_value(page.clone(), global_options);
        emit_page(&projected_page, OutputFormat::Ndjson)?;

        let Some(next_url) = page.get("next").and_then(Value::as_str) else {
            break;
        };
        query_parameters = next_query_parameters(next_url, &query_parameters);
    }

    Ok(())
}

async fn stream_raw_pages(
    api_path: &RawApiPath,
    mut query_parameters: Vec<(String, String)>,
    global_options: &GlobalOptions,
) -> NbxResult<()> {
    let client = netbox_client_for_options(global_options)?;

    loop {
        let page = match client
            .request(HttpMethod::Get, api_path.as_str(), &query_parameters, None)
            .await
        {
            Ok(page) => page,
            Err(error) => {
                let stream_error = NbxError::stream(
                    "failed while streaming raw paginated results",
                    json!({
                        "path": api_path.as_str(),
                        "source": error.message,
                        "detail": error.detail,
                    }),
                );
                print_json_line(&crate::envelope::error_envelope(&stream_error))?;
                return Err(stream_error);
            }
        };

        let projected_page = project_value(page.clone(), global_options);
        emit_page(&projected_page, OutputFormat::Ndjson)?;

        let Some(next_url) = page.get("next").and_then(Value::as_str) else {
            break;
        };
        query_parameters = next_query_parameters(next_url, &query_parameters);
    }

    Ok(())
}

fn netbox_client_for_options(global_options: &GlobalOptions) -> NbxResult<NetBoxClient> {
    let connection = resolve_connection(&ConfigOverrides {
        url: global_options.url.clone(),
        token: global_options.token.clone(),
        context: global_options.context.clone(),
    })?;
    let cache_key = ClientCacheKey {
        url: connection.url,
        token: connection.token,
        timeout_seconds: global_options.timeout,
    };
    let client_cache = CLIENT_CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut cached_clients = client_cache
        .lock()
        .map_err(|_| NbxError::general("NetBox client cache lock was poisoned"))?;

    if let Some(client) = cached_clients.get(&cache_key) {
        return Ok(client.clone());
    }

    let client = NetBoxClient::new(
        cache_key.url.clone(),
        cache_key.token.clone(),
        cache_key.timeout_seconds,
    )?;
    cached_clients.insert(cache_key, client.clone());
    Ok(client)
}

async fn resolve_resource_value(
    resource: ResourceSpec,
    id_or_lookup: &str,
    lookup_field: Option<&str>,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if id_or_lookup.parse::<u64>().is_ok() {
        let path = resource.detail_path.replace("{id}", id_or_lookup);
        let endpoint = endpoint_for(resource.detail_path, HttpMethod::Get)?;
        return send_or_dry_run(endpoint, &path, Vec::new(), None, global_options).await;
    }

    let lookup_field = lookup_field.unwrap_or(resource.default_lookup_field);
    let query_parameters = lookup_query_parameters(lookup_field, id_or_lookup);
    let endpoint = endpoint_for(resource.api_path, HttpMethod::Get)?;
    let page = send_or_dry_run(
        endpoint,
        resource.api_path,
        query_parameters,
        None,
        global_options,
    )
    .await?;
    if global_options.dry_run {
        return Ok(page);
    }

    let results = page
        .get("results")
        .and_then(Value::as_array)
        .ok_or_else(|| NbxError::general("NetBox lookup response did not contain results"))?;

    match results.as_slice() {
        [single] => Ok(single.clone()),
        [] => Err(NbxError::not_found(
            format!("{} not found: {id_or_lookup}", resource.name),
            json!({ "resource": format!("{}.{}", resource.app, resource.name), "lookup": id_or_lookup }),
        )),
        _ => Err(NbxError::validation(
            format!(
                "{} lookup matched multiple records: {id_or_lookup}",
                resource.name
            ),
            json!({ "resource": format!("{}.{}", resource.app, resource.name), "lookup": id_or_lookup, "lookupField": lookup_field }),
        )),
    }
}

async fn resolve_resource_id(
    resource: ResourceSpec,
    id_or_lookup: &str,
    lookup_field: Option<&str>,
    global_options: &GlobalOptions,
) -> NbxResult<ResolvedResourceId> {
    if id_or_lookup.parse::<u64>().is_ok() {
        return Ok(ResolvedResourceId::known(id_or_lookup));
    }

    let lookup_field = lookup_field.unwrap_or(resource.default_lookup_field);
    if global_options.dry_run {
        return Ok(ResolvedResourceId::dry_run_lookup(
            resource,
            lookup_field,
            id_or_lookup,
        ));
    }

    let value =
        resolve_resource_value(resource, id_or_lookup, Some(lookup_field), global_options).await?;
    value
        .get("id")
        .and_then(Value::as_u64)
        .map(|id| ResolvedResourceId::known(id.to_string()))
        .ok_or_else(|| NbxError::general("lookup result did not contain numeric id"))
}

async fn send_or_dry_run(
    endpoint: &EndpointDescriptor,
    path: &str,
    query_parameters: Vec<(String, String)>,
    body: Option<Value>,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if global_options.dry_run {
        return Ok(json!({
            "dryRun": true,
            "method": endpoint.method.as_str(),
            "operationId": endpoint.operation_id,
            "path": path,
            "query": query_parameters,
            "body": body,
        }));
    }

    let client = netbox_client_for_options(global_options)?;
    client
        .request(endpoint.method, path, &query_parameters, body)
        .await
}

async fn send_or_dry_run_with_extra_fields(
    endpoint: &EndpointDescriptor,
    path: &str,
    query_parameters: Vec<(String, String)>,
    body: Option<Value>,
    dry_run_id_resolution: Option<Value>,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    let mut response =
        send_or_dry_run(endpoint, path, query_parameters, body, global_options).await?;
    if global_options.dry_run
        && let Some(id_resolution) = dry_run_id_resolution
        && let Some(response_object) = response.as_object_mut()
    {
        response_object.insert("idResolution".to_owned(), id_resolution);
    }
    Ok(response)
}

async fn send_raw_or_dry_run(
    method: HttpMethod,
    path: &RawApiPath,
    query_parameters: Vec<(String, String)>,
    body: Option<Value>,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if global_options.dry_run {
        return Ok(json!({
            "dryRun": true,
            "method": method.as_str(),
            "path": path.as_str(),
            "query": query_parameters,
            "body": body,
        }));
    }

    let client = netbox_client_for_options(global_options)?;
    client
        .request(method, path.as_str(), &query_parameters, body)
        .await
}

async fn send_or_dry_run_with_optional_token(
    endpoint: &EndpointDescriptor,
    path: &str,
    query_parameters: Vec<(String, String)>,
    body: Option<Value>,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if global_options.dry_run {
        return Ok(json!({
            "dryRun": true,
            "method": endpoint.method.as_str(),
            "operationId": endpoint.operation_id,
            "path": path,
            "query": query_parameters,
            "body": body,
        }));
    }

    let connection = resolve_optional_connection(&ConfigOverrides {
        url: global_options.url.clone(),
        token: global_options.token.clone(),
        context: global_options.context.clone(),
    })?;
    let client = NetBoxClient::new_with_authentication(
        connection.url,
        ClientAuthentication::from_optional_token(connection.token),
        global_options.timeout,
    )?;
    client
        .request_without_version_check(endpoint.method, path, &query_parameters, body)
        .await
}

pub(crate) fn optional_json_body(
    data: Option<String>,
    data_file: Option<String>,
) -> NbxResult<Option<Value>> {
    let json_text = if let Some(data) = data {
        data
    } else if let Some(path) = data_file {
        fs::read_to_string(&path).map_err(|error| {
            NbxError::general(format!("failed to read data file {path}: {error}"))
        })?
    } else {
        return Ok(None);
    };

    serde_json::from_str(&json_text).map(Some).map_err(|error| {
        NbxError::validation(format!("failed to parse JSON payload: {error}"), json!({}))
    })
}

pub(crate) fn insert_optional_string_field(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<String>,
) {
    if let Some(value) = value {
        object.insert(key.to_owned(), Value::String(value));
    }
}

pub(crate) fn insert_optional_bool_field(
    object: &mut Map<String, Value>,
    key: &str,
    value: Option<bool>,
) {
    if let Some(value) = value {
        object.insert(key.to_owned(), Value::Bool(value));
    }
}

/// Convert a comma-separated list of tag slugs to `NetBox`'s expected
/// `[{"slug": "..."}]` shape. `NetBox` accepts existing tag references this
/// way without requiring the full `NestedTagRequest` object.
pub(crate) fn tags_value(tags_csv: &str) -> Value {
    Value::Array(
        tags_csv
            .split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(|tag| json!({ "slug": tag }))
            .collect(),
    )
}

/// Function pointer to a generated `validate_response` deserializer. The
/// generated module knows the typify type for the resource's response shape;
/// this lets the dispatcher detect schema drift at runtime without baking
/// resource-specific code into the shared helpers.
pub(crate) type ResponseValidator = fn(&Value) -> Result<(), serde_json::Error>;

/// Run the validator on a successful response and warn on mismatch. No-op
/// when the response is null/empty (delete) or a dry-run envelope, and when
/// `NBX_SKIP_RESPONSE_VALIDATION=1` is set.
pub(crate) fn maybe_warn_response_drift(
    response: &Value,
    validator: Option<ResponseValidator>,
    resource_name: &str,
) {
    let Some(validator) = validator else {
        return;
    };
    if std::env::var_os("NBX_SKIP_RESPONSE_VALIDATION").is_some() {
        return;
    }
    if response.is_null() {
        return;
    }
    if response.get("dryRun") == Some(&Value::Bool(true)) {
        return;
    }
    if let Err(error) = validator(response) {
        eprintln!(
            "warning: {resource_name} response did not match the pinned schema ({error}); set NBX_SKIP_RESPONSE_VALIDATION=1 to silence"
        );
    }
}

/// Resolve a foreign-key reference to a numeric `NetBox` ID.
///
/// If `lookup_value` parses as `u64`, it's returned as-is. Otherwise, queries
/// `<api_path>?<lookup_field>=<lookup_value>&limit=2` and returns the single
/// matching record's `id`. Multi-match fails with `validation_error`; no match
/// fails with `not_found`.
pub(crate) async fn resolve_reference_id(
    api_path: &'static str,
    lookup_field: &'static str,
    lookup_value: &str,
    global_options: &GlobalOptions,
) -> NbxResult<Value> {
    if let Ok(numeric_id) = lookup_value.parse::<u64>() {
        return Ok(json!(numeric_id));
    }
    if global_options.dry_run {
        return Ok(Value::String(lookup_value.to_owned()));
    }

    let client = netbox_client_for_options(global_options)?;
    let query_parameters = vec![
        (lookup_field.to_owned(), lookup_value.to_owned()),
        ("limit".to_owned(), "2".to_owned()),
    ];
    let page = client
        .request(HttpMethod::Get, api_path, &query_parameters, None)
        .await?;
    let results = page
        .get("results")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            NbxError::general(format!(
                "{api_path} lookup response did not contain results"
            ))
        })?;
    match results.as_slice() {
        [single] => single
            .get("id")
            .and_then(Value::as_u64)
            .map(|id| json!(id))
            .ok_or_else(|| {
                NbxError::general(format!(
                    "{api_path} lookup result did not contain numeric id"
                ))
            }),
        [] => Err(NbxError::not_found(
            format!("{lookup_value} not found at {api_path}"),
            json!({
                "api_path": api_path,
                "lookup_field": lookup_field,
                "lookup": lookup_value,
            }),
        )),
        _ => Err(NbxError::validation(
            format!(
                "{lookup_value} matched multiple records at {api_path}; pass numeric ID to disambiguate"
            ),
            json!({
                "api_path": api_path,
                "lookup_field": lookup_field,
                "lookup": lookup_value,
            }),
        )),
    }
}

fn base_query_parameters(global_options: &GlobalOptions) -> NbxResult<Vec<(String, String)>> {
    let mut query_parameters = parse_query_string(global_options.query.as_deref())?;
    if let Some(limit) = global_options.limit {
        upsert_query_parameter(&mut query_parameters, "limit", limit.to_string());
    }
    if let Some(offset) = global_options.offset {
        upsert_query_parameter(&mut query_parameters, "offset", offset.to_string());
    }
    Ok(query_parameters)
}

fn lookup_query_parameters(lookup_field: &str, lookup_value: &str) -> Vec<(String, String)> {
    vec![
        (lookup_field.to_owned(), lookup_value.to_owned()),
        ("limit".to_owned(), "2".to_owned()),
    ]
}

fn raw_request_target(
    raw_path: &str,
    global_options: &GlobalOptions,
) -> NbxResult<RawRequestTarget> {
    let (api_path, inline_query) = split_raw_api_path(raw_path)?;
    let mut query_parameters = parse_query_string(inline_query)?;
    query_parameters.extend(parse_query_string(global_options.query.as_deref())?);
    if let Some(limit) = global_options.limit {
        upsert_query_parameter(&mut query_parameters, "limit", limit.to_string());
    }
    if let Some(offset) = global_options.offset {
        upsert_query_parameter(&mut query_parameters, "offset", offset.to_string());
    }
    Ok(RawRequestTarget {
        api_path,
        query_parameters,
    })
}

fn split_raw_api_path(raw_path: &str) -> NbxResult<(RawApiPath, Option<&str>)> {
    let trimmed_raw_path = raw_path.trim();
    let (path_without_query, inline_query) = trimmed_raw_path
        .split_once('?')
        .map_or((trimmed_raw_path, None), |(path, query)| {
            (path, Some(query))
        });
    let api_path = RawApiPath::parse(path_without_query, raw_path)?;
    Ok((api_path, inline_query))
}

fn parse_query_string(query: Option<&str>) -> NbxResult<Vec<(String, String)>> {
    let Some(query) = query else {
        return Ok(Vec::new());
    };

    query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let (key, value) = pair.split_once('=').ok_or_else(|| {
                NbxError::validation(
                    "query passthrough must use key=value pairs",
                    json!({ "pair": pair }),
                )
            })?;
            Ok((key.to_owned(), value.to_owned()))
        })
        .collect()
}

fn next_query_parameters(
    next_url: &str,
    current_query_parameters: &[(String, String)],
) -> Vec<(String, String)> {
    let mut query_parameters = current_query_parameters.to_owned();
    let Some(query_string) = next_url.split_once('?').map(|(_, query)| query) else {
        return query_parameters;
    };

    for pair in query_string.split('&') {
        if let Some((key, value)) = pair.split_once('=')
            && (key == "limit" || key == "offset")
        {
            upsert_query_parameter(&mut query_parameters, key, value.to_owned());
        }
    }

    query_parameters
}

fn upsert_query_parameter(query_parameters: &mut Vec<(String, String)>, key: &str, value: String) {
    if let Some((_, existing_value)) = query_parameters
        .iter_mut()
        .find(|(existing_key, _)| existing_key == key)
    {
        *existing_value = value;
        return;
    }

    query_parameters.push((key.to_owned(), value));
}

fn push_optional_query(
    query_parameters: &mut Vec<(String, String)>,
    key: &str,
    value: Option<String>,
) {
    if let Some(value) = value {
        query_parameters.push((key.to_owned(), value));
    }
}

fn endpoint_for(path: &str, method: HttpMethod) -> NbxResult<&'static EndpointDescriptor> {
    ENDPOINTS
        .iter()
        .find(|endpoint| endpoint.path == path && endpoint.method == method)
        .ok_or_else(|| {
            NbxError::general(format!(
                "generated endpoint metadata missing {} {path}",
                method.as_str(),
            ))
        })
}

fn selected_output_format(global_options: &GlobalOptions) -> OutputFormat {
    global_options
        .output
        .unwrap_or_else(crate::output::default_output_format)
}

fn emit_raw_response(value: Value, output_format: OutputFormat) -> NbxResult<()> {
    if is_netbox_paginated_response(&value) {
        emit_page(&value, output_format)
    } else {
        emit_single(&value, output_format)
    }
}

fn is_netbox_paginated_response(value: &Value) -> bool {
    value.as_object().is_some_and(|object| {
        object.contains_key("count") && object.get("results").and_then(Value::as_array).is_some()
    })
}

fn project_value(value: Value, global_options: &GlobalOptions) -> Value {
    let fields = parse_field_projection(
        global_options
            .field
            .as_deref()
            .or(global_options.output_field.as_deref()),
        global_options.output_field.as_deref() == Some("name"),
    );
    apply_projection(&value, &fields)
}

// ============================================================
// Config helpers (unchanged behavior)
// ============================================================

fn config_init(arguments: ConfigInitArgs) -> NbxResult<()> {
    let mut config = NbxConfig {
        default_context: Some(arguments.context.clone()),
        ..NbxConfig::default()
    };
    config.contexts.insert(
        arguments.context,
        ConfigContext {
            url: arguments.url,
            token: arguments.token,
        },
    );
    save_config(&config)?;
    print_json(&data_envelope(json!({ "status": "initialized" })))
}

fn config_set_url(arguments: ConfigValueArgs) -> NbxResult<()> {
    let mut config = load_config()?;
    if let Some(context_name) = arguments.context.or_else(|| config.default_context.clone()) {
        config.contexts.entry(context_name).or_default().url = Some(arguments.value);
    } else {
        config.url = Some(arguments.value);
    }
    save_config(&config)?;
    print_json(&data_envelope(json!({ "status": "updated" })))
}

fn config_set_token(arguments: ConfigValueArgs) -> NbxResult<()> {
    let mut config = load_config()?;
    if let Some(context_name) = arguments.context.or_else(|| config.default_context.clone()) {
        config.contexts.entry(context_name).or_default().token = Some(arguments.value);
    } else {
        config.token = Some(arguments.value);
    }
    save_config(&config)?;
    print_json(&data_envelope(json!({ "status": "updated" })))
}

fn config_show(arguments: ConfigShowArgs) -> NbxResult<()> {
    let mut config = serde_json::to_value(load_config()?)
        .map_err(|error| NbxError::general(format!("failed to serialize config: {error}")))?;
    if !arguments.show_token {
        redact_config_tokens(&mut config);
    }
    print_json(&data_envelope(config))
}

fn config_use_context(arguments: ContextNameArgs) -> NbxResult<()> {
    let mut config = load_config()?;
    if !config.contexts.contains_key(&arguments.name) {
        return Err(NbxError::not_found(
            format!("context not found: {}", arguments.name),
            json!({ "context": arguments.name }),
        ));
    }
    config.default_context = Some(arguments.name);
    save_config(&config)?;
    print_json(&data_envelope(json!({ "status": "updated" })))
}

fn config_current_context() -> NbxResult<()> {
    let config = load_config()?;
    print_json(&data_envelope(json!({ "context": config.default_context })))
}

fn config_get_contexts() -> NbxResult<()> {
    let config = load_config()?;
    let contexts = config.contexts.keys().cloned().collect::<Vec<_>>();
    print_json(&data_envelope(json!({ "contexts": contexts })))
}

fn redact_config_tokens(value: &mut Value) {
    if let Some(token) = value.get("token").and_then(Value::as_str).map(redact_token) {
        value["token"] = json!(token);
    }

    if let Some(contexts) = value.get_mut("contexts").and_then(Value::as_object_mut) {
        for context in contexts.values_mut() {
            if let Some(token) = context
                .get("token")
                .and_then(Value::as_str)
                .map(redact_token)
            {
                context["token"] = json!(token);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{
        body_json, method as method_matcher, path as path_matcher, query_param,
    };
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn global_options_for_mock_server(server: &MockServer) -> GlobalOptions {
        // SAFETY: command tests run sequentially within their own #[tokio::test]
        // runtimes. The version-probe env var only needs to be present for the
        // duration of the test binary; setting it once here is sufficient.
        unsafe {
            std::env::set_var("NBX_SKIP_VERSION_CHECK", "1");
        }
        GlobalOptions {
            url: Some(server.uri()),
            token: Some("test-token".to_owned()),
            context: None,
            output: Some(OutputFormat::Json),
            field: None,
            output_field: None,
            query: None,
            limit: None,
            offset: None,
            page_all: false,
            timeout: 5,
            verbose: false,
            dry_run: false,
        }
    }

    fn global_options_without_connection() -> GlobalOptions {
        GlobalOptions {
            url: None,
            token: None,
            context: None,
            output: Some(OutputFormat::Json),
            field: None,
            output_field: None,
            query: None,
            limit: None,
            offset: None,
            page_all: false,
            timeout: 5,
            verbose: false,
            dry_run: false,
        }
    }

    #[test]
    fn every_resource_spec_path_resolves_in_generated_endpoints() {
        let mut resources = vec![DEVICES, INTERFACES];
        resources.extend(crate::generated::resources::all_resource_specs());

        let known_paths: std::collections::HashSet<&'static str> =
            ENDPOINTS.iter().map(|endpoint| endpoint.path).collect();
        for resource in resources {
            assert!(
                known_paths.contains(resource.api_path),
                "{} api_path {} missing from generated endpoints",
                resource.name,
                resource.api_path,
            );
            assert!(
                known_paths.contains(resource.detail_path),
                "{} detail_path {} missing from generated endpoints",
                resource.name,
                resource.detail_path,
            );
        }
        assert!(
            known_paths.contains(INTERFACES_TRACE_PATH),
            "interfaces trace path {INTERFACES_TRACE_PATH} missing from generated endpoints",
        );
    }

    #[tokio::test]
    async fn dry_run_resource_id_lookup_uses_placeholder_and_resolution_preview() {
        let mut global_options = global_options_without_connection();
        global_options.dry_run = true;

        let resolved_id = resolve_resource_id(
            crate::generated::resources::prefixes::RESOURCE,
            "192.0.2.0/24",
            None,
            &global_options,
        )
        .await
        .expect("dry-run lookup should not require a NetBox connection");

        assert_eq!(resolved_id.path_segment(), "{resolved-id}");
        assert_eq!(
            resolved_id
                .dry_run_id_resolution()
                .expect("lookup dry-run should include resolution details"),
            json!({
                "resource": "ipam.prefixes",
                "lookup": "192.0.2.0/24",
                "lookupField": "prefix",
                "method": "GET",
                "path": "/api/ipam/prefixes/",
                "query": [
                    ["prefix", "192.0.2.0/24"],
                    ["limit", "2"]
                ],
            })
        );
    }

    #[tokio::test]
    async fn status_command_only_requires_url() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/status/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "netbox-version": "4.5.10",
            })))
            .expect(1)
            .mount(&server)
            .await;

        let mut global_options = global_options_for_mock_server(&server);
        global_options.token = None;

        run_status_command(&global_options)
            .await
            .expect("status should run without a token");
    }

    #[test]
    fn parses_query_pairs() {
        assert_eq!(
            parse_query_string(Some("site=dc1&status=active")).expect("query should parse"),
            vec![
                ("site".to_owned(), "dc1".to_owned()),
                ("status".to_owned(), "active".to_owned())
            ]
        );
    }

    #[test]
    fn rejects_malformed_query_pairs() {
        assert!(parse_query_string(Some("site")).is_err());
    }

    #[test]
    fn parses_raw_http_method_case_insensitively() {
        assert_eq!(
            parse_raw_http_method("get").expect("lowercase method should parse"),
            RawHttpMethod::Get
        );
        assert_eq!(
            parse_raw_http_method("POST").expect("uppercase method should parse"),
            RawHttpMethod::Post
        );
    }

    #[test]
    fn raw_request_target_accepts_api_path_with_inline_query() {
        let mut global_options = global_options_without_connection();
        global_options.query = Some("status=active".to_owned());
        global_options.limit = Some(25);

        let raw_request_target =
            raw_request_target("api/plugins/example/widgets/?name=leaf-1", &global_options)
                .expect("raw path should parse");

        assert_eq!(
            raw_request_target.api_path,
            RawApiPath("/api/plugins/example/widgets/".to_owned())
        );
        assert_eq!(
            raw_request_target.query_parameters,
            vec![
                ("name".to_owned(), "leaf-1".to_owned()),
                ("status".to_owned(), "active".to_owned()),
                ("limit".to_owned(), "25".to_owned()),
            ],
        );
    }

    #[test]
    fn raw_request_target_rejects_non_api_paths() {
        let global_options = global_options_without_connection();

        let error = raw_request_target("/dcim/devices/", &global_options)
            .expect_err("non-API path should be rejected");

        assert_eq!(error.code, crate::error::ErrorCode::ValidationError);
    }

    #[tokio::test]
    async fn raw_command_calls_unmodeled_endpoint_with_query_and_body() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("POST"))
            .and(path_matcher("/api/plugins/example/widgets/"))
            .and(query_param("sync", "true"))
            .and(body_json(json!({ "name": "widget-a" })))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": 1,
                "name": "widget-a",
            })))
            .expect(1)
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);

        run_raw_command(
            RawArgs {
                method: RawHttpMethod::Post,
                path: "/api/plugins/example/widgets/?sync=true".to_owned(),
                data: Some(json!({ "name": "widget-a" }).to_string()),
                data_file: None,
            },
            &global_options,
        )
        .await
        .expect("raw command should call arbitrary API path");
    }

    #[tokio::test]
    async fn raw_page_all_dry_run_does_not_require_a_connection() {
        let mut global_options = global_options_without_connection();
        global_options.page_all = true;
        global_options.dry_run = true;

        run_raw_command(
            RawArgs {
                method: RawHttpMethod::Get,
                path: "/api/plugins/example/widgets/".to_owned(),
                data: None,
                data_file: None,
            },
            &global_options,
        )
        .await
        .expect("dry-run raw page-all should not require NetBox connection settings");
    }

    #[tokio::test]
    async fn raw_page_all_rejects_mutation_methods_before_connection_resolution() {
        let mut global_options = global_options_without_connection();
        global_options.page_all = true;

        let error = run_raw_command(
            RawArgs {
                method: RawHttpMethod::Post,
                path: "/api/plugins/example/widgets/".to_owned(),
                data: None,
                data_file: None,
            },
            &global_options,
        )
        .await
        .expect_err("raw page-all should reject mutation methods");

        assert_eq!(error.code, crate::error::ErrorCode::ValidationError);
        assert!(error.message.contains("only supports GET"));
    }

    #[tokio::test]
    async fn raw_page_all_rejects_body_before_json_parsing_or_connection_resolution() {
        let mut global_options = global_options_without_connection();
        global_options.page_all = true;

        let error = run_raw_command(
            RawArgs {
                method: RawHttpMethod::Get,
                path: "/api/plugins/example/widgets/".to_owned(),
                data: Some("{not-json".to_owned()),
                data_file: None,
            },
            &global_options,
        )
        .await
        .expect_err("raw page-all should reject bodies without parsing them");

        assert_eq!(error.code, crate::error::ErrorCode::ValidationError);
        assert!(error.message.contains("does not support request bodies"));
    }

    #[test]
    fn parses_numeric_interface_address() {
        assert_eq!(
            parse_interface_address("42").expect("numeric address parses"),
            InterfaceAddress::Numeric(42)
        );
    }

    #[test]
    fn parses_composite_interface_address() {
        assert_eq!(
            parse_interface_address("srv01:Ethernet1").expect("composite address parses"),
            InterfaceAddress::Composite {
                device_lookup: "srv01".to_owned(),
                interface_name: "Ethernet1".to_owned(),
            }
        );
    }

    #[test]
    fn rejects_bare_interface_name() {
        let error = parse_interface_address("Ethernet1").expect_err("bare name should be rejected");
        assert_eq!(error.code, crate::error::ErrorCode::ValidationError);
    }

    #[test]
    fn rejects_empty_segments_in_composite_address() {
        assert!(parse_interface_address(":Ethernet1").is_err());
        assert!(parse_interface_address("srv01:").is_err());
    }

    #[tokio::test]
    async fn composite_address_resolves_through_device_then_interface_lookup() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/devices/"))
            .and(query_param("name", "srv01"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 1,
                "results": [{ "id": 7, "name": "srv01" }],
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/interfaces/"))
            .and(query_param("device_id", "7"))
            .and(query_param("name", "Ethernet1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 1,
                "results": [{ "id": 91, "name": "Ethernet1", "device": { "id": 7, "name": "srv01" } }],
            })))
            .expect(1)
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let id = resolve_interface_id("srv01:Ethernet1", &global_options)
            .await
            .expect("composite address should resolve");
        assert_eq!(id, "91");
    }

    #[tokio::test]
    async fn composite_address_with_unknown_device_returns_not_found() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/devices/"))
            .and(query_param("name", "ghost"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 0,
                "results": [],
            })))
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let error = resolve_interface_id("ghost:Ethernet1", &global_options)
            .await
            .expect_err("unknown device should fail lookup");
        assert_eq!(error.code, crate::error::ErrorCode::NotFound);
    }

    #[tokio::test]
    async fn vlan_name_resolution_multi_match_returns_validation_error_with_disambiguation_hint() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/ipam/vlans/"))
            .and(query_param("name", "dev-prod"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 2,
                "results": [
                    { "id": 100, "name": "dev-prod" },
                    { "id": 200, "name": "dev-prod" },
                ],
            })))
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let error = resolve_vlan_reference("dev-prod", &global_options)
            .await
            .expect_err("multi-match VLAN lookup should fail");
        assert_eq!(error.code, crate::error::ErrorCode::ValidationError);
        assert!(
            error.message.contains("disambiguate"),
            "message should hint at numeric ID disambiguation: {}",
            error.message
        );
    }

    #[tokio::test]
    async fn vlan_name_resolution_single_match_returns_numeric_id() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/ipam/vlans/"))
            .and(query_param("name", "mgmt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 1,
                "results": [{ "id": 42, "name": "mgmt" }],
            })))
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let resolved = resolve_vlan_reference("mgmt", &global_options)
            .await
            .expect("single-match VLAN lookup should resolve");
        assert_eq!(resolved, json!(42));
    }

    #[tokio::test]
    async fn vlan_name_resolution_passes_numeric_through_unchanged() {
        let server = MockServer::start().await;
        let global_options = global_options_for_mock_server(&server);
        let resolved = resolve_vlan_reference("123", &global_options)
            .await
            .expect("numeric VLAN ref should pass through");
        assert_eq!(resolved, json!(123));
    }

    #[tokio::test]
    async fn cable_trace_calls_get_trace_endpoint() {
        let server = MockServer::start().await;

        let trace_payload = json!([
            [
                { "object_type": "dcim.interface", "id": 91, "name": "Ethernet1" },
                { "object_type": "dcim.cable", "id": 5, "label": "patch-7" },
                { "object_type": "dcim.frontport", "id": 12, "name": "FrontPort1" },
            ]
        ]);

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/interfaces/91/trace/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(trace_payload.clone()))
            .expect(1)
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        interface_trace(
            InterfaceTargetArgs {
                address: "91".to_owned(),
            },
            &global_options,
        )
        .await
        .expect("trace should succeed against mock server");
    }

    #[tokio::test]
    async fn page_all_returns_stream_error_when_later_page_fails() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/ipam/vlans/"))
            .and(query_param("limit", "1"))
            .and(query_param("offset", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 2,
                "next": format!("{}/api/ipam/vlans/?limit=1&offset=1", server.uri()),
                "previous": null,
                "results": [{ "id": 1, "name": "first" }],
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/ipam/vlans/"))
            .and(query_param("limit", "1"))
            .and(query_param("offset", "1"))
            .respond_with(ResponseTemplate::new(503).set_body_json(json!({
                "detail": "temporary upstream failure",
            })))
            .expect(1)
            .mount(&server)
            .await;

        let mut global_options = global_options_for_mock_server(&server);
        global_options.page_all = true;
        global_options.limit = Some(1);

        let error = list_with_query(
            crate::generated::resources::vlans::RESOURCE,
            vec![
                ("limit".to_owned(), "1".to_owned()),
                ("offset".to_owned(), "0".to_owned()),
            ],
            &global_options,
        )
        .await
        .expect_err("mid-stream page failure should return stream_error");

        assert_eq!(error.code, crate::error::ErrorCode::StreamError);
    }

    #[tokio::test]
    async fn device_list_repeats_role_query_for_each_value() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/devices/"))
            .and(query_param("status", "active"))
            .and(query_param("role", "database-server"))
            .and(query_param("role", "application-server"))
            .and(query_param("role", "cache-server"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 0,
                "results": [],
            })))
            .expect(1)
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let arguments = DeviceListArgs {
            site: None,
            role: vec![
                "database-server".to_owned(),
                "application-server".to_owned(),
                "cache-server".to_owned(),
            ],
            status: Some("active".to_owned()),
            tag: None,
            name: None,
        };
        device_list(arguments, &global_options)
            .await
            .expect("multi-role device list should succeed");
    }

    #[tokio::test]
    async fn inventory_items_list_resolves_device_slug_and_filters_by_role() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/devices/"))
            .and(query_param("name", "srv-db-07"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 1,
                "results": [{ "id": 42, "name": "srv-db-07" }],
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/inventory-items/"))
            .and(query_param("device_id", "42"))
            .and(query_param("role", "drive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 0,
                "results": [],
            })))
            .expect(1)
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let arguments = InventoryItemListArgs {
            device: Some("srv-db-07".to_owned()),
            role: vec!["drive".to_owned()],
            manufacturer: None,
            serial: None,
            name: None,
            tag: None,
        };
        inventory_items_list(arguments, &global_options)
            .await
            .expect("inventory-items list with --device slug should succeed");
    }

    #[tokio::test]
    async fn inventory_items_list_passes_numeric_device_through_unchanged() {
        let server = MockServer::start().await;

        Mock::given(method_matcher("GET"))
            .and(path_matcher("/api/dcim/inventory-items/"))
            .and(query_param("device_id", "99"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 0,
                "results": [],
            })))
            .expect(1)
            .mount(&server)
            .await;

        let global_options = global_options_for_mock_server(&server);
        let arguments = InventoryItemListArgs {
            device: Some("99".to_owned()),
            role: vec![],
            manufacturer: None,
            serial: None,
            name: None,
            tag: None,
        };
        inventory_items_list(arguments, &global_options)
            .await
            .expect("inventory-items list with numeric --device should skip name lookup");
    }
}
