use std::convert::TryInto;
use std::path::PathBuf;

use wiremock::MockServer;

use crate::mapper::StubrMock;

pub struct StubrServer {
    instance: MockServer,
}

impl StubrServer {
    pub async fn start() -> Self {
        Self {
            instance: MockServer::start().await,
        }
    }

    pub fn init_log(&self) {
        println!("--------------------------------------------------");
        println!("  Starting stubr server on {}  ", self.instance.uri());
        println!("--------------------------------------------------");
    }

    pub async fn register_stub(&self, stub_file: PathBuf) -> anyhow::Result<()> {
        let file_name = stub_file.file_name().map(|it| it.to_owned());
        let stub: StubrMock = stub_file.try_into()?;
        self.instance.register(stub.0).await;
        if let Some(file_name) = file_name {
            println!(" - mounted {:?}", file_name);
        }
        Ok(())
    }

    pub fn uri(&self) -> String {
        self.instance.uri()
    }
}
