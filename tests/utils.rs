#![allow(dead_code)]

use std::path::PathBuf;

use async_std::task::block_on;
use surf::Response;

use stubr::server::StubrServer;

pub fn given(name: &str) -> StubrServer {
    let server = block_on(StubrServer::start());
    block_on(server.register_stub(stub(name))).unwrap();
    server
}

fn stub(name: &str) -> PathBuf {
    std::env::current_dir()
        .map(|it| it.join(PathBuf::from(format!("tests/stubs/{}.json", name))))
        .expect("Unexpected error")
}

pub trait UriAndQuery {
    fn uri(&self) -> String;

    fn path(&self, path: &str) -> String {
        format!("{}{}", self.uri(), path)
    }

    fn path_query(&self, path: &str, key: &str, value: &str) -> String {
        format!("{}{}?{}={}", self.uri(), path, key, value)
    }

    fn query(&self, key: &str, value: &str) -> String {
        format!("{}?{}={}", self.uri(), key, value)
    }

    fn path_queries(&self, path: &str, q1: (&str, &str), q2: (&str, &str)) -> String {
        format!("{}{}?{}={}&{}={}", self.uri(), path, q1.0, q1.1, q2.0, q2.1)
    }
    fn queries(&self, q1: (&str, &str), q2: (&str, &str)) -> String {
        format!("{}?{}={}&{}={}", self.uri(), q1.0, q1.1, q2.0, q2.1)
    }
}

impl UriAndQuery for StubrServer {
    fn uri(&self) -> String { self.uri() }
}

pub trait ResponseAsserter {
    fn assert_status_eq(&self, status: u16);
    fn assert_ok(&self) { self.assert_status_eq(200) }
    fn assert_not_found(&self) { self.assert_status_eq(404) }
}

impl ResponseAsserter for Response {
    fn assert_status_eq(&self, status: u16) {
        assert_eq!(self.status().as_u16(), status);
    }
}