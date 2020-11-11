use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_not_map_when_invalid_key() {
    let server = mount("req/headers/equal/string");
    let response = block_on(surf::get(&server.uri())
        .set_header("Not-Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_exact_string_value() {
    let server = mount("req/headers/equal/string");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_when_incorrect_string_value() {
    let server = mount("req/headers/equal/string");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/xml")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_many_exact_string_value() {
    let server = mount("req/headers/equal/string-many");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "application/json")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_request_many_exact_string_value_when_one_of_does_not_match() {
    let server = mount("req/headers/equal/string-many");
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
fn should_map_request_exact_int_value() {
    let server = mount("req/headers/equal/int");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "42")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_when_incorrect_int_value() {
    let server = mount("req/headers/equal/int");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "43")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_not_map_when_not_an_int_value() {
    let server = mount("req/headers/equal/int");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_exact_bool_value() {
    let server = mount("req/headers/equal/bool");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "true")).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_not_map_when_incorrect_bool_value() {
    let server = mount("req/headers/equal/bool");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "false")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_not_map_when_not_an_bool_value() {
    let server = mount("req/headers/equal/bool");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_map_request_many_exact_string_and_int_value() {
    let server = mount("req/headers/equal/string-int");
    let response = block_on(surf::get(&server.uri())
        .set_header("Content-Type", "application/json")
        .set_header("Accept", "true")
    ).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}