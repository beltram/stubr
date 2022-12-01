use std::{env::current_dir, path::PathBuf};

use clap::Parser;

use stubr::RecordConfig;

#[derive(Parser, Debug, Eq, PartialEq)]
pub struct CliRecordConfig {
    /// port number the recording proxy server is listening on
    ///
    /// Defaults to 3030
    #[clap(short, long, value_parser)]
    pub port: Option<u16>,
    /// directory where recorded stubs are stored
    ///
    /// Defaults to current directory
    #[clap(short, long, value_parser)]
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
