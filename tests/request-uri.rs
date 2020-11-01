use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_map_request_url_path_uri() {
    let server = mount("req/url/url-path");
    let uri = format!("{}/api/exact-uri", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200)
}

#[test]
fn should_not_match_when_url_path_not_exact() {
    let server = mount("req/url/url-path");
    let uri = format!("{}/api/not-exact-uri", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404)
}

#[test]
fn should_map_request_url_path_pattern_uri() {
    let server = mount("req/url/url-path-pattern");
    let uri = format!("{}/api/regex-uri/abcd", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200)
}

#[test]
fn should_not_match_when_url_path_pattern_not_satisfied() {
    let server = mount("req/url/url-path-pattern");
    let uri = format!("{}/api/regex-uri/1234", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404)
}

#[test]
fn should_not_fail_when_no_url() {
    let server = mount("req/url/no-url");
    let response = block_on(surf::get(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200)
}
