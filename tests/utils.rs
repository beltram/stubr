#![allow(dead_code)]

use std::path::PathBuf;
use std::str::FromStr;

use async_std::task::block_on;
use http_types::headers::{HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use serde::export::fmt::Debug;
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
    fn assert_status_eq(&mut self, status: u16) -> &mut Self;
    fn assert_ok(&mut self) -> &mut Self { self.assert_status_eq(200) }
    fn assert_not_found(&mut self) -> &mut Self { self.assert_status_eq(404) }
    fn assert_body_text(&mut self, body: &str) -> &mut Self;
    fn assert_body_json<T>(&mut self, body: T) -> &mut Self where T: DeserializeOwned + PartialEq + Debug;
    fn assert_body_empty(&mut self) -> &mut Self { self.assert_body_text("") }
    fn assert_header(&mut self, key: &str, value: &str) -> &mut Self;
}

impl ResponseAsserter for Response {
    fn assert_status_eq(&mut self, status: u16) -> &mut Self {
        assert_eq!(u16::from(self.status()), status);
        self
    }

    fn assert_body_text(&mut self, body: &str) -> &mut Self {
        assert_eq!(block_on(self.body_string()).unwrap(), body);
        self
    }

    fn assert_body_json<T>(&mut self, body: T) -> &mut Self where T: DeserializeOwned + PartialEq + Debug {
        assert_eq!(block_on(self.body_json::<T>()).unwrap(), body);
        self
    }

    fn assert_header(&mut self, key: &str, value: &str) -> &mut Self {
        let key = HeaderName::from_str(key).unwrap();
        let value = HeaderValue::from_str(value).unwrap();
        assert_eq!(self.header(key).unwrap().last(), &value);
        self
    }
}