use std::path::PathBuf;

use crate::server::StubrServer;

pub mod server;
pub mod mapper;
pub mod model;

pub struct Stubr {}

impl Stubr {
    pub async fn run(maybe_stubs: Option<PathBuf>) -> anyhow::Result<()> {
        let server = StubrServer::start().await;
        if let Some(stubs) = maybe_stubs {
            server.register_stub(stubs).await?;
        }
        server.init_log();
        loop {}
    }
    pub async fn start() {
        StubrServer::start().await;
    }
}