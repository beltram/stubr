use std::time::Instant;

use clap::Clap;

use cli::Cli;

mod cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    Cli::parse().run(now).await
}