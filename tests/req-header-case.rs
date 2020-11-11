use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_default_to_case_sensitive() {
    let server = mount("req/headers/equal/string");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "Application/Json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_case_insensitive() {
    let server = mount("req/headers/case/insensitive");
    let response_upper = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "Application/Json")).unwrap();
    assert_eq!(response_upper.status().as_u16(), 200);
    let response_lower = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response_lower.status().as_u16(), 200);
}

#[test]
fn insensitive_should_fail_when_invalid_value() {
    let server = mount("req/headers/case/insensitive");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/xml")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn insensitive_should_fail_when_invalid_key() {
    let server = mount("req/headers/case/insensitive");
    let response = block_on(surf::get(&server.uri())
        .set_header("Not-Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_many_case_insensitive() {
    let server = mount("req/headers/case/insensitive-many");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "Application/Json")
        .set_header("Accept", "Application/Json")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "Application/Json")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/json")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_request_many_case_insensitive_string_value_when_one_of_does_not_match() {
    let server = mount("req/headers/case/insensitive-many");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/xml")
        .set_header("Accept", "application/json")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/xml")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_explicit_case_sensitive() {
    let server = mount("req/headers/case/sensitive");
    let response_upper = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "Application/Json")).unwrap();
    assert_eq!(response_upper.status().as_u16(), 404);
    let response_lower = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response_lower.status().as_u16(), 200);
}

#[test]
fn sensitive_should_fail_when_invalid_value() {
    let server = mount("req/headers/case/sensitive");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/xml")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn sensitive_should_fail_when_invalid_key() {
    let server = mount("req/headers/case/sensitive");
    let response = block_on(surf::get(&server.uri())
        .set_header("Not-Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}