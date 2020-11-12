use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_not_default_to_contains() {
    let server = mount("req/headers/equal/string");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_contains() {
    let server = mount("req/headers/contains/single");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_does_not_contain() {
    let server = mount("req/headers/contains/single");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/xml")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_fail_when_invalid_key() {
    let server = mount("req/headers/contains/single");
    let response = block_on(surf::get(&server.uri())
        .set_header("Not-Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_fail_when_missing() {
    let server = mount("req/headers/contains/single");
    let response = block_on(surf::get(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_many_contains() {
    let server = mount("req/headers/contains/many");
    let response =
        block_on(surf::get(&server.uri())
            .set_header("Content-Type", "application/json")
            .set_header("Accept", "application/json")
        ).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_one_of_does_not_contains() {
    let server = mount("req/headers/contains/many");
    let response =
        block_on(surf::get(&server.uri())
            .set_header("Content-Type", "application/xml")
            .set_header("Accept", "application/json")
        ).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let response =
        block_on(surf::get(&server.uri())
            .set_header("Content-Type", "application/json")
            .set_header("Accept", "application/xml")
        ).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let response =
        block_on(surf::get(&server.uri())
            .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
    let response =
        block_on(surf::get(&server.uri())
            .set_header("Accept", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_contains_begin() {
    let server = mount("req/headers/contains/begin");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_support_contains_middle() {
    let server = mount("req/headers/contains/middle");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_support_contains_end() {
    let server = mount("req/headers/contains/end");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}