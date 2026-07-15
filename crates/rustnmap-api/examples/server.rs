// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026  greatwallisme
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! API Server example - Start the REST API server
//!
//! This example demonstrates how to start the `RustNmap` REST API server.
//!
//! # Usage
//!
//! ```bash
//! cargo run --package rustnmap-api --example server
//! ```
//!
//! Or run the compiled binary:
//! ```bash
//! ./target/release/examples/server
//! ```

use std::net::SocketAddr;

use rustnmap_api::{ApiConfig, ApiServer};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer().with_target(false).with_thread_ids(false))
        .with(env_filter)
        .init();

    // Create API configuration. The listen address and API keys are taken from
    // the environment so the server can be run as a managed service with a
    // stable key and a configurable bind address; both fall back to the
    // original example defaults (127.0.0.1:8080 and a freshly-generated key).
    let listen_addr =
        std::env::var("RUSTNMAP_API_LISTEN").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let mut config = ApiConfig::new()
        .with_listen_addr(listen_addr)
        .with_max_concurrent_scans(5);

    // RUSTNMAP_API_KEYS is a comma-separated list of accepted bearer tokens.
    if let Ok(raw) = std::env::var("RUSTNMAP_API_KEYS") {
        let keys: Vec<String> = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        if !keys.is_empty() {
            config = config.with_api_keys(keys);
        }
    }

    // Log server startup information
    tracing::info!("============================================");
    tracing::info!("   RustNmap REST API Server");
    tracing::info!("============================================");
    tracing::info!("Listen address: http://{}", config.listen_addr);
    tracing::info!("Max concurrent scans: {}", config.max_concurrent_scans);
    tracing::info!("API Keys (use in Authorization header):");
    for (i, key) in config.api_keys.iter().enumerate() {
        tracing::info!("  [{}]: {}", i + 1, key);
    }
    tracing::info!("Test endpoints:");
    tracing::info!("  Health check: curl http://127.0.0.1:8080/api/v1/health");
    tracing::info!("  Create scan:  curl -X POST http://127.0.0.1:8080/api/v1/scans \\");
    tracing::info!("                -H 'Authorization: Bearer <API_KEY>' \\");
    tracing::info!("                -H 'Content-Type: application/json' \\");
    tracing::info!(
        "                -d '{{\"targets\":[\"127.0.0.1\"],\"scan_type\":\"connect\"}}'"
    );
    tracing::info!("Press Ctrl+C to stop the server");
    tracing::info!("============================================");

    // Create and run server
    let server = ApiServer::new(&config)?;
    let addr: SocketAddr = config.listen_addr.parse()?;

    server.run(addr).await?;

    Ok(())
}
