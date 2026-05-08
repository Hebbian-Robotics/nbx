use std::collections::BTreeSet;
use std::fmt::Write;

use anyhow::{Context, Result, bail};
use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use typify::{TypeSpace, TypeSpaceSettings};

pub const SUPPORTED_OPENAPI_VERSION: &str = "3.0.3";
pub const V0_1_TARGET_ENDPOINT_PATHS: &[&str] = &[
    "/api/authentication-check/",
    "/api/dcim/cables/",
    "/api/dcim/cables/{id}/",
    "/api/dcim/device-roles/",
    "/api/dcim/device-roles/{id}/",
    "/api/dcim/device-types/",
    "/api/dcim/device-types/{id}/",
    "/api/dcim/devices/",
    "/api/dcim/devices/{id}/",
    "/api/dcim/interfaces/",
    "/api/dcim/interfaces/{id}/",
    "/api/dcim/interfaces/{id}/trace/",
    "/api/dcim/inventory-items/",
    "/api/dcim/inventory-items/{id}/",
    "/api/dcim/locations/",
    "/api/dcim/locations/{id}/",
    "/api/dcim/manufacturers/",
    "/api/dcim/manufacturers/{id}/",
    "/api/dcim/platforms/",
    "/api/dcim/platforms/{id}/",
    "/api/dcim/racks/",
    "/api/dcim/racks/{id}/",
    "/api/dcim/regions/",
    "/api/dcim/regions/{id}/",
    "/api/dcim/site-groups/",
    "/api/dcim/site-groups/{id}/",
    "/api/dcim/sites/",
    "/api/dcim/sites/{id}/",
    "/api/extras/tags/",
    "/api/extras/tags/{id}/",
    "/api/ipam/aggregates/",
    "/api/ipam/aggregates/{id}/",
    "/api/ipam/ip-addresses/",
    "/api/ipam/ip-addresses/{id}/",
    "/api/ipam/prefixes/",
    "/api/ipam/prefixes/{id}/",
    "/api/ipam/prefixes/{id}/available-ips/",
    "/api/ipam/prefixes/{id}/available-prefixes/",
    "/api/ipam/rirs/",
    "/api/ipam/rirs/{id}/",
    "/api/ipam/roles/",
    "/api/ipam/roles/{id}/",
    "/api/ipam/vlan-groups/",
    "/api/ipam/vlan-groups/{id}/",
    "/api/ipam/vlans/",
    "/api/ipam/vlans/{id}/",
    "/api/ipam/vrfs/",
    "/api/ipam/vrfs/{id}/",
    "/api/status/",
    "/api/tenancy/tenant-groups/",
    "/api/tenancy/tenant-groups/{id}/",
    "/api/tenancy/tenants/",
    "/api/tenancy/tenants/{id}/",
];

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpenApiMetadata {
    pub openapi_version: String,
    pub endpoints: Vec<EndpointMetadata>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct EndpointMetadata {
    pub path: String,
    pub method: HttpMethod,
    pub operation_id: String,
    pub parameters: Vec<ParameterMetadata>,
    pub request_body_refs: Vec<String>,
    pub response_refs: Vec<ResponseMetadata>,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}

impl HttpMethod {
    fn as_schema_key(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Patch => "patch",
            Self::Delete => "delete",
        }
    }

    fn as_generated_name(self) -> &'static str {
        match self {
            Self::Get => "Get",
            Self::Post => "Post",
            Self::Patch => "Patch",
            Self::Delete => "Delete",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParameterMetadata {
    pub name: String,
    pub location: String,
    pub required: bool,
    pub description: Option<String>,
    pub schema: Value,
    pub schema_refs: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub status_code: String,
    pub refs: Vec<String>,
}

// ============================================================
// Per-resource module codegen
// ============================================================

/// Hardcoded list of `NetBox` resources whose typed CLI modules are
/// generated from the `OpenAPI` schema. When `extended = true`, codegen
/// emits only the args structs and body builders (no `Action` enum or
/// dispatcher) so a hand-written wrapper in `commands::mod` can layer
/// on resource-specific behavior (composite addressing, name resolution,
/// extra commands like `interfaces trace`).
pub const GENERATED_RESOURCES: &[ResourceCodegenSpec] = &[
    ResourceCodegenSpec {
        app: "dcim",
        name: "sites",
        ident: "Site",
        module_file: "sites.rs",
        api_path: "/api/dcim/sites/",
        detail_path: "/api/dcim/sites/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableSiteRequest",
        response_type: "Site",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "site-groups",
        ident: "SiteGroup",
        module_file: "site_groups.rs",
        api_path: "/api/dcim/site-groups/",
        detail_path: "/api/dcim/site-groups/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableSiteGroupRequest",
        response_type: "SiteGroup",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "regions",
        ident: "Region",
        module_file: "regions.rs",
        api_path: "/api/dcim/regions/",
        detail_path: "/api/dcim/regions/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableRegionRequest",
        response_type: "Region",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "locations",
        ident: "Location",
        module_file: "locations.rs",
        api_path: "/api/dcim/locations/",
        detail_path: "/api/dcim/locations/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableLocationRequest",
        response_type: "Location",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "manufacturers",
        ident: "Manufacturer",
        module_file: "manufacturers.rs",
        api_path: "/api/dcim/manufacturers/",
        detail_path: "/api/dcim/manufacturers/{id}/",
        default_lookup_field: "slug",
        request_schema: "ManufacturerRequest",
        response_type: "Manufacturer",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "device-types",
        ident: "DeviceType",
        module_file: "device_types.rs",
        api_path: "/api/dcim/device-types/",
        detail_path: "/api/dcim/device-types/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableDeviceTypeRequest",
        response_type: "DeviceType",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "device-roles",
        ident: "DeviceRole",
        module_file: "device_roles.rs",
        api_path: "/api/dcim/device-roles/",
        detail_path: "/api/dcim/device-roles/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableDeviceRoleRequest",
        response_type: "DeviceRole",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "platforms",
        ident: "Platform",
        module_file: "platforms.rs",
        api_path: "/api/dcim/platforms/",
        detail_path: "/api/dcim/platforms/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritablePlatformRequest",
        response_type: "Platform",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "racks",
        ident: "Rack",
        module_file: "racks.rs",
        api_path: "/api/dcim/racks/",
        detail_path: "/api/dcim/racks/{id}/",
        default_lookup_field: "name",
        request_schema: "WritableRackRequest",
        response_type: "Rack",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "inventory-items",
        ident: "InventoryItem",
        module_file: "inventory_items.rs",
        api_path: "/api/dcim/inventory-items/",
        detail_path: "/api/dcim/inventory-items/{id}/",
        default_lookup_field: "name",
        request_schema: "WritableInventoryItemRequest",
        response_type: "InventoryItem",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "ip-addresses",
        ident: "IpAddress",
        module_file: "ip_addresses.rs",
        api_path: "/api/ipam/ip-addresses/",
        detail_path: "/api/ipam/ip-addresses/{id}/",
        default_lookup_field: "address",
        request_schema: "WritableIPAddressRequest",
        response_type: "IpAddress",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "prefixes",
        ident: "Prefix",
        module_file: "prefixes.rs",
        api_path: "/api/ipam/prefixes/",
        detail_path: "/api/ipam/prefixes/{id}/",
        default_lookup_field: "prefix",
        request_schema: "WritablePrefixRequest",
        response_type: "Prefix",
        extended: true,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "vrfs",
        ident: "Vrf",
        module_file: "vrfs.rs",
        api_path: "/api/ipam/vrfs/",
        detail_path: "/api/ipam/vrfs/{id}/",
        default_lookup_field: "name",
        request_schema: "VRFRequest",
        response_type: "Vrf",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "roles",
        ident: "Role",
        module_file: "roles.rs",
        api_path: "/api/ipam/roles/",
        detail_path: "/api/ipam/roles/{id}/",
        default_lookup_field: "slug",
        request_schema: "RoleRequest",
        response_type: "Role",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "vlan-groups",
        ident: "VlanGroup",
        module_file: "vlan_groups.rs",
        api_path: "/api/ipam/vlan-groups/",
        detail_path: "/api/ipam/vlan-groups/{id}/",
        default_lookup_field: "slug",
        request_schema: "VLANGroupRequest",
        response_type: "VlanGroup",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "rirs",
        ident: "Rir",
        module_file: "rirs.rs",
        api_path: "/api/ipam/rirs/",
        detail_path: "/api/ipam/rirs/{id}/",
        default_lookup_field: "slug",
        request_schema: "RIRRequest",
        response_type: "Rir",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "aggregates",
        ident: "Aggregate",
        module_file: "aggregates.rs",
        api_path: "/api/ipam/aggregates/",
        detail_path: "/api/ipam/aggregates/{id}/",
        default_lookup_field: "prefix",
        request_schema: "WritableAggregateRequest",
        response_type: "Aggregate",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "ipam",
        name: "vlans",
        ident: "Vlan",
        module_file: "vlans.rs",
        api_path: "/api/ipam/vlans/",
        detail_path: "/api/ipam/vlans/{id}/",
        default_lookup_field: "name",
        request_schema: "WritableVLANRequest",
        response_type: "Vlan",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "extras",
        name: "tags",
        ident: "Tag",
        module_file: "tags.rs",
        api_path: "/api/extras/tags/",
        detail_path: "/api/extras/tags/{id}/",
        default_lookup_field: "slug",
        request_schema: "TagRequest",
        response_type: "Tag",
        extended: false,
        skip_fields: &["object_types"],
    },
    ResourceCodegenSpec {
        app: "tenancy",
        name: "tenant-groups",
        ident: "TenantGroup",
        module_file: "tenant_groups.rs",
        api_path: "/api/tenancy/tenant-groups/",
        detail_path: "/api/tenancy/tenant-groups/{id}/",
        default_lookup_field: "slug",
        request_schema: "WritableTenantGroupRequest",
        response_type: "TenantGroup",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "tenancy",
        name: "tenants",
        ident: "Tenant",
        module_file: "tenants.rs",
        api_path: "/api/tenancy/tenants/",
        detail_path: "/api/tenancy/tenants/{id}/",
        default_lookup_field: "slug",
        request_schema: "TenantRequest",
        response_type: "Tenant",
        extended: false,
        skip_fields: &[],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "cables",
        ident: "Cable",
        module_file: "cables.rs",
        api_path: "/api/dcim/cables/",
        detail_path: "/api/dcim/cables/{id}/",
        default_lookup_field: "label",
        request_schema: "WritableCableRequest",
        response_type: "Cable",
        extended: false,
        skip_fields: &["a_terminations", "b_terminations"],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "devices",
        ident: "Device",
        module_file: "devices.rs",
        api_path: "/api/dcim/devices/",
        detail_path: "/api/dcim/devices/{id}/",
        default_lookup_field: "name",
        request_schema: "WritableDeviceWithConfigContextRequest",
        response_type: "DeviceWithConfigContext",
        extended: true,
        // Skip large free-form / advanced shapes that are easier to set via --data.
        skip_fields: &["local_context_data"],
    },
    ResourceCodegenSpec {
        app: "dcim",
        name: "interfaces",
        ident: "Interface",
        module_file: "interfaces.rs",
        api_path: "/api/dcim/interfaces/",
        detail_path: "/api/dcim/interfaces/{id}/",
        default_lookup_field: "name",
        request_schema: "WritableInterfaceRequest",
        response_type: "Interface",
        extended: true,
        // The hand-written wrapper handles these fields with name resolution
        // and device-scoped lookups; the generated module exposes everything else.
        skip_fields: &["device", "lag", "untagged_vlan", "tagged_vlans"],
    },
];

#[derive(Debug, Clone, Copy)]
pub struct ResourceCodegenSpec {
    pub app: &'static str,
    pub name: &'static str,
    pub ident: &'static str,
    pub module_file: &'static str,
    pub api_path: &'static str,
    pub detail_path: &'static str,
    pub default_lookup_field: &'static str,
    pub request_schema: &'static str,
    /// Typify-generated Rust ident in `crate::generated::types` used to
    /// validate the GET/POST/PATCH response shape after a successful round-trip.
    /// A drift-detection layer; on parse failure nbx warns to stderr.
    pub response_type: &'static str,
    /// When true, codegen omits the `Action` enum and `run_<x>_action` dispatcher
    /// (the hand-written wrapper supplies its own) and the `--data` / `--data-file`
    /// flags (those go on the wrapper).
    pub extended: bool,
    /// Fields to drop from generated args. Useful when the hand-written wrapper
    /// has its own typed flag for a property (e.g. interfaces' `device` accepts
    /// a name and resolves to an ID).
    pub skip_fields: &'static [&'static str],
}

/// Render the contents of every per-resource module plus the aggregator `mod.rs`.
/// Returns `(filename, source)` pairs. Filenames are relative to
/// `src/generated/resources/`.
pub fn render_resource_modules(schema_text: &str) -> Result<Vec<(String, String)>> {
    let root_value =
        serde_json::from_str::<Value>(schema_text).context("failed to parse OpenAPI JSON")?;
    let mut outputs = Vec::new();

    for spec in GENERATED_RESOURCES {
        let request_schema_value = root_value
            .pointer(&format!("/components/schemas/{}", spec.request_schema))
            .with_context(|| format!("schema missing component {}", spec.request_schema))?;
        let module_source = render_one_resource_module(spec, request_schema_value)?;
        outputs.push((spec.module_file.to_owned(), module_source));
    }

    outputs.push(("mod.rs".to_owned(), render_resource_aggregator()));

    Ok(outputs)
}

fn render_resource_aggregator() -> String {
    let mut out = String::from(
        "// AUTO-GENERATED. Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/`.\n\n",
    );
    for spec in GENERATED_RESOURCES {
        let module = spec.module_file.trim_end_matches(".rs");
        writeln!(out, "#[rustfmt::skip]\npub mod {module};")
            .expect("writing to String never fails");
    }
    out.push('\n');
    out.push_str("use crate::commands::ResourceSpec;\n\n");
    out.push_str("pub fn all_resource_specs() -> Vec<ResourceSpec> {\n    vec![\n");
    for spec in GENERATED_RESOURCES {
        let module = spec.module_file.trim_end_matches(".rs");
        writeln!(out, "        {module}::RESOURCE,").expect("writing to String never fails");
    }
    out.push_str("    ]\n}\n");
    out
}

/// Hand-written replacement for a resource's generated `List` action. When a
/// resource appears here the codegen emits `List({args_ident})` and dispatches
/// to `{dispatcher_path}` instead of the shared `GenericListArgs` / `generic_list`
/// pair. Use this when the typed list filters required by the resource's primary
/// use case don't fit the minimal `--name`/`--tag` set on `GenericListArgs`.
#[derive(Debug, Clone, Copy)]
struct ListOverride {
    args_ident: &'static str,
    dispatcher_path: &'static str,
}

const LIST_OVERRIDES: &[(&str, ListOverride)] = &[(
    "InventoryItem",
    ListOverride {
        args_ident: "InventoryItemListArgs",
        dispatcher_path: "crate::commands::inventory_items_list",
    },
)];

fn list_override_for(resource_ident: &str) -> Option<&'static ListOverride> {
    LIST_OVERRIDES
        .iter()
        .find(|(ident, _)| *ident == resource_ident)
        .map(|(_, override_)| override_)
}

