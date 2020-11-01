use std::env;
use std::path::PathBuf;
use std::time::Duration;

use stubr::server::StubrServer;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let maybe_stub = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .and_then(|it| stub_path(it));
    let server = StubrServer::start().await;
    if let Some(stubs) = maybe_stub {
        server.register_stub(stubs).await?;
    }
    server.init_log();
    loop {
        std::thread::sleep(Duration::from_millis(1))
    }
}

fn stub_path(arg: &str) -> Option<PathBuf> {
    env::current_dir()
        .ok()
        .map(|it| it.join(PathBuf::from(arg)))
}
