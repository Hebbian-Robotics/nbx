// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: WritableRackRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.

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
    name: "racks",
    api_path: "/api/dcim/racks/",
    detail_path: "/api/dcim/racks/{id}/",
    default_lookup_field: "name",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RackAirflow {
    #[serde(rename = "front-to-rear")]
    #[value(name = "front-to-rear")]
    FrontToRear,
    #[serde(rename = "rear-to-front")]
    #[value(name = "rear-to-front")]
    RearToFront,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RackFormFactor {
    #[serde(rename = "2-post-frame")]
    #[value(name = "2-post-frame")]
    TwoPostFrame,
    #[serde(rename = "4-post-frame")]
    #[value(name = "4-post-frame")]
    FourPostFrame,
    #[serde(rename = "4-post-cabinet")]
    #[value(name = "4-post-cabinet")]
    FourPostCabinet,
    #[serde(rename = "wall-frame")]
    #[value(name = "wall-frame")]
    WallFrame,
    #[serde(rename = "wall-frame-vertical")]
    #[value(name = "wall-frame-vertical")]
    WallFrameVertical,
    #[serde(rename = "wall-cabinet")]
    #[value(name = "wall-cabinet")]
    WallCabinet,
    #[serde(rename = "wall-cabinet-vertical")]
    #[value(name = "wall-cabinet-vertical")]
    WallCabinetVertical,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RackOuterUnit {
    Mm,
    In,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RackStatus {
    Reserved,
    Available,
    Planned,
    Active,
    Deprecated,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RackWeightUnit {
    Kg,
    G,
    Lb,
    Oz,
}

#[derive(Debug, Args)]
pub struct RackCreateArgs {
    /// * `front-to-rear` - Front to rear
    /// * `rear-to-front` - Rear to front
    #[arg(long)]
    pub airflow: Option<RackAirflow>,

    /// A unique tag used to identify this rack
    #[arg(long)]
    pub asset_tag: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    /// Units are numbered top-to-bottom
    #[arg(long)]
    pub desc_units: Option<bool>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub facility_id: Option<String>,

    /// * `2-post-frame` - 2-post frame
    /// * `4-post-frame` - 4-post frame
    /// * `4-post-cabinet` - 4-post cabinet
    /// * `wall-frame` - Wall-mounted frame
    /// * `wall-frame-vertical` - Wall-mounted frame (vertical)
    /// * `wall-cabinet` - Wall-mounted cabinet
    /// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
    #[arg(long)]
    pub form_factor: Option<RackFormFactor>,

    #[arg(long)]
    pub location: Option<String>,

    /// Maximum load capacity for the rack
    #[arg(long)]
    pub max_weight: Option<u64>,

    /// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
    #[arg(long)]
    pub mounting_depth: Option<u64>,

    #[arg(long)]
    pub name: String,

    /// Outer dimension of rack (depth)
    #[arg(long)]
    pub outer_depth: Option<u64>,

    /// Outer dimension of rack (height)
    #[arg(long)]
    pub outer_height: Option<u64>,

    /// * `mm` - Millimeters
    /// * `in` - Inches
    #[arg(long)]
    pub outer_unit: Option<RackOuterUnit>,

    /// Outer dimension of rack (width)
    #[arg(long)]
    pub outer_width: Option<u64>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub rack_type: Option<String>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub serial: Option<String>,

    #[arg(long)]
    pub site: String,

    /// Starting unit for rack
    #[arg(long)]
    pub starting_unit: Option<u64>,

    /// * `reserved` - Reserved
    /// * `available` - Available
    /// * `planned` - Planned
    /// * `active` - Active
    /// * `deprecated` - Deprecated
    #[arg(long)]
    pub status: Option<RackStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// Height in rack units
    #[arg(long)]
    pub u_height: Option<u64>,

    #[arg(long)]
    pub weight: Option<f64>,

    /// * `kg` - Kilograms
    /// * `g` - Grams
    /// * `lb` - Pounds
    /// * `oz` - Ounces
    #[arg(long)]
    pub weight_unit: Option<RackWeightUnit>,

    /// Rail-to-rail width
    ///
    /// * `10` - 10 inches
    /// * `19` - 19 inches
    /// * `21` - 21 inches
    /// * `23` - 23 inches
    #[arg(long)]
    pub width: Option<u64>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct RackUpdateArgs {
    /// Numeric NetBox ID or default-lookup-field value.
    pub id_or_lookup: String,

    /// Override the default lookup field.
    #[arg(long)]
    pub lookup_field: Option<String>,

    /// * `front-to-rear` - Front to rear
    /// * `rear-to-front` - Rear to front
    #[arg(long)]
    pub airflow: Option<RackAirflow>,

    /// A unique tag used to identify this rack
    #[arg(long)]
    pub asset_tag: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    /// Units are numbered top-to-bottom
    #[arg(long)]
    pub desc_units: Option<bool>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub facility_id: Option<String>,

    /// * `2-post-frame` - 2-post frame
    /// * `4-post-frame` - 4-post frame
    /// * `4-post-cabinet` - 4-post cabinet
    /// * `wall-frame` - Wall-mounted frame
    /// * `wall-frame-vertical` - Wall-mounted frame (vertical)
    /// * `wall-cabinet` - Wall-mounted cabinet
    /// * `wall-cabinet-vertical` - Wall-mounted cabinet (vertical)
    #[arg(long)]
    pub form_factor: Option<RackFormFactor>,

    #[arg(long)]
    pub location: Option<String>,

    /// Maximum load capacity for the rack
    #[arg(long)]
    pub max_weight: Option<u64>,

    /// Maximum depth of a mounted device, in millimeters. For four-post racks, this is the distance between the front and rear rails.
    #[arg(long)]
    pub mounting_depth: Option<u64>,

    #[arg(long)]
    pub name: Option<String>,

    /// Outer dimension of rack (depth)
    #[arg(long)]
    pub outer_depth: Option<u64>,

    /// Outer dimension of rack (height)
    #[arg(long)]
    pub outer_height: Option<u64>,

    /// * `mm` - Millimeters
    /// * `in` - Inches
    #[arg(long)]
    pub outer_unit: Option<RackOuterUnit>,

    /// Outer dimension of rack (width)
    #[arg(long)]
    pub outer_width: Option<u64>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub rack_type: Option<String>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub serial: Option<String>,

    #[arg(long)]
    pub site: Option<String>,

    /// Starting unit for rack
    #[arg(long)]
    pub starting_unit: Option<u64>,

    /// * `reserved` - Reserved
    /// * `available` - Available
    /// * `planned` - Planned
    /// * `active` - Active
    /// * `deprecated` - Deprecated
    #[arg(long)]
    pub status: Option<RackStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// Height in rack units
    #[arg(long)]
    pub u_height: Option<u64>,

    #[arg(long)]
    pub weight: Option<f64>,

    /// * `kg` - Kilograms
    /// * `g` - Grams
    /// * `lb` - Pounds
    /// * `oz` - Ounces
    #[arg(long)]
    pub weight_unit: Option<RackWeightUnit>,

    /// Rail-to-rail width
    ///
    /// * `10` - 10 inches
    /// * `19` - 19 inches
    /// * `21` - 21 inches
    /// * `23` - 23 inches
    #[arg(long)]
    pub width: Option<u64>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum RackAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(RackCreateArgs),
    Update(RackUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_rack_action(action: RackAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        RackAction::List(args) => generic_list(RESOURCE, args, opts).await,
        RackAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        RackAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        RackAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        RackAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        RackAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        RackAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: RackCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    if let Some(v) = args.airflow {
        object.insert("airflow".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "asset_tag", args.asset_tag);
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_bool_field(object, "desc_units", args.desc_units);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "facility_id", args.facility_id);
    if let Some(v) = args.form_factor {
        object.insert("form_factor".to_owned(), json!(v));
    }
    if let Some(v) = args.location {
        object.insert("location".to_owned(), crate::commands::resolve_reference_id("/api/dcim/locations/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.max_weight {
        object.insert("max_weight".to_owned(), json!(v));
    }
    if let Some(v) = args.mounting_depth {
        object.insert("mounting_depth".to_owned(), json!(v));
    }
    object.insert("name".to_owned(), Value::String(args.name));
    if let Some(v) = args.outer_depth {
        object.insert("outer_depth".to_owned(), json!(v));
    }
    if let Some(v) = args.outer_height {
        object.insert("outer_height".to_owned(), json!(v));
    }
    if let Some(v) = args.outer_unit {
        object.insert("outer_unit".to_owned(), json!(v));
    }
    if let Some(v) = args.outer_width {
        object.insert("outer_width".to_owned(), json!(v));
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.rack_type {
        object.insert("rack_type".to_owned(), crate::commands::resolve_reference_id("/api/dcim/rack-types/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/dcim/rack-roles/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "serial", args.serial);
    object.insert("site".to_owned(), crate::commands::resolve_reference_id("/api/dcim/sites/", "slug", &args.site, opts).await?);
    if let Some(v) = args.starting_unit {
        object.insert("starting_unit".to_owned(), json!(v));
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
    if let Some(v) = args.u_height {
        object.insert("u_height".to_owned(), json!(v));
    }
    if let Some(v) = args.weight {
        object.insert("weight".to_owned(), json!(v));
    }
    if let Some(v) = args.weight_unit {
        object.insert("weight_unit".to_owned(), json!(v));
    }
    if let Some(v) = args.width {
        object.insert("width".to_owned(), json!(v));
    }
    Ok(body)
}

async fn update_body(args: RackUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    if let Some(v) = args.airflow {
        object.insert("airflow".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "asset_tag", args.asset_tag);
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_bool_field(object, "desc_units", args.desc_units);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "facility_id", args.facility_id);
    if let Some(v) = args.form_factor {
        object.insert("form_factor".to_owned(), json!(v));
    }
    if let Some(v) = args.location {
        object.insert("location".to_owned(), crate::commands::resolve_reference_id("/api/dcim/locations/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.max_weight {
        object.insert("max_weight".to_owned(), json!(v));
    }
    if let Some(v) = args.mounting_depth {
        object.insert("mounting_depth".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.outer_depth {
        object.insert("outer_depth".to_owned(), json!(v));
    }
    if let Some(v) = args.outer_height {
        object.insert("outer_height".to_owned(), json!(v));
    }
    if let Some(v) = args.outer_unit {
        object.insert("outer_unit".to_owned(), json!(v));
    }
    if let Some(v) = args.outer_width {
        object.insert("outer_width".to_owned(), json!(v));
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.rack_type {
        object.insert("rack_type".to_owned(), crate::commands::resolve_reference_id("/api/dcim/rack-types/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/dcim/rack-roles/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "serial", args.serial);
    if let Some(v) = args.site {
        object.insert("site".to_owned(), crate::commands::resolve_reference_id("/api/dcim/sites/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.starting_unit {
        object.insert("starting_unit".to_owned(), json!(v));
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
    if let Some(v) = args.u_height {
        object.insert("u_height".to_owned(), json!(v));
    }
    if let Some(v) = args.weight {
        object.insert("weight".to_owned(), json!(v));
    }
    if let Some(v) = args.weight_unit {
        object.insert("weight_unit".to_owned(), json!(v));
    }
    if let Some(v) = args.width {
        object.insert("width".to_owned(), json!(v));
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::Rack>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Rack>>(value.clone()).map(|_| ())
}

