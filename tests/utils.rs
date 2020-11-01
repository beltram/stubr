use std::path::PathBuf;

use async_std::task::block_on;

use stubr::server::StubrServer;

pub fn stub(name: &str) -> PathBuf {
    std::env::current_dir()
        .map(|it| it.join(PathBuf::from(format!("tests/stubs/{}.json", name))))
        .expect("Unexpected error")
}

pub fn mount(name: &str) -> StubrServer {
    let server = block_on(StubrServer::start());
    block_on(server.register_stub(stub(name))).unwrap();
    server
}