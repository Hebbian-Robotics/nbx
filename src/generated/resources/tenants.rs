// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: TenantRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.

use clap::Args;
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
    app: "tenancy",
    name: "tenants",
    api_path: "/api/tenancy/tenants/",
    detail_path: "/api/tenancy/tenants/{id}/",
    default_lookup_field: "slug",
};

#[derive(Debug, Args)]
pub struct TenantCreateArgs {
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

    #[arg(long)]
    pub slug: String,

    #[arg(long)]
    pub tags: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct TenantUpdateArgs {
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

    #[arg(long)]
    pub slug: Option<String>,

    #[arg(long)]
    pub tags: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum TenantAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(TenantCreateArgs),
    Update(TenantUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_tenant_action(action: TenantAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        TenantAction::List(args) => generic_list(RESOURCE, args, opts).await,
        TenantAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        TenantAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        TenantAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        TenantAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        TenantAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        TenantAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: TenantCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.group {
        object.insert("group".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenant-groups/", "slug", &v, opts).await?);
    }
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    object.insert("slug".to_owned(), Value::String(args.slug));
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    Ok(body)
}

async fn update_body(args: TenantUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.group {
        object.insert("group".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenant-groups/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "slug", args.slug);
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::Tenant>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Tenant>>(value.clone()).map(|_| ())
}

