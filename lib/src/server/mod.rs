use std::{
    net::TcpListener,
    path::{Path, PathBuf},
};

use crate::wiremock::MockServer;
use async_std::task::block_on;
use futures::future::join_all;
use itertools::Itertools;
use log::info;

use any_stub::AnyStubs;
use stub_finder::StubFinder;

#[cfg(feature = "record-standalone")]
use crate::record::{config::RecordConfig, standalone::StubrRecord};
use crate::{model::JsonStub, Config};

mod any_stub;
pub mod config;
pub mod stub_finder;

/// Allows running a Wiremock mock server from Wiremock stubs.
/// Delegates runtime to wiremock-rs.
pub struct Stubr {
    http_server: MockServer,
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
    pub async fn start<T>(stubs: T) -> Self
    where
        T: Into<AnyStubs>,
    {
        Self::start_with(stubs, Config::default()).await
    }

    /// Runs a mock server in a blocking way.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    pub fn start_blocking<T>(stubs: T) -> Self
    where
        T: Into<AnyStubs>,
    {
        Self::start_blocking_with(stubs, Config::default())
    }

    /// Runs a mock server with some configuration.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub async fn start_with<T>(stubs: T, config: Config) -> Self
    where
        T: Into<AnyStubs>,
    {
        let server = if let Some(p) = config.port {
            Self::start_on(p).await
        } else {
            Self::start_on_random_port().await
        };
        server.register_stubs(stubs.into(), config);
        #[cfg(not(feature = "grpc"))]
        server.register_cloud_features().await;
        server
    }

    /// Runs a mock server in a blocking way with some configuration.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub fn start_blocking_with<T>(stubs: T, config: Config) -> Self
    where
        T: Into<AnyStubs>,
    {
        block_on(Self::start_with(stubs, config))
    }

    /// Proxies requests and converts them into stubs
    #[cfg(feature = "record-standalone")]
    pub fn record() -> StubrRecord {
        StubrRecord::record(RecordConfig::default())
    }

    /// Proxies requests and converts them into stubs
    #[cfg(feature = "record-standalone")]
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
        Self::app_blocking_with(name, Config::default())
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub fn app_blocking_with(name: &str, config: Config) -> Self {
        block_on(Self::app_with(name, config))
    }

    pub async fn apps(names: &[&str]) -> Vec<Self> {
        Self::apps_with(names, Config::default()).await
    }

    pub async fn apps_with(names: &[&str], config: Config) -> Vec<Self> {
        join_all(names.iter().map(|n| async move { Self::app_with(n, config).await })).await
    }

    pub fn apps_blocking(names: &[&str]) -> Vec<Self> {
        Self::apps_blocking_with(names, Config::default())
    }

    pub fn apps_blocking_with(names: &[&str], config: Config) -> Vec<Self> {
        block_on(Self::apps_with(names, config))
    }

    /// Get running server address
    pub fn uri(&self) -> String {
        self.http_server.uri()
    }

    /// Get running server address and concatenate a path to it
    pub fn path(&self, path: &str) -> String {
        format!("{}{}", self.uri(), path)
    }

    async fn start_on(port: u16) -> Self {
        if let Ok(listener) = TcpListener::bind(format!("{}:{}", Self::HOST, port)) {
            Self {
                http_server: MockServer::builder()
                    .disable_request_recording()
                    .listener(listener)
                    .start()
                    .await,
            }
        } else {
            Self::start_on_random_port().await
        }
    }

    async fn start_on_random_port() -> Self {
        Self {
            http_server: MockServer::builder().disable_request_recording().start().await,
        }
    }

    fn register_stubs(&self, stub_folder: AnyStubs, config: Config) {
        stub_folder
            .0
            .iter()
            .flat_map(|folder| self.find_all_mocks(folder).map(move |(s, p)| (s, p, folder)))
            .sorted_by(|(a, _, _), (b, _, _)| a.priority.cmp(&b.priority))
            .filter_map(|(stub, path, folder)| stub.try_creating_from(&config).ok().map(|mock| (mock, path, folder)))
            .for_each(|(mock, file, folder)| {
                block_on(async move {
                    self.http_server.register(mock).await;
                });
                if config.verbose {
                    let maybe_file_name = file.strip_prefix(folder).ok().and_then(|file| file.to_str());
                    if let Some(file_name) = maybe_file_name {
                        info!("mounted stub '{}'", file_name);
                    }
                };
            });
    }

    #[allow(clippy::needless_lifetimes)]
    fn find_all_mocks<'a>(&self, from: &Path) -> impl Iterator<Item = (JsonStub, PathBuf)> + 'a {
        StubFinder::find_all_stubs(from).filter_map(move |path| JsonStub::try_from(&path).ok().map(|stub| (stub, path)))
    }

    #[cfg(not(feature = "grpc"))]
    async fn register_cloud_features(&self) {
        self.http_server.register(crate::cloud::probe::HttpProbe::health_probe()).await;
    }
}

#[cfg(test)]
mod server_test {
    use super::*;

    #[async_std::test]
    async fn should_find_all_mocks_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        assert!(Stubr::start_on_random_port().await.find_all_mocks(&from).count().gt(&2));
    }

    #[async_std::test]
    async fn should_find_all_mocks_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        assert_eq!(Stubr::start_on_random_port().await.find_all_mocks(&from).count(), 1);
    }

    #[async_std::test]
    async fn should_not_find_any_mock_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        assert_eq!(Stubr::start_on_random_port().await.find_all_mocks(&from).count(), 0);
        let from = PathBuf::from("tests/stubs/server/unknown.json");
        assert_eq!(Stubr::start_on_random_port().await.find_all_mocks(&from).count(), 0);
    }
}
