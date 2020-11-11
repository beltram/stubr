use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_not_default_to_contains() {
    let server = mount("req/query/equal/string");
    let uri = format!("{}?age=u", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_contains() {
    let server = mount("req/query/contains/single");
    let uri = format!("{}?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_does_not_contain() {
    let server = mount("req/query/contains/single");
    let uri = format!("{}?age=old", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_fail_when_invalid_key() {
    let server = mount("req/query/contains/single");
    let uri = format!("{}?not-age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}

#[test]
fn should_support_many_contains() {
    let server = mount("req/query/contains/many");
    let uri = format!("{}?age=young&city=paris", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_one_of_does_not_contains() {
    let server = mount("req/query/contains/many");
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
fn should_support_contains_begin() {
    let server = mount("req/query/contains/begin");
    let uri = format!("{}?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_support_contains_middle() {
    let server = mount("req/query/contains/middle");
    let uri = format!("{}?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_support_contains_end() {
    let server = mount("req/query/contains/end");
    let uri = format!("{}?age=young", server.uri());
    let response = block_on(surf::get(&uri)).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}