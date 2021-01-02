use std::path::PathBuf;

use async_trait::async_trait;

use super::Stubr;

#[async_trait]
pub trait StubServer {
    async fn register_stubs(&self, stub_folder: PathBuf);
    fn uri(&self) -> String;
}

#[async_trait]
impl StubServer for Stubr {
    async fn register_stubs(&self, stub_folder: PathBuf) {
        for mock in self.find_all_mocks(stub_folder) {
            self.instance.register(mock).await;
        }
    }

    fn uri(&self) -> String {
        self.instance.uri()
    }
}