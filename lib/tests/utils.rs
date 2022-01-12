use std::{env::current_dir, fs::File, io::Read, path::{Path, PathBuf}};

use serde_json::Value;

use stubr::RecordConfig;
pub use stubr::Stubr;

pub trait UriAndQuery {
    fn get_uri(&self) -> String;

    fn path_query(&self, path: &str, key: &str, value: &str) -> String {
        format!("{}{}?{}={}", self.get_uri(), path, key, value)
    }

    fn query(&self, key: &str, value: &str) -> String {
        format!("{}?{}={}", self.get_uri(), key, value)
    }

    fn path_queries(&self, path: &str, q1: (&str, &str), q2: (&str, &str)) -> String {
        format!("{}{}?{}={}&{}={}", self.get_uri(), path, q1.0, q1.1, q2.0, q2.1)
    }

    fn queries(&self, q1: (&str, &str), q2: (&str, &str)) -> String {
        format!("{}?{}={}&{}={}", self.get_uri(), q1.0, q1.1, q2.0, q2.1)
    }
}

impl UriAndQuery for Stubr {
    fn get_uri(&self) -> String { self.uri() }
}

pub fn assert_recorded_stub_eq(id: &str, expected: Value) {
    let mut content = String::new();
    File::open(stub_file(id)).unwrap().read_to_string(&mut content).unwrap();
    let content: Value = serde_json::from_str(content.as_str()).unwrap();
    assert_eq!(content, expected);
}

pub fn assert_recorded_stub_exists(id: &str) {
    assert!(stub_file(id).exists());
}

fn stub_file(id: &str) -> PathBuf {
    target_dir()
        .join("stubs")
        .join("localhost")
        .join(format!("{}.json", id))
}

pub fn target_dir() -> PathBuf {
    current_dir().ok()
        .and_then(|c| c.parent().map(Path::to_path_buf))
        .map(|p| p.join("target"))
        .unwrap()
}

pub fn record_cfg() -> RecordConfig {
    RecordConfig {
        except_request_headers: Some(relaxed_req_headers()),
        except_response_headers: Some(relaxed_resp_headers()),
        ..Default::default()
    }
}

pub fn relaxed_req_headers() -> Vec<&'static str> {
    vec![
        "accept", "accept-encoding", "content-type", "host", "proxy-connection", "user-agent",
        "expect", "transfer-encoding", "content-length",
    ]
}

pub fn relaxed_resp_headers() -> Vec<&'static str> {
    vec!["date", "content-length", "content-type", "server"]
}

pub fn resp_headers_with_content_type() -> Vec<&'static str> {
    vec!["date", "content-length", "server"]
}
