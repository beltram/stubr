use std::path::PathBuf;

use async_trait::async_trait;

use stubr::{Config, Stubr};

/// Abstraction over a server capable of serving stubs.
/// Mostly used internally in a test context
/// to also execute integration tests against a real
/// [Wiremock](http://wiremock.org/) server instance.
#[async_trait]
pub trait AnyStubServer {
    async fn register_stubs(&self, stub_folder: PathBuf, config: Config);
    fn url(&self) -> String;
}

#[async_trait]
impl AnyStubServer for Stubr {
    async fn register_stubs(&self, stub_folder: PathBuf, config: Config) {
        self.register_stubs(stub_folder, config).await
    }

    fn url(&self) -> String {
        self.uri()
    }
}