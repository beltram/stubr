use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_default_to_case_sensitive() {
    let server = mount("req/headers/exact");
    let response =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "Application/Json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_case_insensitive() {
    let server = mount("req/headers/exact-case-insensitive");
    let response_upper =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "Application/Json")).unwrap();
    assert_eq!(response_upper.status().as_u16(), 200);
    let response_lower =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response_lower.status().as_u16(), 200);
}

#[test]
fn should_support_explicit_case_sensitive() {
    let server = mount("req/headers/explicit-case-sensitive");
    let response_upper =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "Application/Json")).unwrap();
    assert_eq!(response_upper.status().as_u16(), 404);
    let response_lower =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response_lower.status().as_u16(), 200);
}
