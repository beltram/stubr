use std::convert::TryInto;

use stubr::mapper::StubrMock;

use crate::utils::stub;

mod utils;

#[test]
fn should_map_response_status_200() {
    let mock: StubrMock = stub("resp/status/200").try_into().unwrap();
    assert_eq!(mock.0.response().status(), 200);
}

#[test]
fn should_map_response_status_400() {
    let mock: StubrMock = stub("resp/status/400").try_into().unwrap();
    assert_eq!(mock.0.response().status(), 400);
}

#[test]
fn should_map_response_status_500() {
    let mock: StubrMock = stub("resp/status/500").try_into().unwrap();
    assert_eq!(mock.0.response().status(), 500);
}