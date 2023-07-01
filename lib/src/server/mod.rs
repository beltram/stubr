use std::{
    net::TcpListener,
    path::{Path, PathBuf},
};

use async_std::task::block_on;
use futures::future::join_all;
use itertools::Itertools;
use log::info;

use any_stub::AnyStubs;
use stub_finder::StubFinder;

use crate::error::StubrResult;
#[cfg(feature = "record-standalone")]
use crate::record::{config::RecordConfig, standalone::StubrRecord};
use crate::wiremock_rs::MockServer;
use crate::{model::JsonStub, Config};

pub mod any_stub;
pub mod config;
pub mod stub_finder;

/// Allows running a Wiremock mock server from Wiremock stubs.
/// Delegates runtime to wiremock-rs.
pub struct Stubr {
    http_server: MockServer,
}

/// Fallible API
impl Stubr {
    /// Runs a mock server.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    pub async fn try_start<T>(stubs: T) -> StubrResult<Self>
    where
        T: Into<AnyStubs>,
    {
        Self::try_start_with(stubs, Config::default()).await
    }

    /// Runs a mock server with some configuration.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub async fn try_start_with<T>(stubs: T, config: Config) -> StubrResult<Self>
    where
        T: Into<AnyStubs>,
    {
        let server = if let Some(p) = config.port {
            Self::try_start_on(p).await
        } else {
            Self::try_start_on_random_port().await
        }?;
        server.try_register_stubs(stubs.into(), config)?;
        #[cfg(not(feature = "grpc"))]
        server.register_cloud_features().await;
        Ok(server)
    }

    /// Runs a mock server in a blocking way.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    pub fn try_start_blocking<T>(stubs: T) -> StubrResult<Self>
    where
        T: Into<AnyStubs>,
    {
        Self::try_start_blocking_with(stubs, Config::default())
    }

    /// Runs a mock server in a blocking way with some configuration.
    /// The server is unbinded when the instance is dropped.
    /// Use this in a test context.
    /// * `stubs` - folder or file containing the stubs
    /// * `config` - global server configuration
    pub fn try_start_blocking_with<T>(stubs: T, config: Config) -> StubrResult<Self>
    where
        T: Into<AnyStubs>,
    {
        block_on(Self::try_start_with(stubs, config))
    }

    /// Proxies requests and converts them into stubs
    #[cfg(feature = "record-standalone")]
    pub fn try_record() -> StubrResult<StubrRecord> {
        Self::try_record_with(RecordConfig::default())
    }

