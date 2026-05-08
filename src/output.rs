use std::io::{self, IsTerminal};

use clap::ValueEnum;
use comfy_table::Table;
use serde_json::Value;

use crate::envelope::{data_envelope, list_envelope};
use crate::error::{NbxError, NbxResult};

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Table,
    Ndjson,
}

pub fn default_output_format() -> OutputFormat {
    if io::stdout().is_terminal() {
        OutputFormat::Table
    } else {
        OutputFormat::Json
    }
}

pub fn emit_single(value: &Value, output_format: OutputFormat) -> NbxResult<()> {
    let envelope = data_envelope(value.clone());
    match output_format {
        OutputFormat::Json => print_json(&envelope),
        OutputFormat::Ndjson => print_json_line(&envelope),
        OutputFormat::Table => print_table(value),
    }
}

pub fn emit_page(page: &Value, output_format: OutputFormat) -> NbxResult<()> {
    match output_format {
        OutputFormat::Json => print_json(&list_envelope(page)),
        OutputFormat::Ndjson => {
            for item in extract_results(page) {
                print_json_line(&data_envelope(item.clone()))?;
            }
            Ok(())
        }
        OutputFormat::Table => print_table(page),
    }
}

pub fn print_json(value: &Value) -> NbxResult<()> {
    let json_text = serde_json::to_string_pretty(value)
        .map_err(|error| NbxError::general(format!("failed to serialize output: {error}")))?;
    println!("{json_text}");
    Ok(())
}

pub fn print_json_line(value: &Value) -> NbxResult<()> {
    let json_text = serde_json::to_string(value)
        .map_err(|error| NbxError::general(format!("failed to serialize output: {error}")))?;
    println!("{json_text}");
    Ok(())
}

pub fn print_table(value: &Value) -> NbxResult<()> {
    let rows = extract_results(value);
    if rows.is_empty() {
        print_json(&data_envelope(value.clone()))?;
        return Ok(());
    }

    let headers = table_headers(rows);
    let mut table = Table::new();
    table.set_header(headers.clone());

    for row in rows {
        table.add_row(
            headers
                .iter()
                .map(|header| {
                    row.get(header.as_str())
                        .map_or_else(String::new, table_cell)
                })
                .collect::<Vec<_>>(),
        );
    }

    println!("{table}");
    if let Some(count) = value.get("count") {
        println!("count: {count}");
    }

    Ok(())
}

pub fn extract_results(value: &Value) -> &[Value] {
    if let Some(results) = value.get("results").and_then(Value::as_array) {
        return results;
    }

    if let Some(array) = value.as_array() {
        return array;
    }

    std::slice::from_ref(value)
}

fn table_headers(rows: &[Value]) -> Vec<String> {
    let mut headers = Vec::new();
    for row in rows.iter().filter_map(Value::as_object) {
        for key in row.keys() {
            if !headers.contains(key) {
                headers.push(key.to_owned());
            }
            if headers.len() >= 8 {
                return headers;
            }
        }
    }
    headers
}

fn table_cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(value) => value.clone(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::Array(_) | Value::Object(_) => serde_json::to_string(value).unwrap_or_default(),
    }
}
