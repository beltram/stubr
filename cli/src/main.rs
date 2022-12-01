use std::time::Instant;

use clap::Parser;

use cli::{logger::Logger, Cli};

mod cli;
mod record;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    Logger::init()?;
    Cli::parse().run(now).await
}
