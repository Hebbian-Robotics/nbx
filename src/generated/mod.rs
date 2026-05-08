#![allow(dead_code, clippy::pedantic)]

#[rustfmt::skip]
pub mod endpoints;
#[rustfmt::skip]
pub mod resources;
#[rustfmt::skip]
pub mod types;

#[cfg(test)]
mod endpoint_snapshot;

impl endpoints::HttpMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}
