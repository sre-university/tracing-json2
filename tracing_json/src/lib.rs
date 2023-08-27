mod tracing_json;

pub use tracing_json::JsonLayer;

#[cfg(feature = "tracing_json_derive")]
extern crate tracing_json_derive;

/// Derive macro available if serde is built with `features = ["derive"]`.
#[cfg(feature = "tracing_json_derive")]
pub use tracing_json_derive::{JsonTracing};
