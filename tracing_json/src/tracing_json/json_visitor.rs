use std::collections::BTreeMap;

pub struct JsonVisitor<'a>(pub &'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match serde_json::from_str::<serde_json::Value>(value) {
            Ok(value) => {
                self.0.insert(field.name().to_string(), value);
            }
            Err(_) => {
                self.0
                    .insert(field.name().to_string(), serde_json::json!(value));
            }
        }
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let s = format!("{:?}", value);
        match serde_json::from_str::<serde_json::Value>(&s) {
            Ok(value) => {
                self.0.insert(field.name().to_string(), value.into());
            }
            Err(_) => {
                self.0.insert(field.name().to_string(), s.into());
            }
        }
    }
}
