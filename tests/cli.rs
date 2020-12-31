use std::process::{Child, Command};
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use surf::get;

use crate::utils::*;

mod utils;

static PORT: AtomicU16 = AtomicU16::new(60_000);

fn port() -> String {
    PORT.fetch_add(1, Ordering::SeqCst).to_string()
}

fn stubr(args: &[&str]) -> (Child, String) {
    let port = port();
    let addr = format!("http://127.0.0.1:{}", &port);
    let child = Command::cargo_bin("stubr").unwrap()
        .args(args)
        .args(&["--port", &port])
        .spawn().unwrap();
    sleep(Duration::from_millis(200));
    (child, addr)
}

#[async_std::test]
async fn should_serve_stubs_under_dir() {
    let (mut child, addr) = stubr(&["tests/stubs/cli"]);
    get(addr).await.unwrap().assert_ok();
    child.kill().unwrap()
}
