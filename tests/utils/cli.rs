use std::{
    process::{Child, Command},
    sync::atomic::{AtomicU16, Ordering},
    thread::sleep,
    time::Duration,
};

use assert_cmd::prelude::*;

pub struct StubrCli {
    child: Child,
    pub addr: String,
}

impl StubrCli {
    const SLEEP: u64 = 200;
    const DEFAULT_PORT: u16 = 60_000;
    const PORT: AtomicU16 = AtomicU16::new(Self::DEFAULT_PORT);
    const HOST: &'static str = "127.0.0.1";

    pub fn new(args: &[&str]) -> Self {
        let port = Self::port();
        let addr = format!("http://{}:{}", Self::HOST, &port);
        let child = Command::cargo_bin("stubr").unwrap()
            .args(args)
            .args(&["--port", &port])
            .spawn().unwrap();
        sleep(Duration::from_millis(Self::SLEEP));
        Self { child, addr }
    }

    fn port() -> String {
        Self::PORT.fetch_add(1, Ordering::SeqCst).to_string()
    }
}

impl Drop for StubrCli {
    fn drop(&mut self) {
        self.child.kill().unwrap()
    }
}