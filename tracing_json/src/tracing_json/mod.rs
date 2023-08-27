// origins:
// https://burgers.io/custom-logging-in-rust-using-tracing
// https://burgers.io/custom-logging-in-rust-using-tracing-part-2
// https://github.com/bryanburgers/tracing-blog-post/blob/main/examples/figure_10/custom_layer.rs

use std::collections::BTreeMap;

mod json_layer;
mod json_visitor;

pub use json_layer::JsonLayer;

#[derive(Debug)]
struct JsonFieldStorage(BTreeMap<String, serde_json::Value>);
