mod server;
mod handler;
mod error;
mod config;
mod cli;

use tracing_appender::rolling;
use std::fs;
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {

    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    fs::create_dir_all("logs").expect("cannot create logs dir");

    let file_appender = rolling::daily("logs", "app_log");

    let stdout_layer = fmt::Layer::default()
        .with_writer(std::io::stdout)
        .with_line_number(true);

    let file_layer = fmt::Layer::default()
        .with_writer(file_appender)
        .with_line_number(true)
        .with_ansi(false);

    let subscriber =  tracing_subscriber::registry()
        .with(EnvFilter::new(rust_log))
        .with(stdout_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber).expect("subscrib failed");



    tracing::info!("start zkml worker");
    cli::run_cli().await;

}

