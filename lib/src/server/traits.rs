use std::path::PathBuf;

use async_trait::async_trait;

use super::Stubr;

/// Abstraction over a server capable of serving stubs.
/// Mostly used internally in a test context
/// to also execute integration tests against a real
/// [Wiremock](http://wiremock.org/) server instance.
#[async_trait]
pub trait AnyStubServer {
    async fn register_stubs(&self, stub_folder: PathBuf);
    fn uri(&self) -> String;
}

#[async_trait]
impl AnyStubServer for Stubr {
    async fn register_stubs(&self, stub_folder: PathBuf) {
        for mock in self.find_all_mocks(stub_folder) {
            self.instance.register(mock).await;
        }
    }

    fn uri(&self) -> String {
        self.instance.uri()
    }
}