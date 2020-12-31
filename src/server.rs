use std::convert::TryFrom;
use std::net::TcpListener;
use std::path::PathBuf;
use std::time::Duration;

use async_trait::async_trait;
use itertools::Itertools;
use wiremock::MockServer;

use crate::stub::StubrMock;

pub struct StubrServer {
    instance: MockServer,
}

impl StubrServer {
    const SLEEP_DURATION: Duration = Duration::from_millis(1000);

    pub async fn run(stubs: PathBuf, port: Option<u16>) -> anyhow::Result<()> {
        let server = StubrServer::start(port).await;
        server.register_stubs(stubs).await?;
        server.init_log();
        loop { async_std::task::sleep(Self::SLEEP_DURATION).await; }
    }

    pub async fn start(port: Option<u16>) -> Self {
        if let Some(p) = port {
            Self::start_on(p).await
        } else {
            Self::start_on_random_port().await
        }
    }

    async fn start_on(port: u16) -> Self {
        if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Self { instance: MockServer::start_on(listener).await }
        } else {
            Self::start_on_random_port().await
        }
    }

    async fn start_on_random_port() -> Self {
        Self { instance: MockServer::start().await }
    }

    pub fn init_log(&self) {
        println!("Starting stubr server on {}", self.instance.uri());
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

#[async_trait]
pub trait StubServer {
    async fn register_stubs(&self, stub_folder: PathBuf) -> anyhow::Result<()>;
    fn uri(&self) -> String;
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
