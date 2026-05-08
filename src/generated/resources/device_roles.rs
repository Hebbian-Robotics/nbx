// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: WritableDeviceRoleRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.

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
    app: "dcim",
    name: "device-roles",
    api_path: "/api/dcim/device-roles/",
    detail_path: "/api/dcim/device-roles/{id}/",
    default_lookup_field: "slug",
};

#[derive(Debug, Args)]
pub struct DeviceRoleCreateArgs {
    #[arg(long)]
    pub color: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub config_template: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub parent: Option<u64>,

    #[arg(long)]
    pub slug: String,

    #[arg(long)]
    pub tags: Option<String>,

    /// Virtual machines may be assigned to this role
    #[arg(long)]
    pub vm_role: Option<bool>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeviceRoleUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    #[arg(long)]
    pub color: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub config_template: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub parent: Option<u64>,

    #[arg(long)]
    pub slug: Option<String>,

    #[arg(long)]
    pub tags: Option<String>,

    /// Virtual machines may be assigned to this role
    #[arg(long)]
    pub vm_role: Option<bool>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum DeviceRoleAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(DeviceRoleCreateArgs),
    Update(DeviceRoleUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_device_role_action(action: DeviceRoleAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        DeviceRoleAction::List(args) => generic_list(RESOURCE, args, opts).await,
        DeviceRoleAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        DeviceRoleAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        DeviceRoleAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        DeviceRoleAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        DeviceRoleAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        DeviceRoleAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: DeviceRoleCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "color", args.color);
    insert_optional_string_field(object, "comments", args.comments);
    if let Some(v) = args.config_template {
        object.insert("config_template".to_owned(), crate::commands::resolve_reference_id("/api/extras/config-templates/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "description", args.description);
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.parent {
        object.insert("parent".to_owned(), json!(v));
    }
    object.insert("slug".to_owned(), Value::String(args.slug));
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    insert_optional_bool_field(object, "vm_role", args.vm_role);
    Ok(body)
}

async fn update_body(args: DeviceRoleUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "color", args.color);
    insert_optional_string_field(object, "comments", args.comments);
    if let Some(v) = args.config_template {
        object.insert("config_template".to_owned(), crate::commands::resolve_reference_id("/api/extras/config-templates/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.parent {
        object.insert("parent".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "slug", args.slug);
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    insert_optional_bool_field(object, "vm_role", args.vm_role);
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::DeviceRole>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::DeviceRole>>(value.clone()).map(|_| ())
}

