// AUTO-GENERATED. Source: schema/netbox-4.6.0.json :: WritablePrefixRequest
// Run `cargo run -p nbx-codegen -- schema/netbox-4.6.0.json src/generated/resources/` to regenerate.

use clap::Args;
use clap::ValueEnum;
use serde_json::{Map, Value, json};

use crate::commands::{GlobalOptions, ResourceSpec};
use crate::commands::insert_optional_bool_field;
use crate::commands::tags_value;
use crate::commands::insert_optional_string_field;
use crate::error::NbxResult;

pub const RESOURCE: ResourceSpec = ResourceSpec {
    app: "ipam",
    name: "prefixes",
    api_path: "/api/ipam/prefixes/",
    detail_path: "/api/ipam/prefixes/{id}/",
    default_lookup_field: "prefix",
};

#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PrefixStatus {
    Container,
    Active,
    Reserved,
    Deprecated,
}

#[derive(Debug, Args)]
pub struct PrefixCreateFields {
    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// All IP addresses within this prefix are considered usable
    #[arg(long)]
    pub is_pool: Option<bool>,

    /// Treat as fully utilized
    #[arg(long)]
    pub mark_utilized: Option<bool>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub prefix: String,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub scope_id: Option<u64>,

    #[arg(long)]
    pub scope_type: Option<String>,

    /// Operational status of this prefix
    ///
    /// * `container` - Container
    /// * `active` - Active
    /// * `reserved` - Reserved
    /// * `deprecated` - Deprecated
    #[arg(long)]
    pub status: Option<PrefixStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    #[arg(long)]
    pub vlan: Option<String>,

    #[arg(long)]
    pub vrf: Option<String>,

}

#[derive(Debug, Args)]
pub struct PrefixUpdateFields {
    #[arg(long)]
    pub comments: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    /// All IP addresses within this prefix are considered usable
    #[arg(long)]
    pub is_pool: Option<bool>,

    /// Treat as fully utilized
    #[arg(long)]
    pub mark_utilized: Option<bool>,

    #[arg(long)]
    pub owner: Option<u64>,

    #[arg(long)]
    pub prefix: Option<String>,

    #[arg(long)]
    pub role: Option<String>,

    #[arg(long)]
    pub scope_id: Option<u64>,

    #[arg(long)]
    pub scope_type: Option<String>,

    /// Operational status of this prefix
    ///
    /// * `container` - Container
    /// * `active` - Active
    /// * `reserved` - Reserved
    /// * `deprecated` - Deprecated
    #[arg(long)]
    pub status: Option<PrefixStatus>,

    #[arg(long)]
    pub tags: Option<String>,

    #[arg(long)]
    pub tenant: Option<String>,

    #[arg(long)]
    pub vlan: Option<String>,

    #[arg(long)]
    pub vrf: Option<String>,

}

pub async fn create_body(args: PrefixCreateFields, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = json!({});
    let object: &mut Map<String, Value> = body.as_object_mut()
        .expect("json!({}) always returns an object");
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_bool_field(object, "is_pool", args.is_pool);
    insert_optional_bool_field(object, "mark_utilized", args.mark_utilized);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    object.insert("prefix".to_owned(), Value::String(args.prefix));
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/ipam/roles/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.scope_id {
        object.insert("scope_id".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "scope_type", args.scope_type);
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.vlan {
        object.insert("vlan".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlans/", "name", &v, opts).await?);
    }
    if let Some(v) = args.vrf {
        object.insert("vrf".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vrfs/", "name", &v, opts).await?);
    }
    Ok(body)
}

pub async fn update_body(args: PrefixUpdateFields, opts: &GlobalOptions) -> NbxResult<Value> {
    let _ = opts;
    let mut body = json!({});
    let object: &mut Map<String, Value> = body.as_object_mut()
        .expect("json!({}) always returns an object");
    insert_optional_string_field(object, "comments", args.comments);
    insert_optional_string_field(object, "description", args.description);
    insert_optional_bool_field(object, "is_pool", args.is_pool);
    insert_optional_bool_field(object, "mark_utilized", args.mark_utilized);
    if let Some(v) = args.owner {
        object.insert("owner".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "prefix", args.prefix);
    if let Some(v) = args.role {
        object.insert("role".to_owned(), crate::commands::resolve_reference_id("/api/ipam/roles/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.scope_id {
        object.insert("scope_id".to_owned(), json!(v));
    }
    insert_optional_string_field(object, "scope_type", args.scope_type);
    if let Some(v) = args.status {
        object.insert("status".to_owned(), json!(v));
    }
    if let Some(v) = args.tags {
        object.insert("tags".to_owned(), tags_value(&v));
    }
    if let Some(v) = args.tenant {
        object.insert("tenant".to_owned(), crate::commands::resolve_reference_id("/api/tenancy/tenants/", "slug", &v, opts).await?);
    }
    if let Some(v) = args.vlan {
        object.insert("vlan".to_owned(), crate::commands::resolve_reference_id("/api/ipam/vlans/", "name", &v, opts).await?);
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
    serde_json::from_value::<crate::generated::types::Prefix>(value.clone()).map(|_| ())
}

/// Same as [`validate_response`] but expects a JSON array of resource
/// objects. Used by bulk-update dispatch.
pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {
    serde_json::from_value::<Vec<crate::generated::types::Prefix>>(value.clone()).map(|_| ())
}

