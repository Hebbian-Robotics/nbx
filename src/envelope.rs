use serde_json::{Map, Value, json};

use crate::error::NbxError;

pub const SCHEMA_VERSION: u64 = 1;

fn base_envelope() -> Map<String, Value> {
    let mut envelope = Map::new();
    envelope.insert("nbxVersion".to_owned(), json!(env!("CARGO_PKG_VERSION")));
    envelope.insert("schemaVersion".to_owned(), json!(SCHEMA_VERSION));
    envelope
}

pub fn data_envelope(data: Value) -> Value {
    let mut envelope = base_envelope();
    envelope.insert("data".to_owned(), data);
    Value::Object(envelope)
}

pub fn list_envelope(page: &Value) -> Value {
    let mut envelope = base_envelope();

    if let Some(object) = page.as_object() {
        for key in ["count", "next", "previous", "results"] {
            if let Some(value) = object.get(key) {
                envelope.insert(key.to_owned(), value.clone());
            }
        }
        if !object.contains_key("results") {
            envelope.insert("data".to_owned(), page.clone());
        }
    } else {
        envelope.insert("results".to_owned(), page.clone());
    }

    Value::Object(envelope)
}

pub fn error_envelope(error: &NbxError) -> Value {
    let mut envelope = base_envelope();
    envelope.insert(
        "error".to_owned(),
        json!({
            "code": error.code.as_str(),
            "message": error.message,
            "detail": error.detail,
        }),
    );
    Value::Object(envelope)
}
