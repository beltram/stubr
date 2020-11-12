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
fn insensitive_should_fail_when_missing() {
    let server = mount("req/query/case/insensitive");
    let response = block_on(surf::get(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_many_case_insensitive() {
    let server = mount("req/query/case/insensitive-many");
    let uri = format!("{}?age=YOUNG&city=PARIS", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let uri = format!("{}?age=young&city=PARIS", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let uri = format!("{}?age=young&city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_with_many_case_insensitive_string_value_when_one_of_does_not_match() {
    let server = mount("req/query/case/insensitive-many");
    let uri = format!("{}?age=old&city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let uri = format!("{}?age=young&city=lyon", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let uri = format!("{}?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let uri = format!("{}?city=paris", server.uri());
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

#[test]
fn sensitive_should_fail_when_missing() {
    let server = mount("req/query/case/sensitive");
    let response = block_on(surf::get(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}
