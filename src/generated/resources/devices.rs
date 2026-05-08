// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: WritableDeviceWithConfigContextRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.

use clap::Args;
use clap::ValueEnum;
use serde_json::{Map, Value, json};

use crate::commands::{GlobalOptions, ResourceSpec};
use crate::commands::tags_value;
use crate::commands::insert_optional_string_field;
use crate::error::NbxResult;

pub const RESOURCE: ResourceSpec = ResourceSpec {
    app: "dcim",
    name: "devices",
    api_path: "/api/dcim/devices/",
    detail_path: "/api/dcim/devices/{id}/",
    default_lookup_field: "name",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceAirflow {
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
pub enum DeviceFace {
    Front,
    Rear,
}

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceStatus {
    Offline,
    Active,
    Planned,
    Staged,
    Failed,
    Inventory,
    Decommissioning,
}

#[derive(Debug, Args)]
pub struct DeviceCreateFields {
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
    pub airflow: Option<DeviceAirflow>,

    /// A unique tag used to identify this device
    #[arg(long)]
    pub asset_tag: Option<String>,

    #[arg(long)]
    pub cluster: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub config_template: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub device_type: String,

    /// * `front` - Front
    /// * `rear` - Rear
    #[arg(long)]
    pub face: Option<DeviceFace>,

    /// GPS coordinate in decimal format (xx.yyyyyy)
    #[arg(long)]
    pub latitude: Option<f64>,

    #[arg(long)]
    pub location: Option<String>,

    /// GPS coordinate in decimal format (xx.yyyyyy)
    #[arg(long)]
    pub longitude: Option<f64>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub oob_ip: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub platform: Option<String>,

    #[arg(long)]
    pub position: Option<f64>,

    #[arg(long)]
    pub primary_ip4: Option<String>,

    #[arg(long)]
    pub primary_ip6: Option<String>,

    #[arg(long)]
    pub rack: Option<String>,

    #[arg(long)]
    pub role: String,

    /// Chassis serial number, assigned by the manufacturer
    #[arg(long)]
    pub serial: Option<String>,

    #[arg(long)]
    pub site: String,

    /// * `offline` - Offline
    /// * `active` - Active
    /// * `planned` - Planned
    /// * `staged` - Staged
    /// * `failed` - Failed
    /// * `inventory` - Inventory
    /// * `decommissioning` - Decommissioning
    #[arg(long)]
    pub status: Option<DeviceStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    #[arg(long)]
    pub vc_position: Option<u64>,

    /// Virtual chassis master election priority
    #[arg(long)]
    pub vc_priority: Option<u64>,

    #[arg(long)]
    pub virtual_chassis: Option<String>,

}

#[derive(Debug, Args)]
pub struct DeviceUpdateFields {
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
    pub airflow: Option<DeviceAirflow>,

    /// A unique tag used to identify this device
    #[arg(long)]
    pub asset_tag: Option<String>,

    #[arg(long)]
    pub cluster: Option<String>,

    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub config_template: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub device_type: Option<String>,

    /// * `front` - Front
    /// * `rear` - Rear
    #[arg(long)]
    pub face: Option<DeviceFace>,

    /// GPS coordinate in decimal format (xx.yyyyyy)
    #[arg(long)]
    pub latitude: Option<f64>,

    #[arg(long)]
    pub location: Option<String>,

    /// GPS coordinate in decimal format (xx.yyyyyy)
    #[arg(long)]
    pub longitude: Option<f64>,

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub oob_ip: Option<String>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub platform: Option<String>,

    #[arg(long)]
    pub position: Option<f64>,

    #[arg(long)]
    pub primary_ip4: Option<String>,

    #[arg(long)]
    pub primary_ip6: Option<String>,

    #[arg(long)]
    pub rack: Option<String>,

    #[arg(long)]
    pub role: Option<String>,

    /// Chassis serial number, assigned by the manufacturer
    #[arg(long)]
    pub serial: Option<String>,

    #[arg(long)]
    pub site: Option<String>,

    /// * `offline` - Offline
    /// * `active` - Active
    /// * `planned` - Planned
    /// * `staged` - Staged
    /// * `failed` - Failed
    /// * `inventory` - Inventory
    /// * `decommissioning` - Decommissioning
    #[arg(long)]
    pub status: Option<DeviceStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    #[arg(long)]
    pub vc_position: Option<u64>,

    /// Virtual chassis master election priority
    #[arg(long)]
    pub vc_priority: Option<u64>,

    #[arg(long)]
    pub virtual_chassis: Option<String>,

}

pub async fn create_body(args: DeviceCreateFields, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = json!({});
    let object: &mut Map<String, Value> = body.as_object_mut()
        .expect("json!({}) always returns an object");
    if let Some(v) = args.airflow {
        object.insert("airflow".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "asset_tag", args.asset_tag);
    if let Some(v) = args.cluster {
        object.insert("cluster".to_owned(), crate::commands::resolve_reference_id("/api/virtualization/clusters/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "comments", args.comments);
    if let Some(v) = args.config_template {
        object.insert("config_template".to_owned(), crate::commands::resolve_reference_id("/api/extras/config-templates/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "description", args.description);
    object.insert("device_type".to_owned(), crate::commands::resolve_reference_id("/api/dcim/device-types/", "slug", &args.device_type, opts).await?);
    if let Some(v) = args.face {
        object.insert("face".to_owned(), json!(v));
    }
    if let Some(v) = args.latitude {
        object.insert("latitude".to_owned(), json!(v));
    }
    if let Some(v) = args.location {
        object.insert("location".to_owned(), crate::commands::resolve_reference_id("/api/dcim/locations/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.longitude {
        object.insert("longitude".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.oob_ip {
        object.insert("oob_ip".to_owned(), crate::commands::resolve_reference_id("/api/ipam/ip-addresses/", "address", &v, opts).await?);
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.platform {
        object.insert("platform".to_owned(), crate::commands::resolve_reference_id("/api/dcim/platforms/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.position {
        object.insert("position".to_owned(), json!(v));
    }
    if let Some(v) = args.primary_ip4 {
        object.insert("primary_ip4".to_owned(), crate::commands::resolve_reference_id("/api/ipam/ip-addresses/", "address", &v, opts).await?);
    }
    if let Some(v) = args.primary_ip6 {
        object.insert("primary_ip6".to_owned(), crate::commands::resolve_reference_id("/api/ipam/ip-addresses/", "address", &v, opts).await?);
    }
    if let Some(v) = args.rack {
        object.insert("rack".to_owned(), crate::commands::resolve_reference_id("/api/dcim/racks/", "name", &v, opts).await?);
    }
    object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/dcim/device-roles/", "slug", &args.role, opts).await?);
    insert_optional_string_field(object, "serial", args.serial);
    object.insert("site".to_owned(), crate::commands::resolve_reference_id("/api/dcim/sites/", "slug", &args.site, opts).await?);
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.vc_position {
        object.insert("vc_position".to_owned(), json!(v));
    }
    if let Some(v) = args.vc_priority {
        object.insert("vc_priority".to_owned(), json!(v));
    }
    if let Some(v) = args.virtual_chassis {
        object.insert("virtual_chassis".to_owned(), crate::commands::resolve_reference_id("/api/dcim/virtual-chassis/", "name", &v, opts).await?);
    }
    Ok(body)
}

pub async fn update_body(args: DeviceUpdateFields, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = json!({});
    let object: &mut Map<String, Value> = body.as_object_mut()
        .expect("json!({}) always returns an object");
    if let Some(v) = args.airflow {
        object.insert("airflow".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "asset_tag", args.asset_tag);
    if let Some(v) = args.cluster {
        object.insert("cluster".to_owned(), crate::commands::resolve_reference_id("/api/virtualization/clusters/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "comments", args.comments);
    if let Some(v) = args.config_template {
        object.insert("config_template".to_owned(), crate::commands::resolve_reference_id("/api/extras/config-templates/", "name", &v, opts).await?);
    }
    insert_optional_string_field(object, "description", args.description);
    if let Some(v) = args.device_type {
        object.insert("device_type".to_owned(), crate::commands::resolve_reference_id("/api/dcim/device-types/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.face {
        object.insert("face".to_owned(), json!(v));
    }
    if let Some(v) = args.latitude {
        object.insert("latitude".to_owned(), json!(v));
    }
    if let Some(v) = args.location {
        object.insert("location".to_owned(), crate::commands::resolve_reference_id("/api/dcim/locations/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.longitude {
        object.insert("longitude".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "name", args.name);
    if let Some(v) = args.oob_ip {
        object.insert("oob_ip".to_owned(), crate::commands::resolve_reference_id("/api/ipam/ip-addresses/", "address", &v, opts).await?);
    }
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    if let Some(v) = args.platform {
        object.insert("platform".to_owned(), crate::commands::resolve_reference_id("/api/dcim/platforms/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.position {
        object.insert("position".to_owned(), json!(v));
    }
    if let Some(v) = args.primary_ip4 {
        object.insert("primary_ip4".to_owned(), crate::commands::resolve_reference_id("/api/ipam/ip-addresses/", "address", &v, opts).await?);
    }
    if let Some(v) = args.primary_ip6 {
        object.insert("primary_ip6".to_owned(), crate::commands::resolve_reference_id("/api/ipam/ip-addresses/", "address", &v, opts).await?);
    }
    if let Some(v) = args.rack {
        object.insert("rack".to_owned(), crate::commands::resolve_reference_id("/api/dcim/racks/", "name", &v, opts).await?);
    }
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/dcim/device-roles/", "slug", &v, opts).await?);
    }
    insert_optional_string_field(object, "serial", args.serial);
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
    if let Some(v) = args.vc_position {
        object.insert("vc_position".to_owned(), json!(v));
    }
    if let Some(v) = args.vc_priority {
        object.insert("vc_priority".to_owned(), json!(v));
    }
    if let Some(v) = args.virtual_chassis {
        object.insert("virtual_chassis".to_owned(), crate::commands::resolve_reference_id("/api/dcim/virtual-chassis/", "name", &v, opts).await?);
    }
    Ok(body)
}

/// Try to deserialize a successful NetBox response into the typify-generated
/// response struct. Used by dispatchers to detect schema drift at runtime;
/// on parse failure, nbx emits a one-line stderr warning. Set the env var
/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.
pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<crate::generated::types::DeviceWithConfigContext>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::DeviceWithConfigContext>>(value.clone()).map(|_| ())
}

