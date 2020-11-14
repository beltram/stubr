use crate::server::StubrServer;

pub mod stub;
pub mod model;
pub mod server;

pub struct Stubr {}

impl Stubr {
    pub async fn start() {
        StubrServer::start().await;
    }
}
