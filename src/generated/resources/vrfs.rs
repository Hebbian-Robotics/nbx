// AUTO-GENERATED. Source: schema/netbox-4.6.0.json :: VRFRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/resources/` to regenerate.

use clap::Args;
use clap::Subcommand;
use serde_json::{Map, Value, json};

use crate::commands::{GlobalOptions, ResourceSpec};
use crate::commands::insert_optional_bool_field;
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
    name: "vrfs",
    api_path: "/api/ipam/vrfs/",
    detail_path: "/api/ipam/vrfs/{id}/",
    default_lookup_field: "name",
};

#[derive(Debug, Args)]
pub struct VrfCreateArgs {
    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// Prevent duplicate prefixes/IP addresses within this VRF
    #[arg(long)]
    pub enforce_unique: Option<bool>,

    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub owner: Option<u64>,

    /// Unique route distinguisher (as defined in RFC 4364)
    #[arg(long)]
    pub rd: Option<String>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VrfUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// Prevent duplicate prefixes/IP addresses within this VRF
    #[arg(long)]
    pub enforce_unique: Option<bool>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// Unique route distinguisher (as defined in RFC 4364)
    #[arg(long)]
    pub rd: Option<String>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum VrfAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(VrfCreateArgs),
    Update(VrfUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_vrf_action(action: VrfAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        VrfAction::List(args) => generic_list(RESOURCE, args, opts).await,
        VrfAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        VrfAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        VrfAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        VrfAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        VrfAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        VrfAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: VrfCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_bool_field(object, "enforce_unique", args.enforce_unique);
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "rd", args.rd);
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    Ok(body)
}

async fn update_body(args: VrfUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_bool_field(object, "enforce_unique", args.enforce_unique);
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "rd", args.rd);
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::Vrf>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Vrf>>(value.clone()).map(|_| ())
}