/// Maps the schema name of a `Brief<X>Request` to the `NetBox` endpoint and
/// lookup field used to resolve a name/slug to a numeric ID. Driven by the
/// codegen — for any FK whose target appears here, the generated arg becomes
/// `Option<String>` and the body builder calls
/// `crate::commands::resolve_reference_id`.
const FK_RESOLVERS: &[(&str, &str, &str)] = &[
    // (brief_request_schema, api_path, lookup_field)
    (
        "BriefClusterRequest",
        "/api/virtualization/clusters/",
        "name",
    ),
    (
        "BriefConfigTemplateRequest",
        "/api/extras/config-templates/",
        "name",
    ),
    (
        "BriefContactRoleRequest",
        "/api/tenancy/contact-roles/",
        "slug",
    ),
    ("BriefDeviceRequest", "/api/dcim/devices/", "name"),
    ("BriefDeviceRoleRequest", "/api/dcim/device-roles/", "slug"),
    ("BriefDeviceTypeRequest", "/api/dcim/device-types/", "slug"),
    (
        "BriefIPAddressRequest",
        "/api/ipam/ip-addresses/",
        "address",
    ),
    ("BriefInterfaceRequest", "/api/dcim/interfaces/", "name"),
    (
        "BriefInventoryItemRoleRequest",
        "/api/dcim/inventory-item-roles/",
        "slug",
    ),
    ("BriefLocationRequest", "/api/dcim/locations/", "slug"),
    (
        "BriefMACAddressRequest",
        "/api/dcim/mac-addresses/",
        "mac_address",
    ),
    (
        "BriefManufacturerRequest",
        "/api/dcim/manufacturers/",
        "slug",
    ),
    ("BriefModuleRequest", "/api/dcim/modules/", "serial"),
    ("BriefPlatformRequest", "/api/dcim/platforms/", "slug"),
    ("BriefRackRequest", "/api/dcim/racks/", "name"),
    ("BriefRackRoleRequest", "/api/dcim/rack-roles/", "slug"),
    ("BriefRackTypeRequest", "/api/dcim/rack-types/", "slug"),
    ("BriefRegionRequest", "/api/dcim/regions/", "slug"),
    ("BriefRIRRequest", "/api/ipam/rirs/", "slug"),
    ("BriefRoleRequest", "/api/ipam/roles/", "slug"),
    ("BriefSiteGroupRequest", "/api/dcim/site-groups/", "slug"),
    ("BriefSiteRequest", "/api/dcim/sites/", "slug"),
    (
        "BriefTenantGroupRequest",
        "/api/tenancy/tenant-groups/",
        "slug",
    ),
    ("BriefTenantRequest", "/api/tenancy/tenants/", "slug"),
    ("BriefVLANGroupRequest", "/api/ipam/vlan-groups/", "slug"),
    ("BriefVLANRequest", "/api/ipam/vlans/", "name"),
    (
        "BriefVLANTranslationPolicyRequest",
        "/api/ipam/vlan-translation-policies/",
        "name",
    ),
    ("BriefVRFRequest", "/api/ipam/vrfs/", "name"),
    (
        "BriefVirtualChassisRequest",
        "/api/dcim/virtual-chassis/",
        "name",
    ),
];