    /// Proxies requests and converts them into stubs
    #[cfg(feature = "record-standalone")]
    pub fn try_record_with(config: RecordConfig) -> StubrResult<StubrRecord> {
        Ok(StubrRecord::record(config))
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub async fn try_app(name: &str) -> StubrResult<Self> {
        Self::try_app_with(name, Config::default()).await
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub async fn try_app_with(name: &str, config: Config) -> StubrResult<Self> {
        let app = StubFinder::find_app(name)?;
        Self::try_start_with(app, config).await
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub fn try_app_blocking(name: &str) -> StubrResult<Self> {
        Self::try_app_blocking_with(name, Config::default())
    }

    /// Runs stubs of a remote producer app.
    /// * `name` - producer name
    pub fn try_app_blocking_with(name: &str, config: Config) -> StubrResult<Self> {
        block_on(Self::try_app_with(name, config))
    }

    pub async fn try_apps(names: &[&str]) -> StubrResult<Vec<Self>> {
        Self::try_apps_with(names, Config::default()).await
    }

    pub async fn try_apps_with(names: &[&str], config: Config) -> StubrResult<Vec<Self>> {
        Ok(join_all(
            names
                .iter()
                .map(|n| async move { Self::try_app_with(n, config).await.expect("TODO") }),
        )
        .await)
    }

    pub fn try_apps_blocking(names: &[&str]) -> StubrResult<Vec<Self>> {
        Self::try_apps_blocking_with(names, Config::default())
    }

    pub fn try_apps_blocking_with(names: &[&str], config: Config) -> StubrResult<Vec<Self>> {
        block_on(Self::try_apps_with(names, config))
    }

    /// Get running server address
    pub fn uri(&self) -> String {
        self.http_server.uri()
    }

    /// Get running server address and concatenate a path to it
    pub fn path(&self, path: &str) -> String {
        format!("{}{}", self.uri(), path)
    }

    async fn try_start_on(port: u16) -> StubrResult<Self> {
        if let Ok(listener) = TcpListener::bind(format!("{}:{}", Self::HOST, port)) {
            let http_server = MockServer::builder()
                .disable_request_recording()
                .listener(listener)
                .start()
                .await?;
            Ok(Self { http_server })
        } else {
            Self::try_start_on_random_port().await
        }
    }

    async fn try_start_on_random_port() -> StubrResult<Self> {
        let http_server = MockServer::builder().disable_request_recording().start().await?;
        Ok(Self { http_server })
    }

    fn try_register_stubs(&self, stub_folder: AnyStubs, config: Config) -> StubrResult<()> {
        stub_folder
            .0
            .iter()
            .filter_map(|folder| self.try_find_all_mocks(folder).ok().map(|mocks| (folder, mocks)))
            .flat_map(|(folder, mocks)| mocks.map(move |(s, p)| (s, p, folder)))
            .sorted_by(|(a, ..), (b, ..)| a.priority.cmp(&b.priority))
            .filter_map(|(stub, file, folder)| stub.try_creating_from(&config, &file).ok().map(|mock| (mock, file, folder)))
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
        Ok(())
    }

    #[allow(clippy::needless_lifetimes)]
    fn try_find_all_mocks<'a>(&self, from: &Path) -> StubrResult<impl Iterator<Item = (JsonStub, PathBuf)> + 'a> {
        Ok(StubFinder::find_all_stubs(from).filter_map(move |path| JsonStub::try_from(&path).ok().map(|stub| (stub, path))))
    }

    #[cfg(not(feature = "grpc"))]
    async fn register_cloud_features(&self) {
        self.http_server.register(crate::cloud::probe::HttpProbe::health_probe()).await;
    }
}

/// Infallible API
impl Stubr {
    #[cfg(feature = "cloud")]
    const HOST: &'static str = "0.0.0.0";

    #[cfg(not(feature = "cloud"))]
    const HOST: &'static str = "127.0.0.1";

    /// see [Stubr::try_start]
    pub async fn start<T>(stubs: T) -> Self
    where
        T: Into<AnyStubs>,
    {
        Self::try_start(stubs).await.expect("Could not start server")
    }

    /// see [Stubr::try_start_blocking]
    pub fn start_blocking<T>(stubs: T) -> Self
    where
        T: Into<AnyStubs>,
    {
        Self::start_blocking_with(stubs, Config::default())
    }

    /// see [Stubr::try_start_with]
    pub async fn start_with<T>(stubs: T, config: Config) -> Self
    where
        T: Into<AnyStubs>,
    {
        Self::try_start_with(stubs, config).await.expect("Could not start server")
    }

    /// see [Stubr::try_start_blocking_with]
    pub fn start_blocking_with<T>(stubs: T, config: Config) -> Self
    where
        T: Into<AnyStubs>,
    {
        block_on(Self::start_with(stubs, config))
    }

    /// see [Stubr::try_record]
    #[cfg(feature = "record-standalone")]
    pub fn record() -> StubrRecord {
        Self::record_with(RecordConfig::default())
    }

    /// see [Stubr::try_record_with]
    #[cfg(feature = "record-standalone")]
    pub fn record_with(config: RecordConfig) -> StubrRecord {
        Self::try_record_with(config).expect("Failed recording")
    }

    /// see [Stubr::try_app]
    pub async fn app(name: &str) -> Self {
        Self::app_with(name, Config::default()).await
    }

    /// see [Stubr::try_app_with]
    pub async fn app_with(name: &str, config: Config) -> Self {
        let app = StubFinder::find_app(name).expect(&format!("Could not find app {name}"));
        Self::start_with(app, config).await
    }

    /// see [Stubr::try_app_blocking]
    pub fn app_blocking(name: &str) -> Self {
        Self::app_blocking_with(name, Config::default())
    }

    /// see [Stubr::try_app_blocking_with]
    pub fn app_blocking_with(name: &str, config: Config) -> Self {
        block_on(Self::app_with(name, config))
    }

    /// see [Stubr::try_apps]
    pub async fn apps(names: &[&str]) -> Vec<Self> {
        Self::apps_with(names, Config::default()).await
    }

    /// see [Stubr::try_apps_with]
    pub async fn apps_with(names: &[&str], config: Config) -> Vec<Self> {
        join_all(names.iter().map(|n| async move { Self::app_with(n, config).await })).await
    }

    /// see [Stubr::try_apps_blocking]
    pub fn apps_blocking(names: &[&str]) -> Vec<Self> {
        Self::apps_blocking_with(names, Config::default())
    }

    /// see [Stubr::try_apps_blocking_with]
    pub fn apps_blocking_with(names: &[&str], config: Config) -> Vec<Self> {
        block_on(Self::apps_with(names, config))
    }
}

#[cfg(test)]
mod server_test {
    use super::*;

    #[async_std::test]
    async fn should_find_all_mocks_from_dir() {
        let from = PathBuf::from("tests/stubs/server");
        let stubr = Stubr::try_start_on_random_port().await.unwrap();
        assert!(stubr.try_find_all_mocks(&from).unwrap().count().gt(&2));
    }

    #[async_std::test]
    async fn should_find_all_mocks_from_single_file() {
        let from = PathBuf::from("tests/stubs/server/valid.json");
        let stubr = Stubr::try_start_on_random_port().await.unwrap();
        assert_eq!(stubr.try_find_all_mocks(&from).unwrap().count(), 1);
    }

    #[async_std::test]
    async fn should_not_find_any_mock_when_path_does_not_exist() {
        let from = PathBuf::from("tests/stubs/server/unknown");
        let stubr = Stubr::try_start_on_random_port().await.unwrap();
        assert_eq!(stubr.try_find_all_mocks(&from).unwrap().count(), 0);

        let from = PathBuf::from("tests/stubs/server/unknown.json");
        let stubr = Stubr::try_start_on_random_port().await.unwrap();
        assert_eq!(stubr.try_find_all_mocks(&from).unwrap().count(), 0);
    }
}
