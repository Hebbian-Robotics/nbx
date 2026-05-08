// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: WritableVLANRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.

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
    name: "vlans",
    api_path: "/api/ipam/vlans/",
    detail_path: "/api/ipam/vlans/{id}/",
    default_lookup_field: "name",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VlanQinqRole {
    Svlan,
    Cvlan,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum VlanStatus {
    Active,
    Reserved,
    Deprecated,
}

#[derive(Debug, Args)]
pub struct VlanCreateArgs {
    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub group: Option<String>,

    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub owner: Option<u64>,

    /// Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)
    ///
    /// * `svlan` - Service
    /// * `cvlan` - Customer
    #[arg(long)]
    pub qinq_role: Option<VlanQinqRole>,

    #[arg(long)]
    pub qinq_svlan: Option<u64>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub site: Option<String>,

    /// Operational status of this VLAN
    ///
    /// * `active` - Active
    /// * `reserved` - Reserved
    /// * `deprecated` - Deprecated
    #[arg(long)]
    pub status: Option<VlanStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// Numeric VLAN ID (1-4094)
    #[arg(long)]
    pub vid: u64,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VlanUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub group: Option<String>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// Customer/service VLAN designation (for Q-in-Q/IEEE 802.1ad)
    ///
    /// * `svlan` - Service
    /// * `cvlan` - Customer
    #[arg(long)]
    pub qinq_role: Option<VlanQinqRole>,

    #[arg(long)]
    pub qinq_svlan: Option<u64>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub site: Option<String>,

    /// Operational status of this VLAN
    ///
    /// * `active` - Active
    /// * `reserved` - Reserved
    /// * `deprecated` - Deprecated
    #[arg(long)]
    pub status: Option<VlanStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// Numeric VLAN ID (1-4094)
    #[arg(long)]
    pub vid: Option<u64>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum VlanAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(VlanCreateArgs),
    Update(VlanUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_vlan_action(action: VlanAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        VlanAction::List(args) => generic_list(RESOURCE, args, opts).await,
        VlanAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        VlanAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        VlanAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        VlanAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        VlanAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        VlanAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: VlanCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.group {
        object.insert("group".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlan-groups/", "slug", &v, opts).await?);
    }
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.qinq_role {
        object.insert("qinq_role".to_owned(), json!(v));
    }
    if let Some(v) = args.qinq_svlan {
        object.insert("qinq_svlan".to_owned(), json!(v));
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/ipam/roles/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.site {
        object.insert("site".to_owned(), crate::commands::resolve_reference_id("/api/dcim/sites/", "slug", &v, opts).await?);
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
    object.insert("vid".to_owned(), json!(args.vid));
    Ok(body)
}

async fn update_body(args: VlanUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.group {
        object.insert("group".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlan-groups/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.qinq_role {
        object.insert("qinq_role".to_owned(), json!(v));
    }
    if let Some(v) = args.qinq_svlan {
        object.insert("qinq_svlan".to_owned(), json!(v));
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/ipam/roles/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.site {
        object.insert("site".to_owned(), crate::commands::resolve_reference_id("/api/dcim/sites/", "slug", &v, opts).await?);
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
    if let Some(v) = args.vid {
        object.insert("vid".to_owned(), json!(v));
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::Vlan>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Vlan>>(value.clone()).map(|_| ())
}

