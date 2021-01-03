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
    pub fn exec(&self) -> anyhow::Result<()> {
        panic!("Not yet implemented !")
    }
}