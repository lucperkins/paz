use std::process::ExitCode;

use clap::Parser;
use paz::Result;

#[derive(Debug, Parser)]
struct Cli {}

impl Cli {
    async fn execute(&self) -> Result<ExitCode> {
        Ok(ExitCode::SUCCESS)
    }
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