#[derive(Debug, Clone)]
enum PropertyKind {
    String,
    Integer,
    Number,
    Boolean,
    Enum {
        values: Vec<String>,
    },
    /// FK with a known resolver — the field accepts a name or slug, and the
    /// body builder calls `resolve_reference_id` to swap it for a numeric ID.
    ForeignKeyResolvable {
        api_path: &'static str,
        lookup_field: &'static str,
    },
    /// FK without a resolver — accepts a numeric ID only.
    ForeignKeyNumeric,
    /// Array of `NestedTagRequest` — accepts a comma-separated list of tag
    /// slugs; body builder produces `[{"slug": "..."}, ...]`.
    TagArray,
    Skip,
}

fn classify_property(value: &Value) -> PropertyKind {
    let Some(object) = value.as_object() else {
        return PropertyKind::Skip;
    };

    if let Some(one_of) = object.get("oneOf").and_then(Value::as_array)
        && one_of.len() == 2
    {
        let first_is_integer = one_of[0]
            .as_object()
            .and_then(|o| o.get("type"))
            .and_then(Value::as_str)
            == Some("integer");
        let second_has_ref = one_of[1].as_object().is_some_and(|o| {
            o.contains_key("$ref") || o.contains_key("allOf") || o.contains_key("oneOf")
        });
        if first_is_integer && second_has_ref {
            let target = extract_brief_request_name(&one_of[1]);
            if let Some(brief_name) = target.as_deref()
                && let Some((_, api_path, lookup_field)) =
                    FK_RESOLVERS.iter().find(|(name, _, _)| *name == brief_name)
            {
                return PropertyKind::ForeignKeyResolvable {
                    api_path,
                    lookup_field,
                };
            }
            return PropertyKind::ForeignKeyNumeric;
        }
    }

    if object.get("type").and_then(Value::as_str) == Some("array")
        && let Some(items) = object.get("items").and_then(Value::as_object)
        && let Some(reference) = items.get("$ref").and_then(Value::as_str)
        && reference.ends_with("/NestedTagRequest")
    {
        return PropertyKind::TagArray;
    }

    if let (Some(enum_values), Some("string")) = (
        object.get("enum").and_then(Value::as_array),
        object.get("type").and_then(Value::as_str),
    ) {
        let values: Vec<String> = enum_values
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        if !values.is_empty() {
            return PropertyKind::Enum { values };
        }
    }

    match object.get("type").and_then(Value::as_str) {
        Some("string") => PropertyKind::String,
        Some("integer") => PropertyKind::Integer,
        Some("number") => PropertyKind::Number,
        Some("boolean") => PropertyKind::Boolean,
        _ => PropertyKind::Skip,
    }
}

/// Pull a `BriefXRequest` schema name out of an FK property's second `oneOf`
/// branch. The shape is either `{ "$ref": "#/components/schemas/Brief..." }`
/// or `{ "allOf": [{ "$ref": "..." }] }` (sometimes wrapped with `nullable`).
fn extract_brief_request_name(value: &Value) -> Option<String> {
    let object = value.as_object()?;
    if let Some(reference) = object.get("$ref").and_then(Value::as_str) {
        return reference
            .strip_prefix("#/components/schemas/")
            .map(String::from);
    }
    if let Some(all_of) = object.get("allOf").and_then(Value::as_array)
        && let Some(first) = all_of.first()
        && let Some(reference) = first.get("$ref").and_then(Value::as_str)
    {
        return reference
            .strip_prefix("#/components/schemas/")
            .map(String::from);
    }
    None
}

#[derive(Debug, Clone)]
struct ResolvedProperty {
    schema_name: String,
    rust_field_ident: String,
    rust_arg_attr: String,
    description: Option<String>,
    kind: PropertyKind,
    required: bool,
}

fn resolve_properties(
    spec: &ResourceCodegenSpec,
    request_schema_value: &Value,
) -> Result<Vec<ResolvedProperty>> {
    let request_object = request_schema_value.as_object().with_context(|| {
        format!(
            "schema component {} is missing properties",
            spec.request_schema
        )
    })?;
    let properties_object = request_object
        .get("properties")
        .and_then(Value::as_object)
        .with_context(|| {
            format!(
                "schema component {} is missing properties",
                spec.request_schema
            )
        })?;
    let required_set: std::collections::BTreeSet<String> = request_object
        .get("required")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut resolved = Vec::new();
    for (schema_name, value) in properties_object {
        if spec.skip_fields.contains(&schema_name.as_str()) {
            continue;
        }
        let kind = classify_property(value);
        if matches!(kind, PropertyKind::Skip) {
            continue;
        }
        let rust_field_ident = rust_field_name(schema_name);
        let rust_arg_attr = clap_long_attribute(schema_name, &rust_field_ident);
        let description = value
            .get("description")
            .and_then(Value::as_str)
            .map(String::from);
        resolved.push(ResolvedProperty {
            schema_name: schema_name.clone(),
            rust_field_ident,
            rust_arg_attr,
            description,
            kind,
            required: required_set.contains(schema_name),
        });
    }
    Ok(resolved)
}

fn rust_field_name(schema_name: &str) -> String {
    // Schema field names are already snake_case; rename Rust keywords.
    match schema_name {
        "type" => "r#type".to_owned(),
        other => other.to_owned(),
    }
}

fn clap_long_attribute(schema_name: &str, rust_field_ident: &str) -> String {
    // Default: clap derives the flag name from the field. If the schema name has
    // an underscore the default kebab-cases it (`time_zone` -> `--time-zone`).
    // Raw identifiers need an explicit override.
    if rust_field_ident.starts_with("r#") {
        format!("#[arg(long = {schema_name:?})]")
    } else {
        "#[arg(long)]".to_owned()
    }
}

fn render_one_resource_module(
    spec: &ResourceCodegenSpec,
    request_schema_value: &Value,
) -> Result<String> {
    let properties = resolve_properties(spec, request_schema_value)?;

    let mut out = String::new();
    out.push_str("// AUTO-GENERATED. Source: schema/netbox-4.5.10.json :: ");
    out.push_str(spec.request_schema);
    out.push_str(
        "\n// Run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/resources/` to regenerate.\n\n",
    );

    let has_bool_field = properties
        .iter()
        .any(|p| matches!(p.kind, PropertyKind::Boolean));
    let has_value_enum = properties
        .iter()
        .any(|p| matches!(p.kind, PropertyKind::Enum { .. }));

    out.push_str("use clap::Args;\n");
    if has_value_enum {
        out.push_str("use clap::ValueEnum;\n");
    }
    if !spec.extended {
        out.push_str("use clap::Subcommand;\n");
    }
    out.push_str("use serde_json::{Map, Value, json};\n\n");
    out.push_str("use crate::commands::{GlobalOptions, ResourceSpec};\n");
    if has_bool_field {
        out.push_str("use crate::commands::insert_optional_bool_field;\n");
    }
    let has_tag_field = properties
        .iter()
        .any(|p| matches!(p.kind, PropertyKind::TagArray));
    if has_tag_field {
        out.push_str("use crate::commands::tags_value;\n");
    }
    out.push_str("use crate::commands::insert_optional_string_field;\n");
    if spec.extended {
        out.push_str("use crate::error::NbxResult;\n\n");
    } else {
        let list_override = list_override_for(spec.ident);
        out.push_str("use crate::commands::optional_json_body;\n");
        out.push_str("use crate::commands::{\n");
        if list_override.is_some() {
            out.push_str("    BulkDeleteArgs, DeleteArgs, GenericMutationArgs, GetArgs,\n");
            out.push_str("    bulk_delete_resource, delete_resource, generic_get,\n");
            out.push_str("    generic_mutation_body, mutate_collection_with_body,\n");
        } else {
            out.push_str(
                "    BulkDeleteArgs, DeleteArgs, GenericListArgs, GenericMutationArgs, GetArgs,\n",
            );
            out.push_str("    bulk_delete_resource, delete_resource, generic_get,\n");
            out.push_str("    generic_list, generic_mutation_body, mutate_collection_with_body,\n");
        }
        out.push_str("    update_resource_with_body,\n");
        out.push_str("};\n");
        if let Some(override_) = list_override {
            writeln!(
                out,
                "use crate::commands::{ident};",
                ident = override_.args_ident
            )
            .unwrap();
        }
        out.push_str("use crate::generated::endpoints::HttpMethod;\n");
        out.push_str("use crate::error::{NbxError, NbxResult};\n\n");
    }

    writeln!(out, "pub const RESOURCE: ResourceSpec = ResourceSpec {{").unwrap();
    writeln!(out, "    app: {:?},", spec.app).unwrap();
    writeln!(out, "    name: {:?},", spec.name).unwrap();
    writeln!(out, "    api_path: {:?},", spec.api_path).unwrap();
    writeln!(out, "    detail_path: {:?},", spec.detail_path).unwrap();
    writeln!(
        out,
        "    default_lookup_field: {:?},",
        spec.default_lookup_field
    )
    .unwrap();
    out.push_str("};\n\n");

    // Status / role enums (generated per enum-typed property).
    for property in &properties {
        if let PropertyKind::Enum { values } = &property.kind {
            render_value_enum(&mut out, spec.ident, &property.schema_name, values);
        }
    }

    render_create_args(&mut out, spec.ident, &properties, spec.extended);
    render_update_args(&mut out, spec.ident, &properties, spec.extended);
    if !spec.extended {
        render_action_enum(&mut out, spec.ident);
        render_dispatcher(&mut out, spec);
    }
    render_create_body(&mut out, spec.ident, &properties, spec.extended);
    render_update_body(&mut out, spec.ident, &properties, spec.extended);
    render_response_validator(&mut out, spec);

    Ok(out)
}

