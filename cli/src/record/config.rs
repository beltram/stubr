use std::env::current_dir;
use std::path::PathBuf;

use clap::Clap;

use stubr::RecordConfig;

#[derive(Clap, Debug, Eq, PartialEq)]
pub struct CliRecordConfig {
    /// port number the recording proxy server is listening on
    ///
    /// Defaults to 3030
    #[clap(short, long)]
    pub port: Option<u16>,
    /// directory where recorded stubs are stored
    ///
    /// Defaults to current directory
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

impl Default for CliRecordConfig {
    fn default() -> Self {
        Self {
            port: Some(3030),
            output: current_dir().ok(),
        }
    }
}

impl From<CliRecordConfig> for RecordConfig {
    fn from(cli_cfg: CliRecordConfig) -> Self {
        Self {
            port: cli_cfg.port,
            output: cli_cfg.output.or_else(|| current_dir().ok()),
            ..Default::default()
        }
    }
}