use std::{
    env::current_dir,
    ffi::OsStr,
    fs::DirEntry,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use clap::{Parser, ValueHint};
use colored::Colorize;
use log::info;

use commands::Commands;
use stubr::{Config, Stubr};

mod commands;
mod completion;
pub mod logger;

/// A Rust implementation of Wiremock
#[derive(Parser, Debug, Default)]
#[clap(version, about, name = "stubr", bin_name = "stubr", rename_all = "kebab-case")]
pub struct Cli {
    /// stub files directory
    ///
    /// Wiremock stub files are json files.
    /// Defaults to current directory when not present
    #[clap(value_parser, value_hint = ValueHint::DirPath)]
    dir: Option<PathBuf>,
    /// equivalent of 'root-dir' option in Wiremock cli
    ///
    /// Expects a 'mappings' folder under this directory which contains stub files
    #[clap(long = "root-dir", value_parser, value_hint = ValueHint::AnyPath)]
    root_dir: Option<PathBuf>,
    /// port number the server is listening on
    ///
    /// Defaults to a random one
    #[clap(short, long, value_parser)]
    port: Option<u16>,
    /// global delay e.g. 10ms or 2s
    ///
    /// supersedes any locally defined delay
    #[clap(short, long, value_parser)]
    delay: Option<String>,
    /// latency e.g. 10ms or 2s
    ///
    /// adds this delay to any locally defined delay. Simulates network delays.
    #[clap(short, long, value_parser)]
    latency: Option<String>,
    #[clap(subcommand)]
    cmd: Option<Commands>,
}

impl Cli {
    const MAPPINGS_FOLDER: &'static str = "mappings";
    const SLEEP_DURATION: Duration = Duration::from_millis(1000);

    // Runs stubr forever until process exits
    pub async fn run(self, start_time: Instant) -> anyhow::Result<()> {
        if let Some(cmd) = self.cmd {
            cmd.exec().await
        } else {
            Self::run_server(self.stubs_dir(), self.into(), start_time).await
        }
    }

    /// Runs the mock server endlessly until process exits.
    /// Mostly used by the cli.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    async fn run_server(stubs: PathBuf, config: Config, start_time: Instant) -> anyhow::Result<()> {
        let server = Stubr::start_with(stubs, config).await;
        info!("Started {} in {}ms on {}", "stubr".green().bold(), start_time.elapsed().as_millis(), server.uri());
        loop { async_std::task::sleep(Self::SLEEP_DURATION).await; }
    }

    fn stubs_dir(&self) -> PathBuf {
        self.root_dir()
            .or_else(|| self.dir())
            .expect("Could not find stub directory")
    }

    fn dir(&self) -> Option<PathBuf> {
        let current = current_dir().ok()?;
        self.dir.as_ref()
            .map(|d| current.join(d))
            .or(Some(current))
    }

    fn root_dir(&self) -> Option<PathBuf> {
        let current = current_dir().ok()?;
        self.root_dir.as_ref()
            .filter(|root| Self::does_contains_mappings_folder(root))
            .map(|root| current.join(root.join(Self::MAPPINGS_FOLDER)))
    }

    fn does_contains_mappings_folder(input: &Path) -> bool {
        input.read_dir().ok().as_mut()
            .map(|all| all.any(|child| child.map(Self::is_mappings_folder).unwrap_or_default()))
            .unwrap_or_default()
    }

    fn is_mappings_folder(folder: DirEntry) -> bool {
        let path = folder.path();
        path.is_dir() && path.file_name() == Some(OsStr::new(Self::MAPPINGS_FOLDER))
    }

    fn global_delay_milliseconds(&self) -> Option<u64> {
        humantime::parse_duration(self.delay.as_deref()?).ok()
            ?.as_millis()
            .try_into().ok()
    }

    fn latency_milliseconds(&self) -> Option<u64> {
        humantime::parse_duration(self.latency.as_deref()?).ok()
            ?.as_millis()
            .try_into().ok()
    }
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        Self {
            port: cli.port,
            verbose: Some(true),
            global_delay: cli.global_delay_milliseconds(),
            latency: cli.latency_milliseconds(),
        }
    }
}

#[cfg(test)]
mod cli_tests {
    use std::{env::current_dir, path::PathBuf};

    use crate::cli::Cli;

    #[test]
    fn stubs_dir_should_append_dir_to_current_dir() {
        let dir = PathBuf::from("tests/stubs");
        let cli = Cli { dir: Some(dir.clone()), ..Default::default() };
        assert_eq!(cli.stubs_dir(), current_dir().unwrap().join(dir))
    }

    #[test]
    fn stubs_dir_should_default_to_current_dir() {
        let cli = Cli { dir: None, ..Default::default() };
        assert_eq!(cli.stubs_dir(), current_dir().unwrap())
    }

    #[test]
    fn root_dir_should_default_to_none_when_not_provided() {
        assert!(Cli::default().root_dir().is_none())
    }

    #[test]
    fn root_dir_should_be_appended_to_current_dir() {
        let root_dir = PathBuf::from("tests/stubs");
        let cli = Cli { root_dir: Some(root_dir.clone()), ..Default::default() };
        assert_eq!(cli.root_dir().unwrap(), current_dir().unwrap().join(root_dir).join("mappings"))
    }

    #[test]
    fn root_dir_should_have_precedence_over_dir() {
        let dir = PathBuf::from("tests/stubs");
        let root_dir = PathBuf::from("tests/stubs");
        let cli = Cli { dir: Some(dir), root_dir: Some(root_dir.clone()), ..Default::default() };
        assert_eq!(cli.stubs_dir(), current_dir().unwrap().join(root_dir).join("mappings"))
    }
}