// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: WritableCableRequest
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
    app: "dcim",
    name: "cables",
    api_path: "/api/dcim/cables/",
    detail_path: "/api/dcim/cables/{id}/",
    default_lookup_field: "label",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CableLengthUnit {
    Km,
    M,
    Cm,
    Mi,
    Ft,
    In,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CableProfile {
    #[serde(rename = "single-1c1p")]
    #[value(name = "single-1c1p")]
    Single1c1p,
    #[serde(rename = "single-1c2p")]
    #[value(name = "single-1c2p")]
    Single1c2p,
    #[serde(rename = "single-1c4p")]
    #[value(name = "single-1c4p")]
    Single1c4p,
    #[serde(rename = "single-1c6p")]
    #[value(name = "single-1c6p")]
    Single1c6p,
    #[serde(rename = "single-1c8p")]
    #[value(name = "single-1c8p")]
    Single1c8p,
    #[serde(rename = "single-1c12p")]
    #[value(name = "single-1c12p")]
    Single1c12p,
    #[serde(rename = "single-1c16p")]
    #[value(name = "single-1c16p")]
    Single1c16p,
    #[serde(rename = "trunk-2c1p")]
    #[value(name = "trunk-2c1p")]
    Trunk2c1p,
    #[serde(rename = "trunk-2c2p")]
    #[value(name = "trunk-2c2p")]
    Trunk2c2p,
    #[serde(rename = "trunk-2c4p")]
    #[value(name = "trunk-2c4p")]
    Trunk2c4p,
    #[serde(rename = "trunk-2c4p-shuffle")]
    #[value(name = "trunk-2c4p-shuffle")]
    Trunk2c4pShuffle,
    #[serde(rename = "trunk-2c6p")]
    #[value(name = "trunk-2c6p")]
    Trunk2c6p,
    #[serde(rename = "trunk-2c8p")]
    #[value(name = "trunk-2c8p")]
    Trunk2c8p,
    #[serde(rename = "trunk-2c12p")]
    #[value(name = "trunk-2c12p")]
    Trunk2c12p,
    #[serde(rename = "trunk-4c1p")]
    #[value(name = "trunk-4c1p")]
    Trunk4c1p,
    #[serde(rename = "trunk-4c2p")]
    #[value(name = "trunk-4c2p")]
    Trunk4c2p,
    #[serde(rename = "trunk-4c4p")]
    #[value(name = "trunk-4c4p")]
    Trunk4c4p,
    #[serde(rename = "trunk-4c4p-shuffle")]
    #[value(name = "trunk-4c4p-shuffle")]
    Trunk4c4pShuffle,
    #[serde(rename = "trunk-4c6p")]
    #[value(name = "trunk-4c6p")]
    Trunk4c6p,
    #[serde(rename = "trunk-4c8p")]
    #[value(name = "trunk-4c8p")]
    Trunk4c8p,
    #[serde(rename = "trunk-8c4p")]
    #[value(name = "trunk-8c4p")]
    Trunk8c4p,
    #[serde(rename = "breakout-1c2p-2c1p")]
    #[value(name = "breakout-1c2p-2c1p")]
    Breakout1c2p2c1p,
    #[serde(rename = "breakout-1c4p-4c1p")]
    #[value(name = "breakout-1c4p-4c1p")]
    Breakout1c4p4c1p,
    #[serde(rename = "breakout-1c6p-6c1p")]
    #[value(name = "breakout-1c6p-6c1p")]
    Breakout1c6p6c1p,
    #[serde(rename = "breakout-2c4p-8c1p-shuffle")]
    #[value(name = "breakout-2c4p-8c1p-shuffle")]
    Breakout2c4p8c1pShuffle,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CableStatus {
    Connected,
    Planned,
    Decommissioning,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CableType {
    Cat3,
    Cat5,
    Cat5e,
    Cat6,
    Cat6a,
    Cat7,
    Cat7a,
    Cat8,
    #[serde(rename = "mrj21-trunk")]
    #[value(name = "mrj21-trunk")]
    Mrj21Trunk,
    #[serde(rename = "dac-active")]
    #[value(name = "dac-active")]
    DacActive,
    #[serde(rename = "dac-passive")]
    #[value(name = "dac-passive")]
    DacPassive,
    Coaxial,
    #[serde(rename = "rg-6")]
    #[value(name = "rg-6")]
    Rg6,
    #[serde(rename = "rg-8")]
    #[value(name = "rg-8")]
    Rg8,
    #[serde(rename = "rg-11")]
    #[value(name = "rg-11")]
    Rg11,
    #[serde(rename = "rg-59")]
    #[value(name = "rg-59")]
    Rg59,
    #[serde(rename = "rg-62")]
    #[value(name = "rg-62")]
    Rg62,
    #[serde(rename = "rg-213")]
    #[value(name = "rg-213")]
    Rg213,
    #[serde(rename = "lmr-100")]
    #[value(name = "lmr-100")]
    Lmr100,
    #[serde(rename = "lmr-200")]
    #[value(name = "lmr-200")]
    Lmr200,
    #[serde(rename = "lmr-400")]
    #[value(name = "lmr-400")]
    Lmr400,
    Mmf,
    #[serde(rename = "mmf-om1")]
    #[value(name = "mmf-om1")]
    MmfOm1,
    #[serde(rename = "mmf-om2")]
    #[value(name = "mmf-om2")]
    MmfOm2,
    #[serde(rename = "mmf-om3")]
    #[value(name = "mmf-om3")]
    MmfOm3,
    #[serde(rename = "mmf-om4")]
    #[value(name = "mmf-om4")]
    MmfOm4,
    #[serde(rename = "mmf-om5")]
    #[value(name = "mmf-om5")]
    MmfOm5,
    Smf,
    #[serde(rename = "smf-os1")]
    #[value(name = "smf-os1")]
    SmfOs1,
    #[serde(rename = "smf-os2")]
    #[value(name = "smf-os2")]
    SmfOs2,
    Aoc,
    Power,
    Usb,
}

#[derive(Debug, Args)]
pub struct CableCreateArgs {
    #[arg(long)]
    pub color: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub label: Option<String>,

    #[arg(long)]
    pub length: Option<f64>,

    /// * `km` - Kilometers
    /// * `m` - Meters
    /// * `cm` - Centimeters
    /// * `mi` - Miles
    /// * `ft` - Feet
    /// * `in` - Inches
    #[arg(long)]
    pub length_unit: Option<CableLengthUnit>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// * `single-1c1p` - 1C1P
    /// * `single-1c2p` - 1C2P
    /// * `single-1c4p` - 1C4P
    /// * `single-1c6p` - 1C6P
    /// * `single-1c8p` - 1C8P
    /// * `single-1c12p` - 1C12P
    /// * `single-1c16p` - 1C16P
    /// * `trunk-2c1p` - 2C1P trunk
    /// * `trunk-2c2p` - 2C2P trunk
    /// * `trunk-2c4p` - 2C4P trunk
    /// * `trunk-2c4p-shuffle` - 2C4P trunk (shuffle)
    /// * `trunk-2c6p` - 2C6P trunk
    /// * `trunk-2c8p` - 2C8P trunk
    /// * `trunk-2c12p` - 2C12P trunk
    /// * `trunk-4c1p` - 4C1P trunk
    /// * `trunk-4c2p` - 4C2P trunk
    /// * `trunk-4c4p` - 4C4P trunk
    /// * `trunk-4c4p-shuffle` - 4C4P trunk (shuffle)
    /// * `trunk-4c6p` - 4C6P trunk
    /// * `trunk-4c8p` - 4C8P trunk
    /// * `trunk-8c4p` - 8C4P trunk
    /// * `breakout-1c2p-2c1p` - 1C2P:2C1P breakout
    /// * `breakout-1c4p-4c1p` - 1C4P:4C1P breakout
    /// * `breakout-1c6p-6c1p` - 1C6P:6C1P breakout
    /// * `breakout-2c4p-8c1p-shuffle` - 2C4P:8C1P breakout (shuffle)
    #[arg(long)]
    pub profile: Option<CableProfile>,

    /// * `connected` - Connected
    /// * `planned` - Planned
    /// * `decommissioning` - Decommissioning
    #[arg(long)]
    pub status: Option<CableStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// * `cat3` - CAT3
    /// * `cat5` - CAT5
    /// * `cat5e` - CAT5e
    /// * `cat6` - CAT6
    /// * `cat6a` - CAT6a
    /// * `cat7` - CAT7
    /// * `cat7a` - CAT7a
    /// * `cat8` - CAT8
    /// * `mrj21-trunk` - MRJ21 Trunk
    /// * `dac-active` - Direct Attach Copper (Active)
    /// * `dac-passive` - Direct Attach Copper (Passive)
    /// * `coaxial` - Coaxial
    /// * `rg-6` - RG-6
    /// * `rg-8` - RG-8
    /// * `rg-11` - RG-11
    /// * `rg-59` - RG-59
    /// * `rg-62` - RG-62
    /// * `rg-213` - RG-213
    /// * `lmr-100` - LMR-100
    /// * `lmr-200` - LMR-200
    /// * `lmr-400` - LMR-400
    /// * `mmf` - Multimode Fiber
    /// * `mmf-om1` - Multimode Fiber (OM1)
    /// * `mmf-om2` - Multimode Fiber (OM2)
    /// * `mmf-om3` - Multimode Fiber (OM3)
    /// * `mmf-om4` - Multimode Fiber (OM4)
    /// * `mmf-om5` - Multimode Fiber (OM5)
    /// * `smf` - Single-mode Fiber
    /// * `smf-os1` - Single-mode Fiber (OS1)
    /// * `smf-os2` - Single-mode Fiber (OS2)
    /// * `aoc` - Active Optical Cabling (AOC)
    /// * `power` - Power
    /// * `usb` - USB
    #[arg(long = "type")]
    pub r#type: Option<CableType>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Args)]
pub struct CableUpdateArgs {
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
    pub description: Option<String>,

    #[arg(long)]
    pub label: Option<String>,

    #[arg(long)]
    pub length: Option<f64>,

    /// * `km` - Kilometers
    /// * `m` - Meters
    /// * `cm` - Centimeters
    /// * `mi` - Miles
    /// * `ft` - Feet
    /// * `in` - Inches
    #[arg(long)]
    pub length_unit: Option<CableLengthUnit>,

    #[arg(long)]
    pub owner: Option<u64>,

    /// * `single-1c1p` - 1C1P
    /// * `single-1c2p` - 1C2P
    /// * `single-1c4p` - 1C4P
    /// * `single-1c6p` - 1C6P
    /// * `single-1c8p` - 1C8P
    /// * `single-1c12p` - 1C12P
    /// * `single-1c16p` - 1C16P
    /// * `trunk-2c1p` - 2C1P trunk
    /// * `trunk-2c2p` - 2C2P trunk
    /// * `trunk-2c4p` - 2C4P trunk
    /// * `trunk-2c4p-shuffle` - 2C4P trunk (shuffle)
    /// * `trunk-2c6p` - 2C6P trunk
    /// * `trunk-2c8p` - 2C8P trunk
    /// * `trunk-2c12p` - 2C12P trunk
    /// * `trunk-4c1p` - 4C1P trunk
    /// * `trunk-4c2p` - 4C2P trunk
    /// * `trunk-4c4p` - 4C4P trunk
    /// * `trunk-4c4p-shuffle` - 4C4P trunk (shuffle)
    /// * `trunk-4c6p` - 4C6P trunk
    /// * `trunk-4c8p` - 4C8P trunk
    /// * `trunk-8c4p` - 8C4P trunk
    /// * `breakout-1c2p-2c1p` - 1C2P:2C1P breakout
    /// * `breakout-1c4p-4c1p` - 1C4P:4C1P breakout
    /// * `breakout-1c6p-6c1p` - 1C6P:6C1P breakout
    /// * `breakout-2c4p-8c1p-shuffle` - 2C4P:8C1P breakout (shuffle)
    #[arg(long)]
    pub profile: Option<CableProfile>,

    /// * `connected` - Connected
    /// * `planned` - Planned
    /// * `decommissioning` - Decommissioning
    #[arg(long)]
    pub status: Option<CableStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    /// * `cat3` - CAT3
    /// * `cat5` - CAT5
    /// * `cat5e` - CAT5e
    /// * `cat6` - CAT6
    /// * `cat6a` - CAT6a
    /// * `cat7` - CAT7
    /// * `cat7a` - CAT7a
    /// * `cat8` - CAT8
    /// * `mrj21-trunk` - MRJ21 Trunk
    /// * `dac-active` - Direct Attach Copper (Active)
    /// * `dac-passive` - Direct Attach Copper (Passive)
    /// * `coaxial` - Coaxial
    /// * `rg-6` - RG-6
    /// * `rg-8` - RG-8
    /// * `rg-11` - RG-11
    /// * `rg-59` - RG-59
    /// * `rg-62` - RG-62
    /// * `rg-213` - RG-213
    /// * `lmr-100` - LMR-100
    /// * `lmr-200` - LMR-200
    /// * `lmr-400` - LMR-400
    /// * `mmf` - Multimode Fiber
    /// * `mmf-om1` - Multimode Fiber (OM1)
    /// * `mmf-om2` - Multimode Fiber (OM2)
    /// * `mmf-om3` - Multimode Fiber (OM3)
    /// * `mmf-om4` - Multimode Fiber (OM4)
    /// * `mmf-om5` - Multimode Fiber (OM5)
    /// * `smf` - Single-mode Fiber
    /// * `smf-os1` - Single-mode Fiber (OS1)
    /// * `smf-os2` - Single-mode Fiber (OS2)
    /// * `aoc` - Active Optical Cabling (AOC)
    /// * `power` - Power
    /// * `usb` - USB
    #[arg(long = "type")]
    pub r#type: Option<CableType>,

    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.
    #[arg(long, conflicts_with = "data_file")]
    pub data: Option<String>,

    #[arg(long, value_name = "PATH")]
    pub data_file: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum CableAction {
    List(GenericListArgs),
    Get(GetArgs),
    Create(CableCreateArgs),
    Update(CableUpdateArgs),
    Delete(DeleteArgs),
    #[command(name = "bulk-update")]
    BulkUpdate(GenericMutationArgs),
    #[command(name = "bulk-delete")]
    BulkDelete(BulkDeleteArgs),
}

pub async fn run_cable_action(action: CableAction, opts: &GlobalOptions) -> NbxResult<()> {
    match action {
        CableAction::List(args) => generic_list(RESOURCE, args, opts).await,
        CableAction::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,
        CableAction::Create(args) => {
            let body = create_body(args, opts).await?;
            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await
        }
        CableAction::Update(args) => {
            let id = args.id_or_lookup.clone();
            let lookup = args.lookup_field.clone();
            let body = update_body(args, opts).await?;
            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await
        }
        CableAction::Delete(args) => delete_resource(RESOURCE, args, opts).await,
        CableAction::BulkUpdate(args) => {
            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await
        }
        CableAction::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,
    }
}

async fn create_body(args: CableCreateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "color", args.color);
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "label", args.label);
    if let Some(v) = args.length {
        object.insert("length".to_owned(), json!(v));
    }
    if let Some(v) = args.length_unit {
        object.insert("length_unit".to_owned(), json!(v));
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.profile {
        object.insert("profile".to_owned(), json!(v));
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
    if let Some(v) = args.r#type {
        object.insert("type".to_owned(), json!(v));
    }
    Ok(body)
}

async fn update_body(args: CableUpdateArgs, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = optional_json_body(args.data, args.data_file)?
        .unwrap_or_else(|| json!({}));
    let object: &mut Map<String, Value> = body.as_object_mut()
        .ok_or_else(|| NbxError::validation("mutation payload must be a JSON object", json!({})))?;
    insert_optional_string_field(object, "color", args.color);
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_string_field(object, "label", args.label);
    if let Some(v) = args.length {
        object.insert("length".to_owned(), json!(v));
    }
    if let Some(v) = args.length_unit {
        object.insert("length_unit".to_owned(), json!(v));
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.profile {
        object.insert("profile".to_owned(), json!(v));
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
    if let Some(v) = args.r#type {
        object.insert("type".to_owned(), json!(v));
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::Cable>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Cable>>(value.clone()).map(|_| ())
}

