use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_map_request_url_and_query() {
    let server = mount("req/url-query/url-single");
    let uri = format!("{}/api/url?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_missing_query() {
    let server = mount("req/url-query/url-single");
    let uri = format!("{}/api/url", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_fail_when_incorrect_path() {
    let server = mount("req/url-query/url-single");
    let uri = format!("{}/api/not-url?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_fail_when_incorrect_query() {
    let server = mount("req/url-query/url-single");
    let uri = format!("{}/api/url?age=old", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_url_and_many_query() {
    let server = mount("req/url-query/url-many");
    let uri = format!("{}/api/url?age=young&city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_incorrect_uri_with_many_queries() {
    let server = mount("req/url-query/url-many");
    let uri = format!("{}/api/not-url?age=young&city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_fail_when_one_of_queries_does_not_match() {
    let server = mount("req/url-query/url-many");
    let uri = format!("{}/api/url?age=old&city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let uri = format!("{}/api/url?age=young&city=lyon", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let uri = format!("{}/api/url?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let uri = format!("{}/api/url?city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}
