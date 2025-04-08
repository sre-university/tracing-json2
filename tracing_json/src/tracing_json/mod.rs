// origins:
// https://burgers.io/custom-logging-in-rust-using-tracing
// https://burgers.io/custom-logging-in-rust-using-tracing-part-2
// https://github.com/bryanburgers/tracing-blog-post/blob/main/examples/figure_10/custom_layer.rs

use std::collections::BTreeMap;

mod json_layer;
mod json_visitor;
mod try_json;
pub use json_layer::JsonLayer;
pub use try_json::TryJson;

#[derive(Debug)]
struct JsonFieldStorage(BTreeMap<String, serde_json::Value>);