fn render_response_validator(out: &mut String, spec: &ResourceCodegenSpec) {
    let response_type = spec.response_type;
    out.push_str("/// Try to deserialize a successful NetBox response into the typify-generated\n");
    out.push_str("/// response struct. Used by dispatchers to detect schema drift at runtime;\n");
    out.push_str("/// on parse failure, nbx emits a one-line stderr warning. Set the env var\n");
    out.push_str("/// `NBX_SKIP_RESPONSE_VALIDATION=1` to silence the check.\n");
    out.push_str("pub fn validate_response(value: &Value) -> Result<(), serde_json::Error> {\n");
    writeln!(
        out,
        "    serde_json::from_value::<crate::generated::types::{response_type}>(value.clone()).map(|_| ())"
    )
    .unwrap();
    out.push_str("}\n\n");

    // Bulk operations (PATCH on the collection endpoint with an array body)
    // return an array of single-resource objects. This validator deserializes
    // the response as `Vec<T>`, so each row is shape-checked the same way as
    // a single-record response.
    out.push_str("/// Same as [`validate_response`] but expects a JSON array of resource\n");
    out.push_str("/// objects. Used by bulk-update dispatch.\n");
    out.push_str(
        "pub fn validate_bulk_response(value: &Value) -> Result<(), serde_json::Error> {\n",
    );
    writeln!(
        out,
        "    serde_json::from_value::<Vec<crate::generated::types::{response_type}>>(value.clone()).map(|_| ())"
    )
    .unwrap();
    out.push_str("}\n\n");
}

fn render_value_enum(out: &mut String, resource_ident: &str, field_name: &str, values: &[String]) {
    let enum_ident = enum_ident_name(resource_ident, field_name);
    out.push_str("#[derive(Debug, Clone, Copy, ValueEnum, serde::Serialize)]\n");
    out.push_str("#[serde(rename_all = \"lowercase\")]\n");
    writeln!(out, "pub enum {enum_ident} {{").unwrap();

    let mut used_idents: std::collections::BTreeMap<String, u32> =
        std::collections::BTreeMap::new();
    for value in values {
        let base_variant = string_value_to_variant_ident(value);
        let count = used_idents.entry(base_variant.clone()).or_insert(0);
        *count += 1;
        let variant = if *count == 1 {
            base_variant.clone()
        } else {
            format!("{base_variant}{count}")
        };

        let serde_name = value.as_str();
        if serde_name == variant.to_lowercase() {
            writeln!(out, "    {variant},").unwrap();
        } else {
            writeln!(out, "    #[serde(rename = {serde_name:?})]").unwrap();
            writeln!(out, "    #[value(name = {serde_name:?})]").unwrap();
            writeln!(out, "    {variant},").unwrap();
        }
    }
    out.push_str("}\n\n");
}

fn enum_ident_name(resource_ident: &str, field_name: &str) -> String {
    format!("{resource_ident}{}", to_pascal_case(field_name))
}

fn string_value_to_variant_ident(value: &str) -> String {
    let pascal = to_pascal_case(value);
    let mut chars = pascal.chars();
    if let Some(first) = chars.next()
        && first.is_ascii_digit()
    {
        let prefix = match first {
            '0' => "Zero",
            '1' => "One",
            '2' => "Two",
            '3' => "Three",
            '4' => "Four",
            '5' => "Five",
            '6' => "Six",
            '7' => "Seven",
            '8' => "Eight",
            '9' => "Nine",
            _ => unreachable!(),
        };
        return format!("{prefix}{}", chars.as_str());
    }
    pascal
}

fn to_pascal_case(input: &str) -> String {
    let mut out = String::new();
    let mut capitalize_next = true;
    for ch in input.chars() {
        if !ch.is_ascii_alphanumeric() {
            capitalize_next = true;
        } else if capitalize_next {
            out.extend(ch.to_uppercase());
            capitalize_next = false;
        } else {
            out.push(ch);
        }
    }
    out
}

fn render_create_args(
    out: &mut String,
    resource_ident: &str,
    properties: &[ResolvedProperty],
    extended: bool,
) {
    let suffix = if extended {
        "CreateFields"
    } else {
        "CreateArgs"
    };
    writeln!(out, "#[derive(Debug, Args)]").unwrap();
    writeln!(out, "pub struct {resource_ident}{suffix} {{").unwrap();
    for property in properties {
        write_doc_comment(out, property.description.as_deref(), "    ");
        writeln!(out, "    {}", property.rust_arg_attr).unwrap();
        let type_text = create_field_type(resource_ident, property);
        writeln!(out, "    pub {}: {type_text},\n", property.rust_field_ident).unwrap();
    }
    if !extended {
        out.push_str(
            "    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.\n",
        );
        out.push_str("    #[arg(long, conflicts_with = \"data_file\")]\n");
        out.push_str("    pub data: Option<String>,\n\n");
        out.push_str("    #[arg(long, value_name = \"PATH\")]\n");
        out.push_str("    pub data_file: Option<String>,\n");
    }
    out.push_str("}\n\n");
}

fn render_update_args(
    out: &mut String,
    resource_ident: &str,
    properties: &[ResolvedProperty],
    extended: bool,
) {
    let suffix = if extended {
        "UpdateFields"
    } else {
        "UpdateArgs"
    };
    writeln!(out, "#[derive(Debug, Args)]").unwrap();
    writeln!(out, "pub struct {resource_ident}{suffix} {{").unwrap();
    if !extended {
        out.push_str("    /// Numeric NetBox ID or default-lookup-field value.\n");
        out.push_str("    pub id_or_lookup: String,\n\n");
        out.push_str("    /// Override the default lookup field.\n");
        out.push_str("    #[arg(long)]\n");
        out.push_str("    pub lookup_field: Option<String>,\n\n");
    }
    for property in properties {
        write_doc_comment(out, property.description.as_deref(), "    ");
        writeln!(out, "    {}", property.rust_arg_attr).unwrap();
        let type_text = update_field_type(resource_ident, property);
        writeln!(out, "    pub {}: {type_text},\n", property.rust_field_ident).unwrap();
    }
    if !extended {
        out.push_str(
            "    /// Free-form JSON body merged with flag-derived fields. Flags take precedence.\n",
        );
        out.push_str("    #[arg(long, conflicts_with = \"data_file\")]\n");
        out.push_str("    pub data: Option<String>,\n\n");
        out.push_str("    #[arg(long, value_name = \"PATH\")]\n");
        out.push_str("    pub data_file: Option<String>,\n");
    }
    out.push_str("}\n\n");
}

fn create_field_type(resource_ident: &str, property: &ResolvedProperty) -> String {
    let inner = base_type_text(resource_ident, property);
    if property.required {
        inner
    } else {
        format!("Option<{inner}>")
    }
}

fn update_field_type(resource_ident: &str, property: &ResolvedProperty) -> String {
    let inner = base_type_text(resource_ident, property);
    format!("Option<{inner}>")
}

fn base_type_text(resource_ident: &str, property: &ResolvedProperty) -> String {
    match &property.kind {
        PropertyKind::String
        | PropertyKind::ForeignKeyResolvable { .. }
        | PropertyKind::TagArray => "String".to_owned(),
        PropertyKind::Integer | PropertyKind::ForeignKeyNumeric => "u64".to_owned(),
        PropertyKind::Number => "f64".to_owned(),
        PropertyKind::Boolean => "bool".to_owned(),
        PropertyKind::Enum { .. } => enum_ident_name(resource_ident, &property.schema_name),
        PropertyKind::Skip => unreachable!("skip properties are filtered before rendering"),
    }
}

