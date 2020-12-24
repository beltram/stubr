use std::convert::TryFrom;
use std::path::PathBuf;

use async_trait::async_trait;
use itertools::Itertools;
use wiremock::MockServer;

use crate::stub::StubrMock;

#[async_trait]
pub trait StubServer {
    async fn register_stubs(&self, stub_folder: PathBuf) -> anyhow::Result<()>;
    fn uri(&self) -> String;
}

pub struct StubrServer {
    instance: MockServer,
}

#[async_trait]
impl StubServer for StubrServer {

    async fn register_stubs(&self, stub_folder: PathBuf) -> anyhow::Result<()> {
        let stubs = self.get_all_stubs(stub_folder);
        stubs.iter()
            .flat_map(|it| it.file_name())
            .for_each(|it| println!(" - mounted stub {:?}", it));
        let mocks = stubs.into_iter()
            .flat_map(StubrMock::try_from)
            .map(|it| it.0)
            .collect_vec();
        for mock in mocks {
            self.instance.register(mock).await;
        }
        Ok(())
    }

    fn uri(&self) -> String {
        self.instance.uri()
    }
}

impl StubrServer {
    pub async fn start() -> Self {
        Self { instance: MockServer::start().await }
    }

    pub fn init_log(&self) {
        println!("--------------------------------------------------");
        println!("  Starting stubr server on {}  ", self.instance.uri());
        println!("--------------------------------------------------");
    }

    fn get_all_stubs(&self, from: PathBuf) -> Vec<PathBuf> {
        if from.is_file() {
            vec![from]
        } else {
            from.read_dir()
                .map(|dir| dir.into_iter().flatten().map(|it| it.path()).collect_vec())
                .unwrap_or_default()
        }
    }
}
