use std::process::ExitCode;

use axum::{routing::post, Router};
use clap::Parser;
use paz::Result;
use tokio::net::TcpListener;

#[derive(Debug, Parser)]
struct Cli {}

impl Cli {
    async fn execute(&self) -> Result<ExitCode> {
        let router = Router::new().route("/", post(deploy));
        let listener = TcpListener::bind("0.0.0.0:8080").await?;
        axum::serve(listener, router).await?;
        Ok(ExitCode::SUCCESS)
    }
}

async fn deploy() -> &'static str {
    "ok"
}

fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .theme(color_eyre::config::Theme::new())
        .install()?;

    let cli = Cli::parse();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(10)
        .build()?
        .block_on(async { cli.execute().await })?;

    Ok(())
}
