use std::collections::BTreeMap;
use tracing_subscriber::Layer;

use super::json_visitor::JsonVisitor;
use super::JsonFieldStorage;

pub struct JsonLayer;

impl JsonLayer {
    fn get_rfc_3339_time() -> String {
        let now = chrono::Utc::now();
        now.to_rfc3339_opts(chrono::SecondsFormat::Micros, true)
    }
}

impl<S> Layer<S> for JsonLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).unwrap();
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        attrs.record(&mut visitor);
        let storage = JsonFieldStorage(fields);
        let mut extensions = span.extensions_mut();
        extensions.insert(storage);
    }

    fn on_record(
        &self,
        id: &tracing::span::Id,
        values: &tracing::span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Get the span whose data is being recorded
        let span = ctx.span(id).unwrap();

        // Get a mutable reference to the data we created in new_span
        let mut extensions_mut = span.extensions_mut();
        let custom_field_storage: &mut JsonFieldStorage =
            extensions_mut.get_mut::<JsonFieldStorage>().unwrap();
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut custom_field_storage.0;

        // And add to using our old friend the visitor!
        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // All of the span context
        let mut spans = vec![];
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let extensions = span.extensions();
                let storage = extensions.get::<JsonFieldStorage>();
                if let Some(storage) = storage {
                    let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
                    spans.push(serde_json::json!({
                        "target": span.metadata().target(),
                        "name": span.name(),
                        "level": span.metadata().level().to_string(),
                        "fields": field_data,
                    }));
                }
            }
        }

        // The fields of the event
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // And create our output
        let output = serde_json::json!({
            "severity": event.metadata().level().to_string(),
            "message": fields.get("message").clone().unwrap_or(&serde_json::json!("default message")),
            "in_file": format!("{}:{}", event.metadata().file().unwrap_or_default(), event.metadata().line().unwrap_or_default()),
            "_":{
                "timestamp": Self::get_rfc_3339_time(),
                "target": event.metadata().target(),
                "fields": fields,
                "spans": spans,
            }
        });
        // match serde_json::to_string_pretty(&output) {
        match serde_json::to_string(&output) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Error serializing JSON: {} for this value {}", e, output),
        }
    }
}