fn write_doc_comment(out: &mut String, description: Option<&str>, indent: &str) {
    let Some(description) = description else {
        return;
    };
    let trimmed = description.trim();
    if trimmed.is_empty() {
        return;
    }
    for line in trimmed.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            writeln!(out, "{indent}///").unwrap();
        } else {
            writeln!(out, "{indent}/// {line}").unwrap();
        }
    }
}

fn render_action_enum(out: &mut String, resource_ident: &str) {
    writeln!(out, "#[derive(Debug, Subcommand)]").unwrap();
    writeln!(out, "pub enum {resource_ident}Action {{").unwrap();
    let list_args_ident = list_override_for(resource_ident)
        .map_or("GenericListArgs", |override_| override_.args_ident);
    writeln!(out, "    List({list_args_ident}),").unwrap();
    out.push_str("    Get(GetArgs),\n");
    writeln!(out, "    Create({resource_ident}CreateArgs),").unwrap();
    writeln!(out, "    Update({resource_ident}UpdateArgs),").unwrap();
    out.push_str("    Delete(DeleteArgs),\n");
    out.push_str("    #[command(name = \"bulk-update\")]\n");
    out.push_str("    BulkUpdate(GenericMutationArgs),\n");
    out.push_str("    #[command(name = \"bulk-delete\")]\n");
    out.push_str("    BulkDelete(BulkDeleteArgs),\n");
    out.push_str("}\n\n");
}

