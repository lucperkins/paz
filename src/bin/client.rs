use std::process::ExitCode;

use clap::{Parser, Subcommand};
use paz::Result;

#[derive(Debug, Subcommand)]
enum Command {
    /// Deploy a Paz application.
    Deploy,
}

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    async fn execute(&self) -> Result<ExitCode> {
        use Command::*;

        match self.command {
            Deploy => {
                reqwest::Client::new()
                    .post("http://0.0.0.0:8080/deploy")
                    .send()
                    .await?;

                Ok(ExitCode::SUCCESS)
            }
        }
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
