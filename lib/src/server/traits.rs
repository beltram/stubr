use std::{env, path::PathBuf};

use async_trait::async_trait;

use super::{Config, Stubr};

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
        for (mock, file) in self.find_all_mocks(&stub_folder) {
            self.instance.register(mock).await;
            if config.verbose.unwrap_or_default() {
                let maybe_file_name = env::current_dir().ok()
                    .and_then(|current| file.strip_prefix(current).ok())
                    .and_then(|file| file.to_str());
                if let Some(file_name) = maybe_file_name {
                    println!("+ mounted '{}'", file_name);
                }
            }
        }
    }

    fn url(&self) -> String {
        self.uri()
    }
}