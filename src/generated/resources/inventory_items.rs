// AUTO-GENERATED. Source: schema/netbox-4.6.0.json :: WritableInventoryItemRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/resources/` to regenerate.

use clap::Args;
use clap::ValueEnum;
use clap::Subcommand;
use serde_json::{Map, Value, json};

use crate::commands::{GlobalOptions, ResourceSpec};
use crate::commands::insert_optional_bool_field;
use crate::commands::tags_value;
use crate::commands::insert_optional_string_field;
use crate::commands::optional_json_body;
use crate::commands::{
    BulkDeleteArgs, DeleteArgs, GenericMutationArgs, GetArgs,
    bulk_delete_resource, delete_resource, generic_get,
    generic_mutation_body, mutate_collection_with_body,
    update_resource_with_body,
};
use crate::commands::InventoryItemListArgs;
use crate::generated::endpoints::HttpMethod;
use crate::error::{NbxError, NbxResult};

pub const RESOURCE: ResourceSpec = ResourceSpec {
    app: "dcim",
    name: "inventory-items",
    api_path: "/api/dcim/inventory-items/",
    detail_path: "/api/dcim/inventory-items/{id}/",
    default_lookup_field: "name",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InventoryItemStatus {
    Offline,
    Active,
    Planned,
    Staged,
    Failed,
    Decommissioning,
}

#[derive(Debug, Args)]
pub struct InventoryItemCreateArgs {
    /// A unique tag used to identify this item
    #[arg(long)]
    pub asset_tag: Option<String>,

    #[arg(long)]
    pub component_id: Option<u64>,

    #[arg(long)]
    pub component_type: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub device: String,

    /// This item was automatically discovered
    #[arg(long)]
    pub discovered: Option<bool>,

    /// Physical label
    #[arg(long)]
    pub label: Option<String>,

    #[arg(long)]
    pub manufacturer: Option<String>,

    #[arg(long)]
    pub name: String,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub parent: Option<u64>,

    /// Manufacturer-assigned part identifier
    #[arg(long)]
    pub part_id: Option<String>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub serial: Option<String>,

    /// * `offline` - Offline
    /// * `active` - Active
    /// * `planned` - Planned
    /// * `staged` - Staged
    /// * `failed` - Failed
    /// * `decommissioning` - Decommissioning
    #[arg(long)]
    pub status: Option<InventoryItemStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct InventoryItemUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    /// A unique tag used to identify this item
    #[arg(long)]
    pub asset_tag: Option<String>,

    #[arg(long)]
    pub component_id: Option<u64>,

    #[arg(long)]
    pub component_type: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub device: Option<String>,

    /// This item was automatically discovered
    #[arg(long)]
    pub discovered: Option<bool>,

    /// Physical label
    #[arg(long)]
    pub label: Option<String>,

    #[arg(long)]
    pub manufacturer: Option<String>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub parent: Option<u64>,

    /// Manufacturer-assigned part identifier
    #[arg(long)]
    pub part_id: Option<String>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub serial: Option<String>,

    /// * `offline` - Offline
    /// * `active` - Active
    /// * `planned` - Planned
    /// * `staged` - Staged
    /// * `failed` - Failed
    /// * `decommissioning` - Decommissioning
    #[arg(long)]
    pub status: Option<InventoryItemStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum InventoryItemAction {
    List(InventoryItemListArgs),
    Get(GetArgs),
    Create(InventoryItemCreateArgs),
    Update(InventoryItemUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_inventory_item_action(action: InventoryItemAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        InventoryItemAction::List(args) => crate::commands::inventory_items_list(args, opts).await,
        InventoryItemAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        InventoryItemAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        InventoryItemAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        InventoryItemAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        InventoryItemAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        InventoryItemAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: InventoryItemCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "asset_tag", args.asset_tag);
    if let Some(v) = args.component_id {
        object.insert("component_id".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "component_type", args.component_type);
    insert_optional_string_field(object, "description", args.description);
    object.insert("device".to_owned(), crate::commands::resolve_reference_id("/api/dcim/devices/", "name", &args.device, opts).await?);
    insert_optional_bool_field(object, "discovered", args.discovered);
    insert_optional_string_field(object, "label", args.label);
    if let Some(v) = args.manufacturer {
        object.insert("manufacturer".to_owned(), crate::commands::resolve_reference_id("/api/dcim/manufacturers/", "slug", &v, opts).await?);
    }
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.parent {
        object.insert("parent".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "part_id", args.part_id);
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/dcim/inventory-item-roles/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "serial", args.serial);
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    Ok(body)
}

async fn update_body(args: InventoryItemUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "asset_tag", args.asset_tag);
    if let Some(v) = args.component_id {
        object.insert("component_id".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "component_type", args.component_type);
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.device {
        object.insert("device".to_owned(), crate::commands::resolve_reference_id("/api/dcim/devices/", "name", &v, opts).await?);
    }
    insert_optional_bool_field(object, "discovered", args.discovered);
    insert_optional_string_field(object, "label", args.label);
    if let Some(v) = args.manufacturer {
        object.insert("manufacturer".to_owned(), crate::commands::resolve_reference_id("/api/dcim/manufacturers/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.parent {
        object.insert("parent".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "part_id", args.part_id);
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/dcim/inventory-item-roles/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "serial", args.serial);
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
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
    serde_json::from_value::<crate::generated::types::InventoryItem>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::InventoryItem>>(value.clone()).map(|_| ())
}

