use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::{NbxError, NbxResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NbxConfig {
    pub url: Option<String>,
    pub token: Option<String>,
    pub default_context: Option<String>,
    #[serde(default)]
    pub contexts: BTreeMap<String, ConfigContext>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfigContext {
    pub url: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedConnection {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedOptionalConnection {
    pub url: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub url: Option<String>,
    pub token: Option<String>,
    pub context: Option<String>,
}

pub fn config_path() -> NbxResult<PathBuf> {
    let config_directory = dirs::config_dir()
        .ok_or_else(|| NbxError::general("failed to locate user config directory"))?
        .join("nbx");
    Ok(config_directory.join("config.toml"))
}

pub fn load_config() -> NbxResult<NbxConfig> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(NbxConfig::default());
    }

    warn_if_config_permissions_are_loose(&path);

    let config_text = fs::read_to_string(&path).map_err(|error| {
        NbxError::general(format!("failed to read config {}: {error}", path.display()))
    })?;
    toml::from_str(&config_text).map_err(|error| {
        NbxError::validation(
            format!("failed to parse config {}: {error}", path.display()),
            json!({ "path": path }),
        )
    })
}

pub fn save_config(config: &NbxConfig) -> NbxResult<()> {
    let path = config_path()?;
    if let Some(parent_directory) = path.parent() {
        fs::create_dir_all(parent_directory).map_err(|error| {
            NbxError::general(format!(
                "failed to create config directory {}: {error}",
                parent_directory.display()
            ))
        })?;
    }

    let config_text = toml::to_string_pretty(config)
        .map_err(|error| NbxError::general(format!("failed to serialize config: {error}")))?;
    fs::write(&path, config_text).map_err(|error| {
        NbxError::general(format!(
            "failed to write config {}: {error}",
            path.display()
        ))
    })?;
    set_config_permissions_private(&path)?;
    Ok(())
}

pub fn resolve_connection(overrides: &ConfigOverrides) -> NbxResult<ResolvedConnection> {
    let connection = resolve_optional_connection(overrides)?;
    let token = connection.token.ok_or_else(|| {
        NbxError::validation(
            "missing NetBox token; set --token, NETBOX_TOKEN, or nbx config set-token",
            json!({}),
        )
    })?;

    Ok(ResolvedConnection {
        url: connection.url,
        token,
    })
}

pub fn resolve_optional_connection(
    overrides: &ConfigOverrides,
) -> NbxResult<ResolvedOptionalConnection> {
    let config = load_config()?;
    let selected_context = overrides
        .context
        .clone()
        .or_else(|| env::var("NBX_CONTEXT").ok())
        .or_else(|| config.default_context.clone());
    let context = selected_context
        .as_ref()
        .and_then(|context_name| config.contexts.get(context_name));

    let url = overrides
        .url
        .clone()
        .or_else(|| env::var("NETBOX_URL").ok())
        .or_else(|| context.and_then(|context| context.url.clone()))
        .or_else(|| config.url.clone())
        .ok_or_else(|| {
            NbxError::validation(
                "missing NetBox URL; set --url, NETBOX_URL, or nbx config set-url",
                json!({}),
            )
        })?;

    let token = overrides
        .token
        .clone()
        .or_else(|| env::var("NETBOX_TOKEN").ok())
        .or_else(|| context.and_then(|context| context.token.clone()))
        .or_else(|| config.token.clone());

    Ok(ResolvedOptionalConnection {
        url: normalize_base_url(&url),
        token,
    })
}

pub fn normalize_base_url(url: &str) -> String {
    url.trim_end_matches('/').to_owned()
}

pub fn redact_token(token: &str) -> String {
    if token.len() <= 8 {
        return "********".to_owned();
    }

    format!("{}…{}", &token[..4], &token[token.len() - 4..])
}

#[cfg(unix)]
fn set_config_permissions_private(path: &PathBuf) -> NbxResult<()> {
    use std::os::unix::fs::PermissionsExt;

    let permissions = fs::Permissions::from_mode(0o600);
    fs::set_permissions(path, permissions).map_err(|error| {
        NbxError::general(format!(
            "failed to set config permissions on {}: {error}",
            path.display()
        ))
    })
}

#[cfg(not(unix))]
fn set_config_permissions_private(_path: &PathBuf) -> NbxResult<()> {
    Ok(())
}

#[cfg(unix)]
fn warn_if_config_permissions_are_loose(path: &PathBuf) {
    use std::os::unix::fs::PermissionsExt;

    let Ok(metadata) = fs::metadata(path) else {
        return;
    };
    let mode = metadata.permissions().mode() & 0o777;
    if mode & 0o077 != 0 {
        eprintln!(
            "warning: config file {} has broad permissions {:o}; expected 600",
            path.display(),
            mode
        );
    }
}

#[cfg(not(unix))]
fn warn_if_config_permissions_are_loose(_path: &PathBuf) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_base_url() {
        assert_eq!(
            normalize_base_url("https://netbox.example///"),
            "https://netbox.example"
        );
    }
}
