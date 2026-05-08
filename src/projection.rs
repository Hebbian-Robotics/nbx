use serde_json::{Map, Value};

pub fn parse_field_projection(field_projection: Option<&str>, name_only: bool) -> Vec<String> {
    if name_only {
        return vec!["name".to_owned()];
    }

    field_projection
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|field| !field.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub fn apply_projection(value: &Value, fields: &[String]) -> Value {
    if fields.is_empty() {
        return value.clone();
    }

    match value {
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|item| apply_projection_to_object(item, fields))
                .collect(),
        ),
        Value::Object(object) if object.contains_key("results") => {
            let mut projected_page = object.clone();
            if let Some(results) = object.get("results").and_then(Value::as_array) {
                projected_page.insert(
                    "results".to_owned(),
                    Value::Array(
                        results
                            .iter()
                            .map(|item| apply_projection_to_object(item, fields))
                            .collect(),
                    ),
                );
            }
            Value::Object(projected_page)
        }
        Value::Object(_) => apply_projection_to_object(value, fields),
        _ => value.clone(),
    }
}

fn apply_projection_to_object(value: &Value, fields: &[String]) -> Value {
    let mut projected = Map::new();

    for field in fields {
        if let Some(projected_value) = get_dot_path(value, field) {
            insert_dot_path(&mut projected, field, projected_value.clone());
        }
    }

    Value::Object(projected)
}

fn get_dot_path<'a>(value: &'a Value, field: &str) -> Option<&'a Value> {
    let mut current_value = value;
    for segment in field.split('.') {
        current_value = current_value.get(segment)?;
    }
    Some(current_value)
}

fn insert_dot_path(object: &mut Map<String, Value>, field: &str, value: Value) {
    let mut segments = field.split('.').peekable();
    let mut current_object = object;

    while let Some(segment) = segments.next() {
        if segments.peek().is_none() {
            current_object.insert(segment.to_owned(), value);
            return;
        }

        current_object = current_object
            .entry(segment.to_owned())
            .or_insert_with(|| Value::Object(Map::new()))
            .as_object_mut()
            .expect("projection path should only create objects");
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn projects_nested_fields() {
        let value = json!({"name": "srv01", "primary_ip4": {"address": "192.0.2.1/32"}, "id": 1});
        let fields = vec!["name".to_owned(), "primary_ip4.address".to_owned()];

        assert_eq!(
            apply_projection(&value, &fields),
            json!({"name": "srv01", "primary_ip4": {"address": "192.0.2.1/32"}})
        );
    }
}
