use clap::Clap;

use super::completion::Shell;

#[derive(Clap, Debug, PartialEq)]
pub enum Commands {
    /// generates & installs completion scripts for the given shell
    Completion {
        #[clap(subcommand)]
        shell: Shell
    }
}

impl Commands {
    /// Dispatches subcommands
    pub fn exec(&self) -> anyhow::Result<()> {
        match self {
            Commands::Completion { shell } => shell.generate_and_install(),
        }
        Ok(())
    }
}