//! Provides logging initialization for the web server.
//!

use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, util::SubscriberInitExt};

use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
// use opentelemetry_sdk::trace::tracer::Tracer;

use crate::{Context, Result};

/// Logging system initialization
pub fn init(log_level: &str) -> Result {
    let log_level = log_level.to_lowercase();

    configure(&log_level).with_context(|| "failed to initialize logging")?;

    tracing::info!("logging level: {}", log_level);

    Ok(())
}

/// Initialize logging builder with its levels.
fn configure(level: &str) -> Result {
    let level = level.parse::<Level>()?;

    #[cfg(not(windows))]
    let enable_ansi = true;
    #[cfg(windows)]
    let enable_ansi = false;

    tokio::runtime::Builder::new_current_thread()
        .build()

    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_simple()?;
    let telemetry = tracing_opentelemetry::layer::<Registry>().with_tracer(tracer);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr.with_max_level(level))
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(enable_ansi);

    match Registry::default()
        .with(telemetry)
        .with(fmt_layer)
        .try_init()
    {
        Err(err) => Err(anyhow!(err)),
        _ => Ok(()),
    }
}
