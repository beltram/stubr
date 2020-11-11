use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_default_to_case_sensitive() {
    let server = mount("req/query/equal/string");
    let uri = format!("{}?age=YOUNG", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_case_insensitive() {
    let server = mount("req/query/case/insensitive");
    let uri = format!("{}?age=YOUNG", server.uri());
    let response_upper = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response_upper.status().as_u16(), 200);
    let uri = format!("{}?age=young", server.uri());
    let response_lower = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response_lower.status().as_u16(), 200);
}

#[test]
fn insensitive_should_fail_when_invalid_key() {
    let server = mount("req/query/case/insensitive");
    let uri = format!("{}?not-age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_explicit_case_sensitive() {
    let server = mount("req/query/case/sensitive");
    let uri = format!("{}?age=YOUNG", server.uri());
    let response_upper = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response_upper.status().as_u16(), 404);
    let uri = format!("{}?age=young", server.uri());
    let response_lower = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response_lower.status().as_u16(), 200);
}

#[test]
fn sensitive_should_fail_when_invalid_key() {
    let server = mount("req/query/case/sensitive");
    let uri = format!("{}?not-age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}
