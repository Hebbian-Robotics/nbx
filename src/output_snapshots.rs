//! Snapshot tests for the agent-facing output contract.
//!
//! Snapshot names are suffixed with the current `schemaVersion` so any change
//! to the pinned shape forces a same-PR `schemaVersion` bump. The
//! `nbxVersion` field is normalized away from the snapshots so crate version
//! bumps don't churn pinned output.

use std::fmt::Write;

use comfy_table::Table;
use insta::with_settings;
use serde_json::{Value, json};

use crate::envelope::{SCHEMA_VERSION, data_envelope, error_envelope, list_envelope};
use crate::error::{ErrorCode, NbxError};

fn schema_version_suffix() -> String {
    format!("schema_v{SCHEMA_VERSION}")
}

fn redact_nbx_version(mut envelope: Value) -> Value {
    if let Some(object) = envelope.as_object_mut()
        && let Some(version_value) = object.get_mut("nbxVersion")
    {
        *version_value = json!("[redacted]");
    }
    envelope
}

fn render_table_string(value: &Value) -> String {
    let rows = crate::output::extract_results(value);
    let mut headers: Vec<String> = Vec::new();
    for row in rows.iter().filter_map(Value::as_object) {
        for key in row.keys() {
            if !headers.contains(key) {
                headers.push(key.clone());
            }
        }
    }

    let mut table = Table::new();
    table.set_header(headers.clone());
    for row in rows {
        table.add_row(headers.iter().map(|header| {
            row.get(header.as_str())
                .map(|value| match value {
                    Value::Null => String::new(),
                    Value::String(text) => text.clone(),
                    other => other.to_string(),
                })
                .unwrap_or_default()
        }));
    }

    let mut rendered = table.to_string();
    if let Some(count) = value.get("count") {
        write!(rendered, "\ncount: {count}").expect("writing to String never fails");
    }
    rendered
}

#[test]
fn data_envelope_for_single_resource() {
    let envelope = redact_nbx_version(data_envelope(json!({
        "id": 7,
        "name": "srv-db-07",
        "status": { "value": "active", "label": "Active" },
    })));

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_json_snapshot!("data_envelope_single_device", envelope);
    });
}

#[test]
fn list_envelope_for_paginated_page() {
    let page = json!({
        "count": 2,
        "next": "https://netbox.example/api/dcim/devices/?limit=2&offset=2",
        "previous": null,
        "results": [
            { "id": 1, "name": "srv-db-01" },
            { "id": 2, "name": "srv-db-02" },
        ],
    });

    let envelope = redact_nbx_version(list_envelope(&page));

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_json_snapshot!("list_envelope_devices_page", envelope);
    });
}

#[test]
fn error_envelope_for_each_error_class() {
    let cases = [
        (
            "not_found",
            NbxError::not_found(
                "device not found: srv-db-07",
                json!({ "resource": "dcim.device", "lookup": "srv-db-07" }),
            ),
        ),
        (
            "auth_failed",
            NbxError::new(
                ErrorCode::AuthFailed,
                "NetBox authentication failed",
                json!({ "status": 401 }),
            ),
        ),
        (
            "validation_error",
            NbxError::validation(
                "NetBox validation failed",
                json!({ "status": 422, "response": { "name": ["This field is required."] } }),
            ),
        ),
        (
            "stream_error",
            NbxError::stream(
                "failed while streaming paginated results",
                json!({ "resource": "dcim.devices", "source": "request failed" }),
            ),
        ),
        ("general", NbxError::general("NetBox request failed")),
    ];

    let envelopes: Vec<Value> = cases
        .iter()
        .map(|(_, error)| redact_nbx_version(error_envelope(error)))
        .collect();

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_json_snapshot!("error_envelopes_all_classes", envelopes);
    });
}

#[test]
fn ndjson_line_for_streamed_record() {
    let envelope = redact_nbx_version(data_envelope(json!({
        "id": 1,
        "name": "srv-db-01",
    })));
    let line = serde_json::to_string(&envelope).expect("envelope should serialize");

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_snapshot!("ndjson_line_streamed_device", line);
    });
}

#[test]
fn table_rendering_for_paginated_page() {
    let page = json!({
        "count": 2,
        "results": [
            { "id": 1, "name": "srv-db-01", "status": "active" },
            { "id": 2, "name": "srv-db-02", "status": "planned" },
        ],
    });

    let rendered = render_table_string(&page);

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_snapshot!("table_devices_page", rendered);
    });
}

#[test]
fn list_envelope_for_interface_page() {
    let page = json!({
        "count": 2,
        "next": null,
        "previous": null,
        "results": [
            {
                "id": 91,
                "name": "Ethernet1",
                "type": { "value": "10gbase-t", "label": "10GBASE-T (10GE)" },
                "enabled": true,
                "mgmt_only": false,
                "device": { "id": 7, "name": "srv01" },
            },
            {
                "id": 92,
                "name": "Ethernet2",
                "type": { "value": "10gbase-t", "label": "10GBASE-T (10GE)" },
                "enabled": false,
                "mgmt_only": false,
                "device": { "id": 7, "name": "srv01" },
            },
        ],
    });

    let envelope = redact_nbx_version(list_envelope(&page));

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_json_snapshot!("list_envelope_interfaces_page", envelope);
    });
}

#[test]
fn data_envelope_for_interface_with_vlan_bindings() {
    let envelope = redact_nbx_version(data_envelope(json!({
        "id": 91,
        "name": "Ethernet1",
        "type": { "value": "10gbase-t", "label": "10GBASE-T (10GE)" },
        "enabled": true,
        "mode": { "value": "tagged", "label": "Tagged" },
        "untagged_vlan": { "id": 100, "name": "mgmt", "vid": 100 },
        "tagged_vlans": [
            { "id": 200, "name": "dev", "vid": 200 },
            { "id": 201, "name": "prod", "vid": 201 },
        ],
        "device": { "id": 7, "name": "srv01" },
    })));

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_json_snapshot!("data_envelope_interface_with_vlans", envelope);
    });
}

#[test]
fn data_envelope_for_cable_trace_path() {
    let envelope = redact_nbx_version(data_envelope(json!([
        [
            { "object_type": "dcim.interface", "id": 91, "name": "Ethernet1" },
            { "object_type": "dcim.cable", "id": 5, "label": "patch-7" },
            { "object_type": "dcim.frontport", "id": 12, "name": "FrontPort1" },
        ],
        [
            { "object_type": "dcim.rearport", "id": 13, "name": "RearPort1" },
            { "object_type": "dcim.cable", "id": 6, "label": "trunk-3" },
            { "object_type": "dcim.interface", "id": 200, "name": "Ethernet48" },
        ]
    ])));

    with_settings!({snapshot_suffix => schema_version_suffix()}, {
        insta::assert_json_snapshot!("data_envelope_cable_trace", envelope);
    });
}
