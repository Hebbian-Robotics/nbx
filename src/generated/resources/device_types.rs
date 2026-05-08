// AUTO-GENERATED. Source: schema/netbox-4.6.0.json :: WritableDeviceTypeRequest
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
    BulkDeleteArgs, DeleteArgs, GenericListArgs, GenericMutationArgs, GetArgs,
    bulk_delete_resource, delete_resource, generic_get,
    generic_list, generic_mutation_body, mutate_collection_with_body,
    update_resource_with_body,
};
use crate::generated::endpoints::HttpMethod;
use crate::error::{NbxError, NbxResult};

pub const RESOURCE: ResourceSpec = ResourceSpec {
    app: "dcim",
    name: "device-types",
    api_path: "/api/dcim/device-types/",
    detail_path: "/api/dcim/device-types/{id}/",
    default_lookup_field: "slug",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceTypeAirflow {
    #[serde(rename = "front-to-rear")]
    #[value(name = "front-to-rear")]
    FrontToRear,
    #[serde(rename = "rear-to-front")]
    #[value(name = "rear-to-front")]
    RearToFront,
    #[serde(rename = "left-to-right")]
    #[value(name = "left-to-right")]
    LeftToRight,
    #[serde(rename = "right-to-left")]
    #[value(name = "right-to-left")]
    RightToLeft,
    #[serde(rename = "side-to-rear")]
    #[value(name = "side-to-rear")]
    SideToRear,
    #[serde(rename = "rear-to-side")]
    #[value(name = "rear-to-side")]
    RearToSide,
    #[serde(rename = "bottom-to-top")]
    #[value(name = "bottom-to-top")]
    BottomToTop,
    #[serde(rename = "top-to-bottom")]
    #[value(name = "top-to-bottom")]
    TopToBottom,
    Passive,
    Mixed,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceTypeSubdeviceRole {
    Parent,
    Child,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceTypeWeightUnit {
    Kg,
    G,
    Lb,
    Oz,
}

#[derive(Debug, Args)]
pub struct DeviceTypeCreateArgs {
    /// * `front-to-rear` - Front to rear
    /// * `rear-to-front` - Rear to front
    /// * `left-to-right` - Left to right
    /// * `right-to-left` - Right to left
    /// * `side-to-rear` - Side to rear
    /// * `rear-to-side` - Rear to side
    /// * `bottom-to-top` - Bottom to top
    /// * `top-to-bottom` - Top to bottom
    /// * `passive` - Passive
    /// * `mixed` - Mixed
    #[arg(long)]
    pub airflow: Option<DeviceTypeAirflow>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub default_platform: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// Devices of this type are excluded when calculating rack utilization.
    #[arg(long)]
    pub exclude_from_utilization: Option<bool>,

    #[arg(long)]
    pub front_image: Option<String>,

    /// Device consumes both front and rear rack faces.
    #[arg(long)]
    pub is_full_depth: Option<bool>,

    #[arg(long)]
    pub manufacturer: String,

    #[arg(long)]
    pub model: String,

    #[arg(long)]
    pub owner: Option<u64>,

    /// Discrete part number (optional)
    #[arg(long)]
    pub part_number: Option<String>,

    #[arg(long)]
    pub rear_image: Option<String>,

    #[arg(long)]
    pub slug: String,

    /// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
    ///
    /// * `parent` - Parent
    /// * `child` - Child
    #[arg(long)]
    pub subdevice_role: Option<DeviceTypeSubdeviceRole>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub u_height: Option<f64>,

    #[arg(long)]
    pub weight: Option<f64>,

    /// * `kg` - Kilograms
    /// * `g` - Grams
    /// * `lb` - Pounds
    /// * `oz` - Ounces
    #[arg(long)]
    pub weight_unit: Option<DeviceTypeWeightUnit>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct DeviceTypeUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    /// * `front-to-rear` - Front to rear
    /// * `rear-to-front` - Rear to front
    /// * `left-to-right` - Left to right
    /// * `right-to-left` - Right to left
    /// * `side-to-rear` - Side to rear
    /// * `rear-to-side` - Rear to side
    /// * `bottom-to-top` - Bottom to top
    /// * `top-to-bottom` - Top to bottom
    /// * `passive` - Passive
    /// * `mixed` - Mixed
    #[arg(long)]
    pub airflow: Option<DeviceTypeAirflow>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub default_platform: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// Devices of this type are excluded when calculating rack utilization.
    #[arg(long)]
    pub exclude_from_utilization: Option<bool>,

    #[arg(long)]
    pub front_image: Option<String>,

    /// Device consumes both front and rear rack faces.
    #[arg(long)]
    pub is_full_depth: Option<bool>,

    #[arg(long)]
    pub manufacturer: Option<String>,

    #[arg(long)]
    pub model: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// Discrete part number (optional)
    #[arg(long)]
    pub part_number: Option<String>,

    #[arg(long)]
    pub rear_image: Option<String>,

    #[arg(long)]
    pub slug: Option<String>,

    /// Parent devices house child devices in device bays. Leave blank if this device type is neither a parent nor a child.
    ///
    /// * `parent` - Parent
    /// * `child` - Child
    #[arg(long)]
    pub subdevice_role: Option<DeviceTypeSubdeviceRole>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub u_height: Option<f64>,

    #[arg(long)]
    pub weight: Option<f64>,

    /// * `kg` - Kilograms
    /// * `g` - Grams
    /// * `lb` - Pounds
    /// * `oz` - Ounces
    #[arg(long)]
    pub weight_unit: Option<DeviceTypeWeightUnit>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum DeviceTypeAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(DeviceTypeCreateArgs),
    Update(DeviceTypeUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_device_type_action(action: DeviceTypeAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        DeviceTypeAction::List(args) => generic_list(RESOURCE, args, opts).await,
        DeviceTypeAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        DeviceTypeAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        DeviceTypeAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        DeviceTypeAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        DeviceTypeAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        DeviceTypeAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: DeviceTypeCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    if let Some(v) = args.airflow {
        object.insert("airflow".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "comments", args.comments);
    if let Some(v) = args.default_platform {
        object.insert("default_platform".to_owned(), crate::commands::resolve_reference_id("/api/dcim/platforms/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "description", args.description);
    insert_optional_bool_field(object, "exclude_from_utilization", args.exclude_from_utilization);
    insert_optional_string_field(object, "front_image", args.front_image);
    insert_optional_bool_field(object, "is_full_depth", args.is_full_depth);
    object.insert("manufacturer".to_owned(), crate::commands::resolve_reference_id("/api/dcim/manufacturers/", "slug", &args.manufacturer, opts).await?);
    object.insert("model".to_owned(), Value::String(args.model));
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "part_number", args.part_number);
    insert_optional_string_field(object, "rear_image", args.rear_image);
    object.insert("slug".to_owned(), Value::String(args.slug));
    if let Some(v) = args.subdevice_role {
        object.insert("subdevice_role".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.u_height {
        object.insert("u_height".to_owned(), json!(v));
    }
    if let Some(v) = args.weight {
        object.insert("weight".to_owned(), json!(v));
    }
    if let Some(v) = args.weight_unit {
        object.insert("weight_unit".to_owned(), json!(v));
    }
    Ok(body)
}

async fn update_body(args: DeviceTypeUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    if let Some(v) = args.airflow {
        object.insert("airflow".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "comments", args.comments);
    if let Some(v) = args.default_platform {
        object.insert("default_platform".to_owned(), crate::commands::resolve_reference_id("/api/dcim/platforms/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "description", args.description);
    insert_optional_bool_field(object, "exclude_from_utilization", args.exclude_from_utilization);
    insert_optional_string_field(object, "front_image", args.front_image);
    insert_optional_bool_field(object, "is_full_depth", args.is_full_depth);
    if let Some(v) = args.manufacturer {
        object.insert("manufacturer".to_owned(), crate::commands::resolve_reference_id("/api/dcim/manufacturers/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "model", args.model);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "part_number", args.part_number);
    insert_optional_string_field(object, "rear_image", args.rear_image);
    insert_optional_string_field(object, "slug", args.slug);
    if let Some(v) = args.subdevice_role {
        object.insert("subdevice_role".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.u_height {
        object.insert("u_height".to_owned(), json!(v));
    }
    if let Some(v) = args.weight {
        object.insert("weight".to_owned(), json!(v));
    }
    if let Some(v) = args.weight_unit {
        object.insert("weight_unit".to_owned(), json!(v));
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::DeviceType>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::DeviceType>>(value.clone()).map(|_| ())
}

