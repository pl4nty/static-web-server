// SPDX-License-Identifier: MIT OR Apache-2.0
// This file is part of Static Web Server.
// See https://static-web-server.net/ for more information
// Copyright (C) 2019-present Jose Quintana <joseluisq.net>

//! Provides logging initialization for the web server.
//!

use tracing::Level;
use tracing_subscriber::{filter::Targets, fmt::format::FmtSpan, prelude::*};

use tracing_subscriber::layer::SubscriberExt;
use tracing_opentelemetry::OpenTelemetryLayer; // MetricsLayer

use opentelemetry_sdk::{
    // metrics::MeterProvider,
    runtime,
    trace::{BatchConfig, Config}
};

use crate::{Context, Result};

// use std::sync::Arc;

/// Logging system initialization
pub fn init(log_level: &str) -> Result {
    let log_level = log_level.to_lowercase();

    configure(&log_level).with_context(|| "failed to initialize logging")?;

    Ok(())
}

/// Initialize logging builder with its levels.
#[tokio::main]
async fn configure(level: &str) -> Result {
    let level = level
        .parse::<Level>()
        .with_context(|| "failed to parse log level")?;

    #[cfg(not(windows))]
    let enable_ansi = true;
    #[cfg(windows)]
    let enable_ansi = false;

    let filtered_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_span_events(FmtSpan::CLOSE)
        .with_ansi(enable_ansi)
        .with_filter(
            Targets::default()
                .with_default(level)
                .with_target("static_web_server::info", Level::INFO)
                .with_target("static_web_server::warn", Level::WARN),
        );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(Config::default())
        .with_batch_config(BatchConfig::default())
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(runtime::Tokio)
        .unwrap();

    let exporter = opentelemetry_prometheus::exporter().build()?;
    let meter_provider = MeterProvider::builder().with_reader(exporter).build();

    match tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))
        .with(MetricsLayer::new(meter_provider))
        .with(filtered_layer)
        .try_init()
    {
        Err(err) => Err(anyhow!(err)),
        _ => Ok(()),
    }
}

/// Custom info level macro.
#[macro_export]
macro_rules! server_info {
    ($($arg:tt)*) => {
        tracing::info!(
            target: "static_web_server::info",
            $($arg)*
        )
    };
}

/// Custom warn level macro.
#[macro_export]
macro_rules! server_warn {
    ($($arg:tt)*) => {
        tracing::warn!(
            target: "static_web_server::warn",
            $($arg)*
        )
    };
}
