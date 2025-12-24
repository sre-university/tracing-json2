use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::*;
use tracing_core::{Level, LevelFilter};
use tracing_json2::{JsonLayer, TryJson};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn init_tracing() {
    let filter = EnvFilter::new("async_functions=info");
    tracing_subscriber::registry()
        .with(JsonLayer)
        .with(filter)
        .init();

    // tracing_subscriber::fmt()
    //     .with_thread_ids(true)
    //     .with_file(true)
    //     .with_line_number(true)
    //     .with_target(true)
    //     .with_env_filter(filter)
    //     .init();

    // tracing_subscriber::registry()
    //     .with(JsonLayer)
    //     .with(
    //         tracing_subscriber::filter::Targets::default()
    //             .with_targets(vec![("examples", LevelFilter::TRACE)])
    //             .with_default(Level::TRACE),
    //     )
    //     .init();
}

#[derive(Serialize, Deserialize)]
struct SomeStruct {
    val1: String,
    val2: String,
}

#[tracing::instrument(skip(a, b), fields(new_value3=tracing::field::Empty))]
pub async fn my_async_func2(a: i32, b: String) {
    info!("my_async_func2 started");
    tracing::Span::current().record(
        "new_value3",
        tracing::field::debug("a new value for async func2"),
    );

    info!("my_async_func2 done");
}

#[tracing::instrument(
    skip(a, b),
    fields(new_value1=tracing::field::Empty, new_value2=tracing::field::Empty)
)]
// We are now injecting `PgPool` to retrieve stored credentials from the database
pub async fn my_async_func1(a: i32, b: String) {
    info!("my_async_func1 started");
    let some_struct_val = SomeStruct {
        val1: "val_123".to_string(),
        val2: "any val".to_string(),
    };
    tracing::Span::current().record(
        "new_value1",
        tracing::field::debug(TryJson(&some_struct_val)),
    );
    sleep(Duration::from_millis(100)).await;
    my_async_func2(a + 1, b + "_extended").await;

    info!("my_async_func1 done");
}

#[tokio::main]
async fn main() {
    init_tracing();
    my_async_func1(123, "test_string".to_string()).await;
}
