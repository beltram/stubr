use std::env;
use std::path::PathBuf;
use std::time::Duration;

use stubr::server::StubrServer;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let maybe_stubs = maybe_path_arg().and_then(|it| maybe_stubs(it));
    let server = StubrServer::start().await;
    if let Some(stubs) = maybe_stubs {
        server.register_stubs(stubs).await?;
    }
    server.init_log();
    loop {
        async_std::task::sleep(Duration::from_millis(1000)).await;
    }
}

fn maybe_path_arg() -> Option<String> {
    env::args().collect::<Vec<String>>().get(1).map(|it| it.to_string())
}

fn maybe_stubs(arg: String) -> Option<PathBuf> {
    env::current_dir().ok()
        .map(|it| it.join(PathBuf::from(arg)))
}