use std::{convert::TryFrom, net::TcpListener};

use async_std::{path::PathBuf, task::block_on};
use async_std::stream::Stream;
use futures::stream::StreamExt;
use log::info;
use wiremock::{Mock, MockServer};

use stub::StubrMock;
use stub_finder::StubFinder;

use crate::{cloud::probe::HttpProbe, Config};
#[cfg(feature = "record")]
use crate::record::{config::RecordConfig, StubrRecord};

mod stub;
pub mod stub_finder;
pub mod config;

/// Allows running a Wiremock mock server from Wiremock stubs.
/// Delegates runtime to wiremock-rs.
pub struct Stubr {
    instance: MockServer,
}

impl Stubr {
    #[cfg(feature = "cloud")]
    const HOST: &'static str = "0.0.0.0";

    #[cfg(not(feature = "cloud"))]
    const HOST: &'static str = "127.0.0.1";

    /// Runs a mock server.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    pub async fn start<T>(stubs: T) -> Self where T: Into<PathBuf> {
        Self::start_with(stubs, Config::default()).await
    }

    /// Runs a mock server in a blocking way.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    pub fn start_blocking<T>(stubs: T) -> Self where T: Into<PathBuf> {
        block_on(Self::start(stubs))
    }

    /// Runs a mock server with some configuration.
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
        server.register_stubs(stubs.into(), config).await;
        server.register_cloud_features().await;
        server
    }

    /// Runs a mock server in a blocking way with some configuration.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub fn start_blocking_with<T>(stubs: T, config: Config) -> Self where T: Into<PathBuf> {
        block_on(Self::start_with(stubs, config))
    }

    /// Proxies requests and converts them into stubs
    #[cfg(feature = "record")]
    pub fn record() -> StubrRecord {
        StubrRecord::record(RecordConfig::default())
    }

    /// Proxies requests and converts them into stubs
    #[cfg(feature = "record")]
    pub fn record_with(config: RecordConfig) -> StubrRecord {
        StubrRecord::record(config)
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub async fn app(name: &str) -> Self {
        Self::app_with(name, Config::default()).await
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub async fn app_with(name: &str, config: Config) -> Self {
        Self::start_with(StubFinder::find_app(name), config).await
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub fn app_blocking(name: &str) -> Self {
        block_on(Self::app(name))
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub fn app_blocking_with(name: &str, config: Config) -> Self {
        block_on(Self::app_with(name, config))
    }

    /// Get running server address
    pub fn uri(&self) -> String {
        self.instance.uri()
    }

    async fn start_on(port: u16) -> Self {
        if let Ok(listener) = TcpListener::bind(format!("{}:{}", Self::HOST, port)) {
            Self {
                instance: MockServer::builder().disable_request_recording().listener(listener).start().await
            }
        } else {
            Self::start_on_random_port().await
        }
    }

    async fn start_on_random_port() -> Self {
        Self { instance: MockServer::builder().disable_request_recording().start().await }
    }

    async fn register_stubs(&self, stub_folder: PathBuf, config: Config) {
        let cfg = &config;
        let folder = &stub_folder;
        self.find_all_mocks(&stub_folder, &config).await.for_each(|(mock, file)| async move {
            self.instance.register(mock).await;
            if cfg.verbose.unwrap_or_default() {
                let maybe_file_name = file.strip_prefix(&folder).ok().and_then(|file| file.to_str());
                if let Some(file_name) = maybe_file_name {
                    info!("mounted stub '{}'", file_name);
                }
            }
        }).await;
    }

    #[allow(clippy::needless_lifetimes)]
    async fn find_all_mocks<'a>(&self, from: &PathBuf, config: &'a Config) -> impl Stream<Item=(Mock, PathBuf)> + 'a {
        StubFinder::find_all_stubs(from).await
            .map(move |path| {
                StubrMock::try_from((&path, config))
                    .map(|mock| (mock.0, path))
            })
            .filter_map(|it| async { it.ok() })
    }

    async fn register_cloud_features(&self) {
        self.instance.register(HttpProbe::health_probe()).await;
    }
}

#[cfg(test)]
mod server_test {
    use futures::stream::StreamExt;

    use super::*;

    #[async_std::test]
    async fn should_find_all_mocks_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let config = Config::default();
        let mocks = Stubr::start_on_random_port().await
            .find_all_mocks(&from, &config).await
            .collect::<Vec<(Mock, PathBuf)>>().await;
        assert_eq!(mocks.len(), 2);
    }

    #[async_std::test]
    async fn should_find_all_mocks_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        let config = Config::default();
        let mocks = Stubr::start_on_random_port().await
            .find_all_mocks(&from, &config).await
            .collect::<Vec<(Mock, PathBuf)>>().await;
        assert_eq!(mocks.len(), 1);
    }

    #[async_std::test]
    async fn should_not_find_any_mock_when_none_valid() {
        let from = PathBuf::from("tests/stubs/server/invalid");
        let config = Config::default();
        let mocks = Stubr::start_on_random_port().await
            .find_all_mocks(&from, &config).await
            .collect::<Vec<(Mock, PathBuf)>>().await;
        assert_eq!(mocks.len(), 0);
    }

    #[async_std::test]
    async fn should_not_find_any_mock_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let config = Config::default();
        let mocks = Stubr::start_on_random_port().await
            .find_all_mocks(&from, &config).await
            .collect::<Vec<(Mock, PathBuf)>>().await;
        assert_eq!(mocks.len(), 0);
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let mocks = Stubr::start_on_random_port().await
            .find_all_mocks(&from, &config).await
            .collect::<Vec<(Mock, PathBuf)>>().await;
        assert_eq!(mocks.len(), 0);
    }
}