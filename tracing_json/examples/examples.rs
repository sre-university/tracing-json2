use serde::{Deserialize, Serialize};
use tracing::*;
use tracing_core::{Level, LevelFilter};
use tracing_json::JsonLayer;
use tracing_json_derive::JsonTracing;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(JsonTracing, Serialize, Deserialize)]
struct BazData {
    baz_id: i32,
    baz_name: String,
    baz_comment: Option<String>,
}

fn baz(data: BazData) {
    let span = span!(Level::TRACE, "baz_span", ?data);
    let _enter = span.enter();

    trace!("baz trace");
}

#[derive(JsonTracing, Serialize, Deserialize)]
struct BarData {
    bar_id: i32,
    bar_name: String,
    bar_comment: Option<String>,
}

#[instrument]
fn bar(data: BarData) {
    let span = span!(Level::TRACE, "bar_span", ?data);
    let _enter = span.enter();

    debug!("bar before");
    baz(BazData {
        baz_id: 123,
        baz_name: "baz".to_string(),
        baz_comment: None,
    });
    debug!("bar after");
}

#[derive(JsonTracing, Serialize, Deserialize)]
struct FooData {
    foo_id: i32,
    foo_name: String,
    foo_comment: Option<String>,
}

#[instrument]
fn foo_func(data: FooData, data2: BarData, port: u16) {
    let span = span!(Level::TRACE, "foo_span", ?data, ?data2, port);
    let _enter = span.enter();

    warn!("foo before");

    bar(BarData {
        bar_id: 123,
        bar_name: "bar".to_string(),
        bar_comment: None,
    });

    warn!("foo after");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(JsonLayer)
        .with(
            tracing_subscriber::filter::Targets::default()
                .with_targets(vec![("examples", LevelFilter::TRACE)])
                .with_default(Level::TRACE),
        )
        .init();
}

fn main() {
    init_tracing();

    info!("tracing app started");

    foo_func(
        FooData {
            foo_id: -123,
            foo_name: "foo".to_string(),
            foo_comment: None,
        },
        BarData {
            bar_id: 1234,
            bar_name: "bar".to_string(),
            bar_comment: Option::from("bar comment".to_string()),
        },
        456,
    );

    info!("bye");
}