fn render_dispatcher(out: &mut String, spec: &ResourceCodegenSpec) {
    let resource_ident = spec.ident;
    let snake = ident_to_snake(resource_ident);
    writeln!(
        out,
        "pub async fn run_{snake}_action(action: {resource_ident}Action, opts: &GlobalOptions) -> NbxResult<()> {{"
    )
    .unwrap();
    out.push_str("    match action {\n");
    let list_dispatch = list_override_for(resource_ident).map_or_else(
        || "generic_list(RESOURCE, args, opts).await".to_owned(),
        |override_| format!("{}(args, opts).await", override_.dispatcher_path),
    );
    writeln!(
        out,
        "        {resource_ident}Action::List(args) => {list_dispatch},"
    )
    .unwrap();
    writeln!(
        out,
        "        {resource_ident}Action::Get(args) => generic_get(RESOURCE, args, Some(validate_response), opts).await,"
    )
    .unwrap();
    writeln!(out, "        {resource_ident}Action::Create(args) => {{").unwrap();
    out.push_str("            let body = create_body(args, opts).await?;\n");
    out.push_str("            mutate_collection_with_body(RESOURCE, HttpMethod::Post, body, Some(validate_response), opts).await\n");
    out.push_str("        }\n");
    writeln!(out, "        {resource_ident}Action::Update(args) => {{").unwrap();
    out.push_str("            let id = args.id_or_lookup.clone();\n");
    out.push_str("            let lookup = args.lookup_field.clone();\n");
    out.push_str("            let body = update_body(args, opts).await?;\n");
    out.push_str("            update_resource_with_body(RESOURCE, &id, lookup.as_deref(), body, Some(validate_response), opts).await\n");
    out.push_str("        }\n");
    writeln!(
        out,
        "        {resource_ident}Action::Delete(args) => delete_resource(RESOURCE, args, opts).await,"
    )
    .unwrap();
    writeln!(
        out,
        "        {resource_ident}Action::BulkUpdate(args) => {{"
    )
    .unwrap();
    out.push_str("            mutate_collection_with_body(RESOURCE, HttpMethod::Patch, generic_mutation_body(args)?, Some(validate_bulk_response), opts).await\n");
    out.push_str("        }\n");
    writeln!(
        out,
        "        {resource_ident}Action::BulkDelete(args) => bulk_delete_resource(RESOURCE, args, opts).await,"
    )
    .unwrap();
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn ident_to_snake(ident: &str) -> String {
    let mut out = String::new();
    for (index, ch) in ident.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn render_create_body(
    out: &mut String,
    resource_ident: &str,
    properties: &[ResolvedProperty],
    extended: bool,
) {
    let visibility = if extended { "pub " } else { "" };
    let suffix = if extended {
        "CreateFields"
    } else {
        "CreateArgs"
    };
    writeln!(
        out,
        "{visibility}async fn create_body(args: {resource_ident}{suffix}, opts: &GlobalOptions) -> NbxResult<Value> {{"
    )
    .unwrap();
    out.push_str("    let _ = opts;\n");
    body_prelude(out, extended);
    for property in properties {
        emit_property_into_body(out, property, /*is_create*/ true);
    }
    out.push_str("    Ok(body)\n");
    out.push_str("}\n\n");
}

fn render_update_body(
    out: &mut String,
    resource_ident: &str,
    properties: &[ResolvedProperty],
    extended: bool,
) {
    let visibility = if extended { "pub " } else { "" };
    let suffix = if extended {
        "UpdateFields"
    } else {
        "UpdateArgs"
    };
    writeln!(
        out,
        "{visibility}async fn update_body(args: {resource_ident}{suffix}, opts: &GlobalOptions) -> NbxResult<Value> {{"
    )
    .unwrap();
    out.push_str("    let _ = opts;\n");
    body_prelude(out, extended);
    for property in properties {
        emit_property_into_body(out, property, /*is_create*/ false);
    }
    out.push_str("    Ok(body)\n");
    out.push_str("}\n\n");
}

fn body_prelude(out: &mut String, extended: bool) {
    if extended {
        // Extended body builders return a freshly-built JSON object — the
        // hand-written wrapper is responsible for merging --data and
        // extended-resolved fields on top.
        out.push_str("    let mut body = json!({});\n");
        out.push_str("    let object: &mut Map<String, Value> = body.as_object_mut()\n");
        out.push_str("        .expect(\"json!({}) always returns an object\");\n");
    } else {
        out.push_str("    let mut body = optional_json_body(args.data, args.data_file)?\n");
        out.push_str("        .unwrap_or_else(|| json!({}));\n");
        out.push_str("    let object: &mut Map<String, Value> = body.as_object_mut()\n");
        out.push_str("        .ok_or_else(|| NbxError::validation(\"mutation payload must be a JSON object\", json!({})))?;\n");
    }
}

fn emit_property_into_body(out: &mut String, property: &ResolvedProperty, is_create: bool) {
    let access = format!("args.{}", property.rust_field_ident);
    let key = property.schema_name.as_str();
    let required_in_create = is_create && property.required;

    if required_in_create {
        match &property.kind {
            PropertyKind::String => {
                writeln!(
                    out,
                    "    object.insert({key:?}.to_owned(), Value::String({access}));"
                )
                .unwrap();
            }
            PropertyKind::ForeignKeyResolvable {
                api_path,
                lookup_field,
            } => {
                writeln!(
                    out,
                    "    object.insert({key:?}.to_owned(), crate::commands::resolve_reference_id({api_path:?}, {lookup_field:?}, &{access}, opts).await?);"
                )
                .unwrap();
            }
            PropertyKind::TagArray => {
                writeln!(
                    out,
                    "    object.insert({key:?}.to_owned(), tags_value(&{access}));"
                )
                .unwrap();
            }
            PropertyKind::Skip => {}
            _ => {
                writeln!(
                    out,
                    "    object.insert({key:?}.to_owned(), json!({access}));"
                )
                .unwrap();
            }
        }
        return;
    }

    match &property.kind {
        PropertyKind::String => {
            writeln!(
                out,
                "    insert_optional_string_field(object, {key:?}, {access});"
            )
            .unwrap();
        }
        PropertyKind::Boolean => {
            writeln!(
                out,
                "    insert_optional_bool_field(object, {key:?}, {access});"
            )
            .unwrap();
        }
        PropertyKind::Integer
        | PropertyKind::Number
        | PropertyKind::ForeignKeyNumeric
        | PropertyKind::Enum { .. } => {
            writeln!(out, "    if let Some(v) = {access} {{").unwrap();
            writeln!(out, "        object.insert({key:?}.to_owned(), json!(v));").unwrap();
            out.push_str("    }\n");
        }
        PropertyKind::ForeignKeyResolvable {
            api_path,
            lookup_field,
        } => {
            writeln!(out, "    if let Some(v) = {access} {{").unwrap();
            writeln!(
                out,
                "        object.insert({key:?}.to_owned(), crate::commands::resolve_reference_id({api_path:?}, {lookup_field:?}, &v, opts).await?);"
            )
            .unwrap();
            out.push_str("    }\n");
        }
        PropertyKind::TagArray => {
            writeln!(out, "    if let Some(v) = {access} {{").unwrap();
            writeln!(
                out,
                "        object.insert({key:?}.to_owned(), tags_value(&v));"
            )
            .unwrap();
            out.push_str("    }\n");
        }
        PropertyKind::Skip => {}
    }
}

pub fn extract_endpoint_metadata(schema_text: &str) -> Result<OpenApiMetadata> {
    let root_value =
        serde_json::from_str::<Value>(schema_text).context("failed to parse OpenAPI JSON")?;
    let root_object = require_object(&root_value, "$")?;

    let openapi_version = require_string_field(root_object, "openapi", "$")?;
    if openapi_version != SUPPORTED_OPENAPI_VERSION {
        bail!(
            "unsupported OpenAPI version at $.openapi: expected {SUPPORTED_OPENAPI_VERSION}, found {openapi_version}",
        );
    }

    let paths_value = require_field(root_object, "paths", "$")?;
    let paths_object = require_object(paths_value, "$.paths")?;
    let mut path_entries = paths_object.iter().collect::<Vec<_>>();
    path_entries.sort_by_key(|(path, _)| path.as_str());

    let mut endpoints = Vec::new();
    for (path, path_item_value) in path_entries {
        let path_json_path = format!("$.paths.{path}");
        let path_item_object = require_object(path_item_value, &path_json_path)?;

        for http_method in [
            HttpMethod::Get,
            HttpMethod::Post,
            HttpMethod::Patch,
            HttpMethod::Delete,
        ] {
            if let Some(operation_value) = path_item_object.get(http_method.as_schema_key()) {
                endpoints.push(extract_operation_metadata(
                    path,
                    http_method,
                    operation_value,
                    &format!("{path_json_path}.{}", http_method.as_schema_key()),
                )?);
            }
        }
    }

    Ok(OpenApiMetadata {
        openapi_version: openapi_version.to_owned(),
        endpoints,
    })
}

pub fn filter_endpoint_metadata(
    endpoint_metadata: &[EndpointMetadata],
    target_paths: &[&str],
) -> Result<Vec<EndpointMetadata>> {
    let mut filtered_endpoint_metadata = Vec::new();

    for target_path in target_paths {
        let mut matching_endpoints = endpoint_metadata
            .iter()
            .filter(|endpoint| endpoint.path == *target_path)
            .cloned()
            .collect::<Vec<_>>();

        if matching_endpoints.is_empty() {
            bail!("missing endpoint metadata for target path {target_path}");
        }

        matching_endpoints.sort_by(|left_endpoint, right_endpoint| {
            left_endpoint.method.cmp(&right_endpoint.method)
        });

        for matching_endpoint in matching_endpoints {
            if filtered_endpoint_metadata
                .iter()
                .any(|existing_endpoint: &EndpointMetadata| {
                    existing_endpoint.path == matching_endpoint.path
                        && existing_endpoint.method == matching_endpoint.method
                })
            {
                let method = matching_endpoint.method;
                let path = matching_endpoint.path;
                bail!("duplicate endpoint metadata for {method:?} {path}");
            }

            filtered_endpoint_metadata.push(matching_endpoint);
        }
    }

    Ok(filtered_endpoint_metadata)
}

pub fn render_endpoint_descriptors(endpoint_metadata: &[EndpointMetadata]) -> String {
    let mut rendered_source = String::from(concat!(
        "#[derive(Debug, Clone, Copy, Eq, PartialEq)]\n",
        "pub enum HttpMethod {\n",
        "    Get,\n",
        "    Post,\n",
        "    Patch,\n",
        "    Delete,\n",
        "}\n\n",
        "#[derive(Debug, Clone, Copy, Eq, PartialEq)]\n",
        "pub struct ParameterDescriptor {\n",
        "    pub name: &'static str,\n",
        "    pub location: &'static str,\n",
        "    pub required: bool,\n",
        "    pub description: Option<&'static str>,\n",
        "    pub schema_refs: &'static [&'static str],\n",
        "}\n\n",
        "#[derive(Debug, Clone, Copy, Eq, PartialEq)]\n",
        "pub struct ResponseDescriptor {\n",
        "    pub status_code: &'static str,\n",
        "    pub refs: &'static [&'static str],\n",
        "}\n\n",
        "#[derive(Debug, Clone, Copy, Eq, PartialEq)]\n",
        "pub struct EndpointDescriptor {\n",
        "    pub method: HttpMethod,\n",
        "    pub operation_id: &'static str,\n",
        "    pub path: &'static str,\n",
        "    pub parameters: &'static [ParameterDescriptor],\n",
        "    pub request_body_refs: &'static [&'static str],\n",
        "    pub response_refs: &'static [ResponseDescriptor],\n",
        "}\n\n",
        "#[rustfmt::skip]\n",
        "pub const ENDPOINTS: &[EndpointDescriptor] = &[\n",
    ));

    for endpoint in endpoint_metadata {
        rendered_source.push_str("    EndpointDescriptor {\n");
        writeln!(
            rendered_source,
            "        method: HttpMethod::{},",
            endpoint.method.as_generated_name()
        )
        .expect("writing to String never fails");
        writeln!(
            rendered_source,
            "        operation_id: {},",
            rust_string_literal(&endpoint.operation_id)
        )
        .expect("writing to String never fails");
        writeln!(
            rendered_source,
            "        path: {},",
            rust_string_literal(&endpoint.path)
        )
        .expect("writing to String never fails");

        if endpoint.parameters.is_empty() {
            rendered_source.push_str("        parameters: &[],\n");
        } else {
            rendered_source.push_str("        parameters: &[\n");
            for parameter in &endpoint.parameters {
                rendered_source.push_str("            ParameterDescriptor {\n");
                writeln!(
                    rendered_source,
                    "                name: {},",
                    rust_string_literal(&parameter.name)
                )
                .expect("writing to String never fails");
                writeln!(
                    rendered_source,
                    "                location: {},",
                    rust_string_literal(&parameter.location)
                )
                .expect("writing to String never fails");
                writeln!(
                    rendered_source,
                    "                required: {},",
                    parameter.required
                )
                .expect("writing to String never fails");
                writeln!(
                    rendered_source,
                    "                description: {},",
                    optional_rust_string_literal(parameter.description.as_deref())
                )
                .expect("writing to String never fails");
                writeln!(
                    rendered_source,
                    "                schema_refs: {},",
                    rust_string_slice_literal(&parameter.schema_refs)
                )
                .expect("writing to String never fails");
                rendered_source.push_str("            },\n");
            }
            rendered_source.push_str("        ],\n");
        }

        writeln!(
            rendered_source,
            "        request_body_refs: {},",
            rust_string_slice_literal(&endpoint.request_body_refs)
        )
        .expect("writing to String never fails");

        if endpoint.response_refs.is_empty() {
            rendered_source.push_str("        response_refs: &[],\n");
        } else {
            rendered_source.push_str("        response_refs: &[\n");
            for response in &endpoint.response_refs {
                rendered_source.push_str("            ResponseDescriptor {\n");
                writeln!(
                    rendered_source,
                    "                status_code: {},",
                    rust_string_literal(&response.status_code)
                )
                .expect("writing to String never fails");
                writeln!(
                    rendered_source,
                    "                refs: {},",
                    rust_string_slice_literal(&response.refs)
                )
                .expect("writing to String never fails");
                rendered_source.push_str("            },\n");
            }
            rendered_source.push_str("        ],\n");
        }

        rendered_source.push_str("    },\n");
    }

    rendered_source.push_str("];\n");
    rendered_source
}

pub fn render_typify_types(schema_text: &str) -> Result<String> {
    let openapi_value =
        serde_json::from_str::<Value>(schema_text).context("failed to parse schema JSON")?;
    let schemas = openapi_value
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .context("missing OpenAPI components.schemas")?;

    let mut definitions = schemars::Map::new();
    for (schema_name, schema_value) in schemas {
        let mut converted_schema = schema_value.clone();
        normalize_openapi_schema(&mut converted_schema);
        let schema = serde_json::from_value::<schemars::schema::Schema>(converted_schema)
            .with_context(|| {
                format!("failed to deserialize converted JSON schema for component {schema_name}")
            })?;
        definitions.insert(schema_name.clone(), schema);
    }

    let root_schema = RootSchema {
        meta_schema: Some("http://json-schema.org/draft-07/schema#".to_owned()),
        schema: schemars::schema::SchemaObject::default(),
        definitions,
    };

    let mut type_space = TypeSpace::new(&TypeSpaceSettings::default());
    type_space
        .add_root_schema(root_schema)
        .context("typify failed")?;

    let generated_tokens = type_space.to_stream();
    let syntax_tree = syn::parse2(generated_tokens).context("failed to parse generated Rust")?;
    let mut generated_source = String::from(
        "#![allow(clippy::derivable_impls)]\n\
         #![allow(clippy::derive_partial_eq_without_eq)]\n\
         #![allow(clippy::large_enum_variant)]\n\
         #![allow(clippy::len_zero)]\n\
         #![allow(clippy::too_many_lines)]\n\n",
    );
    generated_source.push_str(&prettyplease::unparse(&syntax_tree));
    Ok(generated_source)
}

fn normalize_openapi_schema(value: &mut Value) {
    match value {
        Value::Object(object) => {
            if let Some(Value::String(reference)) = object.get_mut("$ref")
                && let Some(schema_name) = reference.strip_prefix("#/components/schemas/")
            {
                *reference = format!("#/definitions/{schema_name}");
            }

            let is_nullable = object
                .remove("nullable")
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            let has_null_enum_value = object
                .get("enum")
                .and_then(Value::as_array)
                .is_some_and(|enum_values| enum_values.iter().any(Value::is_null));
            let is_nullable = is_nullable || has_null_enum_value;
            object.remove("enum");

            normalize_exclusive_bound(object, "maximum", "exclusiveMaximum");
            normalize_exclusive_bound(object, "minimum", "exclusiveMinimum");

            for key in [
                "discriminator",
                "example",
                "externalDocs",
                "readOnly",
                "writeOnly",
                "xml",
                "x-spec-enum-id",
            ] {
                object.remove(key);
            }

            for child_value in object.values_mut() {
                normalize_openapi_schema(child_value);
            }

            if is_nullable {
                let mut non_null_schema = Value::Object(std::mem::take(object));
                if non_null_schema == json!({}) {
                    non_null_schema = json!({});
                }
                *value = json!({
                    "anyOf": [
                        non_null_schema,
                        { "type": "null" }
                    ]
                });
            }
        }
        Value::Array(values) => {
            for child_value in values {
                normalize_openapi_schema(child_value);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

fn normalize_exclusive_bound(
    object: &mut Map<String, Value>,
    bound_key: &'static str,
    exclusive_bound_key: &'static str,
) {
    let Some(is_exclusive) = object.get(exclusive_bound_key).and_then(Value::as_bool) else {
        return;
    };

    object.remove(exclusive_bound_key);
    if is_exclusive && let Some(bound_value) = object.remove(bound_key) {
        object.insert(exclusive_bound_key.to_owned(), bound_value);
    }
}

fn extract_operation_metadata(
    path: &str,
    method: HttpMethod,
    operation_value: &Value,
    operation_json_path: &str,
) -> Result<EndpointMetadata> {
    let operation_object = require_object(operation_value, operation_json_path)?;
    let operation_id = require_string_field(operation_object, "operationId", operation_json_path)?;
    let parameters = extract_parameters(operation_object, operation_json_path)?;
    let request_body_refs =
        extract_optional_request_body_refs(operation_object, operation_json_path)?;
    let response_refs = extract_response_refs(operation_object, operation_json_path)?;

    Ok(EndpointMetadata {
        path: path.to_owned(),
        method,
        operation_id: operation_id.to_owned(),
        parameters,
        request_body_refs,
        response_refs,
    })
}

fn rust_string_literal(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization should not fail")
}

fn optional_rust_string_literal(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("Some({})", rust_string_literal(value)),
        None => "None".to_owned(),
    }
}

fn rust_string_slice_literal(values: &[String]) -> String {
    if values.is_empty() {
        return "&[]".to_owned();
    }

    let values = values
        .iter()
        .map(|value| rust_string_literal(value))
        .collect::<Vec<_>>()
        .join(", ");

    format!("&[{values}]")
}

fn extract_parameters(
    operation_object: &Map<String, Value>,
    operation_json_path: &str,
) -> Result<Vec<ParameterMetadata>> {
    let Some(parameters_value) = operation_object.get("parameters") else {
        return Ok(Vec::new());
    };

    let parameter_values = parameters_value.as_array().with_context(|| {
        format!(
            "unsupported schema shape at {operation_json_path}.parameters: expected an array of parameter objects",
        )
    })?;

    parameter_values
        .iter()
        .enumerate()
        .map(|(parameter_index, parameter_value)| {
            let parameter_json_path =
                format!("{operation_json_path}.parameters[{parameter_index}]");
            extract_parameter(parameter_value, &parameter_json_path)
        })
        .collect()
}

fn extract_parameter(
    parameter_value: &Value,
    parameter_json_path: &str,
) -> Result<ParameterMetadata> {
    let parameter_object = require_object(parameter_value, parameter_json_path)?;
    if parameter_object.contains_key("$ref") {
        bail!(
            "unsupported schema shape at {parameter_json_path}: parameter references are not supported by the v0.1 extractor",
        );
    }

    let name = require_string_field(parameter_object, "name", parameter_json_path)?;
    let location = require_string_field(parameter_object, "in", parameter_json_path)?;
    let required = match parameter_object.get("required") {
        Some(value) => value.as_bool().with_context(|| {
            format!(
                "unsupported schema shape at {parameter_json_path}.required: expected a boolean"
            )
        })?,
        None => false,
    };
    let description = parameter_object
        .get("description")
        .map(|description_value| {
            description_value
                .as_str()
                .map(ToOwned::to_owned)
                .with_context(|| {
                    format!(
                        "unsupported schema shape at {parameter_json_path}.description: expected a string",
                    )
                })
        })
        .transpose()?;
    let schema = require_field(parameter_object, "schema", parameter_json_path)?.clone();
    let schema_refs = collect_refs(&schema);

    Ok(ParameterMetadata {
        name: name.to_owned(),
        location: location.to_owned(),
        required,
        description,
        schema,
        schema_refs,
    })
}

fn extract_optional_request_body_refs(
    operation_object: &Map<String, Value>,
    operation_json_path: &str,
) -> Result<Vec<String>> {
    let Some(request_body_value) = operation_object.get("requestBody") else {
        return Ok(Vec::new());
    };

    let request_body_json_path = format!("{operation_json_path}.requestBody");
    let request_body_object = require_object(request_body_value, &request_body_json_path)?;

    if let Some(reference_value) = request_body_object.get("$ref") {
        let reference = reference_value.as_str().with_context(|| {
            format!(
                "unsupported schema shape at {request_body_json_path}.$ref: expected a string reference",
            )
        })?;
        return Ok(vec![reference.to_owned()]);
    }

    let content_value = require_field(request_body_object, "content", &request_body_json_path)?;
    Ok(collect_refs(content_value))
}

fn extract_response_refs(
    operation_object: &Map<String, Value>,
    operation_json_path: &str,
) -> Result<Vec<ResponseMetadata>> {
    let responses_value = require_field(operation_object, "responses", operation_json_path)?;
    let responses_json_path = format!("{operation_json_path}.responses");
    let responses_object = require_object(responses_value, &responses_json_path)?;

    if responses_object.is_empty() {
        bail!(
            "unsupported schema shape at {responses_json_path}: expected at least one response entry",
        );
    }

    let mut response_entries = responses_object.iter().collect::<Vec<_>>();
    response_entries.sort_by(|(left_status_code, _), (right_status_code, _)| {
        left_status_code.cmp(right_status_code)
    });

    response_entries
        .into_iter()
        .map(|(status_code, response_value)| {
            let response_json_path = format!("{operation_json_path}.responses.{status_code}");
            extract_response_metadata(status_code, response_value, &response_json_path)
        })
        .collect()
}

fn extract_response_metadata(
    status_code: &str,
    response_value: &Value,
    response_json_path: &str,
) -> Result<ResponseMetadata> {
    let response_object = require_object(response_value, response_json_path)?;

    let refs = if let Some(reference_value) = response_object.get("$ref") {
        let reference = reference_value.as_str().with_context(|| {
            format!(
                "unsupported schema shape at {response_json_path}.$ref: expected a string reference",
            )
        })?;
        vec![reference.to_owned()]
    } else {
        response_object
            .get("content")
            .map_or_else(Vec::new, collect_refs)
    };

    Ok(ResponseMetadata {
        status_code: status_code.to_owned(),
        refs,
    })
}

fn collect_refs(value: &Value) -> Vec<String> {
    fn collect_refs_into(value: &Value, refs: &mut BTreeSet<String>) {
        match value {
            Value::Object(object) => {
                if let Some(Value::String(reference)) = object.get("$ref") {
                    refs.insert(reference.to_owned());
                }

                for child_value in object.values() {
                    collect_refs_into(child_value, refs);
                }
            }
            Value::Array(values) => {
                for child_value in values {
                    collect_refs_into(child_value, refs);
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
        }
    }

    let mut refs = BTreeSet::new();
    collect_refs_into(value, &mut refs);
    refs.into_iter().collect()
}

fn require_field<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
    json_path: &str,
) -> Result<&'a Value> {
    object
        .get(field)
        .with_context(|| format!("missing required field `{field}` at {json_path}"))
}

fn require_string_field<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
    json_path: &str,
) -> Result<&'a str> {
    require_field(object, field, json_path)?
        .as_str()
        .with_context(|| {
            format!("unsupported schema shape at {json_path}.{field}: expected a string")
        })
}

fn require_object<'a>(value: &'a Value, json_path: &str) -> Result<&'a Map<String, Value>> {
    value
        .as_object()
        .with_context(|| format!("unsupported schema shape at {json_path}: expected an object"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_openapi_303() {
        let schema_text = r#"{
            "openapi": "3.0.3",
            "paths": {}
        }"#;

        let metadata = extract_endpoint_metadata(schema_text).expect("schema should parse");

        assert_eq!(metadata.openapi_version, "3.0.3");
        assert!(metadata.endpoints.is_empty());
    }

    #[test]
    fn rejects_unsupported_openapi_versions() {
        let schema_text = r#"{
            "openapi": "3.1.0",
            "paths": {}
        }"#;

        let error = extract_endpoint_metadata(schema_text).expect_err("schema should fail");

        assert_eq!(
            error.to_string(),
            "unsupported OpenAPI version at $.openapi: expected 3.0.3, found 3.1.0"
        );
    }

    #[test]
    fn extracts_representative_endpoint_metadata() {
        let schema_text = r##"{
            "openapi": "3.0.3",
            "paths": {
                "/api/dcim/devices/": {
                    "get": {
                        "operationId": "dcim_devices_list",
                        "parameters": [
                            {
                                "name": "limit",
                                "in": "query",
                                "required": false,
                                "description": "Number of results to return per page.",
                                "schema": { "type": "integer" }
                            },
                            {
                                "name": "site",
                                "in": "query",
                                "schema": {
                                    "$ref": "#/components/schemas/SiteFilter"
                                }
                            }
                        ],
                        "responses": {
                            "200": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/PaginatedDeviceList"
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "post": {
                        "operationId": "dcim_devices_create",
                        "requestBody": {
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/WritableDevice"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "201": {
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/Device"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }"##;

        let metadata = extract_endpoint_metadata(schema_text).expect("schema should parse");

        assert_eq!(metadata.endpoints.len(), 2);
        assert_eq!(metadata.endpoints[0].path, "/api/dcim/devices/");
        assert_eq!(metadata.endpoints[0].method, HttpMethod::Get);
        assert_eq!(metadata.endpoints[0].operation_id, "dcim_devices_list");
        assert_eq!(metadata.endpoints[0].parameters[0].name, "limit");
        assert!(!metadata.endpoints[0].parameters[1].required);
        assert_eq!(
            metadata.endpoints[0].parameters[1].schema_refs,
            vec!["#/components/schemas/SiteFilter"]
        );
        assert_eq!(
            metadata.endpoints[0].response_refs[0].refs,
            vec!["#/components/schemas/PaginatedDeviceList"]
        );
        assert_eq!(
            metadata.endpoints[1].request_body_refs,
            vec!["#/components/schemas/WritableDevice"]
        );
    }

    #[test]
    fn fails_clearly_when_required_operation_metadata_is_missing() {
        let schema_text = r#"{
            "openapi": "3.0.3",
            "paths": {
                "/api/dcim/devices/": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": ""
                            }
                        }
                    }
                }
            }
        }"#;

        let error = extract_endpoint_metadata(schema_text).expect_err("schema should fail");

        assert_eq!(
            error.to_string(),
            "missing required field `operationId` at $.paths./api/dcim/devices/.get"
        );
    }

    #[test]
    fn extracts_v0_1_metadata_from_pinned_netbox_schema() {
        let schema_text = include_str!("../../../schema/netbox-4.5.10.json");
        let metadata = extract_endpoint_metadata(schema_text).expect("schema should parse");
        let target_endpoint_metadata =
            filter_endpoint_metadata(&metadata.endpoints, V0_1_TARGET_ENDPOINT_PATHS)
                .expect("target endpoint metadata should exist");

        assert_eq!(metadata.openapi_version, SUPPORTED_OPENAPI_VERSION);
        assert_eq!(target_endpoint_metadata.len(), 175);

        let devices_list_endpoint = target_endpoint_metadata
            .iter()
            .find(|endpoint| {
                endpoint.path == "/api/dcim/devices/" && endpoint.method == HttpMethod::Get
            })
            .expect("devices list endpoint should exist");

        assert_eq!(devices_list_endpoint.operation_id, "dcim_devices_list");
        assert!(
            devices_list_endpoint
                .parameters
                .iter()
                .any(|parameter| parameter.name == "limit" && parameter.location == "query")
        );
        assert_eq!(
            devices_list_endpoint.response_refs[0].refs,
            vec!["#/components/schemas/PaginatedDeviceWithConfigContextList"]
        );

        let vlans_patch_endpoint = target_endpoint_metadata
            .iter()
            .find(|endpoint| {
                endpoint.path == "/api/ipam/vlans/{id}/" && endpoint.method == HttpMethod::Patch
            })
            .expect("VLAN patch endpoint should exist");

        assert_eq!(
            vlans_patch_endpoint.request_body_refs,
            vec!["#/components/schemas/PatchedWritableVLANRequest"]
        );
    }

    /// Every entry in [`FK_RESOLVERS`] must point at a real `NetBox` endpoint
    /// whose GET operation accepts the declared lookup field as a query
    /// parameter. Without this guard a `NetBox` release that renames an
    /// endpoint or filter could silently break name-resolved FK flags
    /// (`--site dc1`, `--device-type acme-server`, etc.) — the build would
    /// still succeed but `cargo run -- ...` would 404 at runtime.
    #[test]
    fn fk_resolvers_match_pinned_schema() {
        let schema_text = include_str!("../../../schema/netbox-4.5.10.json");
        let metadata = extract_endpoint_metadata(schema_text).expect("schema should parse");

        for (brief_schema, api_path, lookup_field) in FK_RESOLVERS {
            let endpoint = metadata
                .endpoints
                .iter()
                .find(|endpoint| endpoint.path == *api_path && endpoint.method == HttpMethod::Get);
            let endpoint = endpoint.unwrap_or_else(|| {
                panic!(
                    "FK_RESOLVERS entry for {brief_schema} points at {api_path}, \
                     but the pinned schema has no GET operation at that path"
                )
            });

            let has_lookup_field = endpoint
                .parameters
                .iter()
                .any(|parameter| parameter.name == *lookup_field && parameter.location == "query");
            assert!(
                has_lookup_field,
                "FK_RESOLVERS entry for {brief_schema} declares lookup field {lookup_field:?}, \
                 but {api_path} GET does not expose that as a query parameter"
            );
        }
    }

    // ============================================================
    // Generated-code drift checks
    //
    // These three tests assert that every committed file under
    // `src/generated/` is byte-identical to what the codegen would
    // produce from the pinned schema. They replace the old `build.rs`
    // drift guard, which made `nbx` depend on `nbx-codegen` as a
    // [build-dependencies] path dep — that prevented publishing nbx to
    // crates.io. Moving the checks here keeps developer-experience
    // (running `cargo test --workspace` catches stale generated files)
    // without baking the codegen helper into the install path.
    //
    // To regenerate after touching codegen or bumping the schema:
    //   cargo run -p nbx-codegen -- schema/netbox-X.Y.Z.json src/generated/endpoints.rs
    //   cargo run -p nbx-codegen -- schema/netbox-X.Y.Z.json src/generated/types.rs
    //   cargo run -p nbx-codegen -- schema/netbox-X.Y.Z.json src/generated/resources/
    // ============================================================

    #[test]
    fn endpoints_rs_matches_pinned_schema() {
        let schema_text = include_str!("../../../schema/netbox-4.5.10.json");
        let committed = include_str!("../../../src/generated/endpoints.rs");

        let metadata = extract_endpoint_metadata(schema_text).expect("pinned schema should parse");
        let target_endpoints =
            filter_endpoint_metadata(&metadata.endpoints, V0_1_TARGET_ENDPOINT_PATHS)
                .expect("v0.1 target endpoint metadata should resolve");
        let regenerated = render_endpoint_descriptors(&target_endpoints);

        assert_eq!(
            committed, regenerated,
            "src/generated/endpoints.rs is stale; \
             run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/endpoints.rs`"
        );
    }

    #[test]
    fn types_rs_matches_pinned_schema() {
        let schema_text = include_str!("../../../schema/netbox-4.5.10.json");
        let committed = include_str!("../../../src/generated/types.rs");

        let regenerated =
            render_typify_types(schema_text).expect("typify generation should succeed");

        assert_eq!(
            committed, regenerated,
            "src/generated/types.rs is stale; \
             run `cargo run -p nbx-codegen -- schema/netbox-4.5.10.json src/generated/types.rs`"
        );
    }

    #[test]
    fn resource_modules_match_pinned_schema() {
        use std::path::Path;

        let schema_text = include_str!("../../../schema/netbox-4.5.10.json");
        let regenerated_outputs =
            render_resource_modules(schema_text).expect("resource modules should render");

        let resources_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("src")
            .join("generated")
            .join("resources");

        for (filename, regenerated) in &regenerated_outputs {
            let committed_path = resources_dir.join(filename);
            let committed = std::fs::read_to_string(&committed_path).unwrap_or_else(|error| {
                panic!(
                    "failed to read {}: {error}; run `cargo run -p nbx-codegen -- \
                     schema/netbox-4.5.10.json src/generated/resources/`",
                    committed_path.display()
                )
            });
            assert_eq!(
                &committed,
                regenerated,
                "{} is stale; run `cargo run -p nbx-codegen -- \
                 schema/netbox-4.5.10.json src/generated/resources/`",
                committed_path.display()
            );
        }
    }
}
