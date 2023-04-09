// SPDX-License-Identifier: MIT OR Apache-2.0
// This file is part of Static Web Server.
// See https://static-web-server.net/ for more information
// Copyright (C) 2019-present Jose Quintana <joseluisq.net>

//! Provides logging initialization for the web server.
//!

use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, util::SubscriberInitExt};

use opentelemetry::sdk::export::trace::stdout;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use crate::{Context, Result};

/// Logging system initialization
pub fn init(log_level: &str) -> Result {
    let log_level = log_level.to_lowercase();

    configure(&log_level).with_context(|| "failed to initialize logging")?;

    Ok(())
}

/// Initialize logging builder with its levels.
fn configure(level: &str) -> Result {
    let level = level
        .parse::<Level>()
        .with_context(|| "failed to parse log level")?;

    #[cfg(not(windows))]
    let enable_ansi = true;
    #[cfg(windows)]
    let enable_ansi = false;

    let tracer = stdout::new_pipeline().install_simple();
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
