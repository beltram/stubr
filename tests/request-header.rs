use async_std::task::block_on;

use crate::utils::*;

mod utils;

#[test]
fn should_map_request_exact_header() {
    let server = mount("req/headers/exact");
    let response =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "application/json")).unwrap();
    assert_eq!(response.status().as_u16(), 200)
}

#[test]
fn should_not_match_when_not_exact_header_key() {
    let server = mount("req/headers/exact");
    let response =
        block_on(surf::get(&server.uri()).set_header("Not-Content-Type", "application/json"))
            .unwrap();
    assert_eq!(response.status().as_u16(), 404)
}

#[test]
fn should_not_match_when_not_exact_header_value() {
    let server = mount("req/headers/exact");
    let response =
        block_on(surf::get(&server.uri()).set_header("Content-Type", "application/not-json"))
            .unwrap();
    assert_eq!(response.status().as_u16(), 404)
}
