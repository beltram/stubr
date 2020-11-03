use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_not_default_to_contains() {
    let server = mount("req/headers/exact");
    let response =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_contains() {
    let server = mount("req/headers/contains");
    let response =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_does_not_contain() {
    let server = mount("req/headers/contains");
    let response =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "application/xml")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}