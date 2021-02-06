use std::{convert::TryFrom, net::TcpListener, path::PathBuf, time::Duration};

use itertools::Itertools;
use wiremock::{Mock, MockServer};

use stub::StubrMock;
use traits::AnyStubServer;

use crate::Config;

mod stub;
pub mod traits;
pub mod config;

/// Allows running a Wiremock mock server from Wiremock stubs.
/// Delegates runtime to wiremock-rs.
pub struct Stubr {
    instance: MockServer,
}

impl Stubr {
    const HOST: &'static str = "127.0.0.1";
    const SLEEP_DURATION: Duration = Duration::from_millis(1000);

    /// Runs the mock server endlessly until process exits.
    /// Mostly used by the cli.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub async fn run<T>(stubs: T, config: Config) -> anyhow::Result<()> where T: Into<PathBuf> {
        let server = Self::start_with(stubs, config).await;
        server.init_log();
        loop { async_std::task::sleep(Self::SLEEP_DURATION).await; }
    }

    /// Runs a mock server.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    pub async fn start<T>(stubs: T) -> Self where T: Into<PathBuf> {
        Self::start_with(stubs, Config::default()).await
    }

    /// Runs a mock server.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub async fn start_with<T>(stubs: T, config: Config) -> Self where T: Into<PathBuf> {
        let server = if let Some(p) = config.port {
            Self::start_on(p).await
        } else {
            Self::start_on_random_port().await
        };
        server.register_stubs(stubs.into()).await;
        server
    }

    async fn start_on(port: u16) -> Self {
        if let Ok(listener) = TcpListener::bind(format!("{}:{}", Self::HOST, port)) {
            Self { instance: MockServer::start_on(listener).await }
        } else {
            Self::start_on_random_port().await
        }
    }

    async fn start_on_random_port() -> Self {
        Self { instance: MockServer::start().await }
    }

    fn init_log(&self) {
        println!("Started stubr server on {}", self.instance.uri());
    }

    fn find_all_files(&self, from: PathBuf) -> Vec<PathBuf> {
        if from.exists() {
            if from.is_dir() {
                from.read_dir()
                    .map(|dir| dir.into_iter().flatten()
                        .map(|it| it.path())
                        .filter(|it| it.is_file())
                        .collect_vec())
                    .unwrap_or_default()
            } else { vec![from] }
        } else { vec![] }
    }

    fn find_all_mocks(&self, from: PathBuf) -> Vec<Mock> {
        self.find_all_files(from).into_iter()
            .flat_map(StubrMock::try_from)
            .map(|it| {
                println!("- mounted stub {:?}", it.1);
                it.0
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod server_test {
    use itertools::Itertools;

    use super::*;

    #[async_std::test]
    async fn should_find_all_files_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let files = Stubr::start_on_random_port().await.find_all_files(from);
        assert_eq!(files.len(), 2);
        let file_names = files.iter()
            .map(|it| it.file_name().unwrap().to_str().unwrap())
            .collect_vec();
        assert!(file_names.contains(&"valid.json"));
        assert!(file_names.contains(&"also_valid.json"));
    }

    #[async_std::test]
    async fn should_find_all_files_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        let files = Stubr::start_on_random_port().await.find_all_files(from);
        assert_eq!(files.len(), 1);
        let file_names = files.iter()
            .map(|it| it.file_name().unwrap().to_str().unwrap())
            .collect_vec();
        assert!(file_names.contains(&"valid.json"));
    }

    #[async_std::test]
    async fn should_not_find_any_file_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let files = Stubr::start_on_random_port().await.find_all_files(from);
        assert!(files.is_empty());
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let files = Stubr::start_on_random_port().await.find_all_files(from);
        assert!(files.is_empty());
    }

    #[async_std::test]
    async fn should_find_all_mocks_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let mocks = Stubr::start_on_random_port().await.find_all_mocks(from);
        assert_eq!(mocks.len(), 2);
    }

    #[async_std::test]
    async fn should_find_all_mocks_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        let mocks = Stubr::start_on_random_port().await.find_all_mocks(from);
        assert_eq!(mocks.len(), 1);
    }

    #[async_std::test]
    async fn should_not_find_any_mock_when_none_valid() {
        let from = PathBuf::from("tests/stubs/server/invalid");
        let mocks = Stubr::start_on_random_port().await.find_all_mocks(from);
        assert!(mocks.is_empty());
    }

    #[async_std::test]
    async fn should_not_find_any_mock_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let mocks = Stubr::start_on_random_port().await.find_all_mocks(from);
        assert!(mocks.is_empty());
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let mocks = Stubr::start_on_random_port().await.find_all_mocks(from);
        assert!(mocks.is_empty());
    }
}
