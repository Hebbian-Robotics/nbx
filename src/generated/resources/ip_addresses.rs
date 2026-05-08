// AUTO-GENERATED. Source: schema/netbox-4.6.0.json :: WritableIPAddressRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/resources/` to regenerate.

use clap::Args;
use clap::ValueEnum;
use clap::Subcommand;
use serde_json::{Map, Value, json};

use crate::commands::{GlobalOptions, ResourceSpec};
use crate::commands::tags_value;
use crate::commands::insert_optional_string_field;
use crate::commands::optional_json_body;
use crate::commands::{
    BulkDeleteArgs, DeleteArgs, GenericListArgs, GenericMutationArgs, GetArgs,
    bulk_delete_resource, delete_resource, generic_get,
    generic_list, generic_mutation_body, mutate_collection_with_body,
    update_resource_with_body,
};
use crate::generated::endpoints::HttpMethod;
use crate::error::{NbxError, NbxResult};

pub const RESOURCE: ResourceSpec = ResourceSpec {
    app: "ipam",
    name: "ip-addresses",
    api_path: "/api/ipam/ip-addresses/",
    detail_path: "/api/ipam/ip-addresses/{id}/",
    default_lookup_field: "address",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IpAddressRole {
    Loopback,
    Secondary,
    Anycast,
    Vip,
    Vrrp,
    Hsrp,
    Glbp,
    Carp,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum IpAddressStatus {
    Active,
    Reserved,
    Deprecated,
    Dhcp,
    Slaac,
}

#[derive(Debug, Args)]
pub struct IpAddressCreateArgs {
    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub assigned_object_id: Option<u64>,

    #[arg(long)]
    pub assigned_object_type: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// Hostname or FQDN (not case-sensitive)
    #[arg(long)]
    pub dns_name: Option<String>,

    /// The IP for which this address is the "outside" IP
    #[arg(long)]
    pub nat_inside: Option<u64>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// The functional role of this IP
    ///
    /// * `loopback` - Loopback
    /// * `secondary` - Secondary
    /// * `anycast` - Anycast
    /// * `vip` - VIP
    /// * `vrrp` - VRRP
    /// * `hsrp` - HSRP
    /// * `glbp` - GLBP
    /// * `carp` - CARP
    #[arg(long)]
    pub role: Option<IpAddressRole>,

    /// The operational status of this IP
    ///
    /// * `active` - Active
    /// * `reserved` - Reserved
    /// * `deprecated` - Deprecated
    /// * `dhcp` - DHCP
    /// * `slaac` - SLAAC
    #[arg(long)]
    pub status: Option<IpAddressStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    #[arg(long)]
    pub vrf: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct IpAddressUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    #[arg(long)]
    pub address: Option<String>,

    #[arg(long)]
    pub assigned_object_id: Option<u64>,

    #[arg(long)]
    pub assigned_object_type: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// Hostname or FQDN (not case-sensitive)
    #[arg(long)]
    pub dns_name: Option<String>,

    /// The IP for which this address is the "outside" IP
    #[arg(long)]
    pub nat_inside: Option<u64>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// The functional role of this IP
    ///
    /// * `loopback` - Loopback
    /// * `secondary` - Secondary
    /// * `anycast` - Anycast
    /// * `vip` - VIP
    /// * `vrrp` - VRRP
    /// * `hsrp` - HSRP
    /// * `glbp` - GLBP
    /// * `carp` - CARP
    #[arg(long)]
    pub role: Option<IpAddressRole>,

    /// The operational status of this IP
    ///
    /// * `active` - Active
    /// * `reserved` - Reserved
    /// * `deprecated` - Deprecated
    /// * `dhcp` - DHCP
    /// * `slaac` - SLAAC
    #[arg(long)]
    pub status: Option<IpAddressStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    #[arg(long)]
    pub vrf: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum IpAddressAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(IpAddressCreateArgs),
    Update(IpAddressUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_ip_address_action(action: IpAddressAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        IpAddressAction::List(args) => generic_list(RESOURCE, args, opts).await,
        IpAddressAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        IpAddressAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        IpAddressAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        IpAddressAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        IpAddressAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        IpAddressAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: IpAddressCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    object.insert("address".to_owned(), Value::String(args.address));
    if let Some(v) = args.assigned_object_id {
        object.insert("assigned_object_id".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "assigned_object_type", args.assigned_object_type);
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "dns_name", args.dns_name);
    if let Some(v) = args.nat_inside {
        object.insert("nat_inside".to_owned(), json!(v));
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), json!(v));
    }
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.vrf {
        object.insert("vrf".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vrfs/", "name", &v, opts).await?);
    }
    Ok(body)
}

async fn update_body(args: IpAddressUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "address", args.address);
    if let Some(v) = args.assigned_object_id {
        object.insert("assigned_object_id".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "assigned_object_type", args.assigned_object_type);
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "dns_name", args.dns_name);
    if let Some(v) = args.nat_inside {
        object.insert("nat_inside".to_owned(), json!(v));
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), json!(v));
    }
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.vrf {
        object.insert("vrf".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vrfs/", "name", &v, opts).await?);
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::IpAddress>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::IpAddress>>(value.clone()).map(|_| ())
}

