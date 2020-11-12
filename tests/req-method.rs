use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_map_request_method_get() {
    let server = mount("req/method/get");
    let response = block_on(surf::get(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_map_request_method_post() {
    let server = mount("req/method/post");
    let response = block_on(surf::post(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_map_request_method_put() {
    let server = mount("req/method/put");
    let response = block_on(surf::put(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_map_request_method_delete() {
    let server = mount("req/method/delete");
    let response = block_on(surf::delete(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_map_request_method_patch() {
    let server = mount("req/method/patch");
    let response = block_on(surf::patch(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

// TODO figure out why hanging on CI
/*#[test]
fn should_map_request_method_head() {
    let server = mount("req/method/head");
    let response = block_on(surf::head(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}*/

#[test]
fn should_map_request_method_options() {
    let server = mount("req/method/options");
    let response = block_on(surf::options(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_map_request_method_connect() {
    let server = mount("req/method/connect");
    let response = block_on(surf::connect(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_map_request_method_trace() {
    let server = mount("req/method/trace");
    let response = block_on(surf::trace(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[test]
fn should_fail_when_invalid_method() {
    let server = mount("req/method/get");
    let response = block_on(surf::post(&server.uri())).unwrap();
    assert_eq!(response.status().as_u16(), 404);
}
