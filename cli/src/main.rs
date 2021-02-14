use clap::Clap;

use cli::Cli;

mod cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().run().await
}