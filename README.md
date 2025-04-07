### Tracing json compatible with Google Logging Exporter

```


tracing_subscriber::registry()
    .with(JsonLayer)
    .with(
        tracing_subscriber::filter::Targets::default()
            .with_targets(vec![("examples", LevelFilter::TRACE)])
            .with_default(Level::TRACE),
    )
    .init();
```