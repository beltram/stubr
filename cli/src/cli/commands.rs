use clap::Parser;

use crate::record::config::CliRecordConfig;

use super::super::record::Record;
use super::completion::Shell;

#[derive(Parser, Debug, Eq, PartialEq)]
pub enum Commands {
    /// generates & installs completion scripts for the given shell
    Completion {
        #[clap(subcommand)]
        shell: Shell,
    },
    /// Records incoming exchanges and convert them to stubs
    Record {
        #[clap(flatten)]
        config: CliRecordConfig,
    },
}

impl Commands {
    /// Dispatches subcommands
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            Commands::Completion { shell } => shell.generate_and_install(),
            Commands::Record { config } => Record::record(config).await,
        }
        Ok(())
    }
}
