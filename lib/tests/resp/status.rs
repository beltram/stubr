use crate::*;

// Stubr only
#[async_std::test]
#[stubr::mock("resp/status/default.json")]
async fn status_should_default_to_200() {
    client::get(stubr.uri()).await.expect_status(200);
}

#[stubr::iso_test("resp/status/200.json")]
async fn should_map_response_status_200<'a>() {
    client::get(stubr.uri()).await.is_ok_iso().expect_status(200);
}

#[stubr::iso_test("resp/status/400.json")]
async fn should_map_response_status_400() {
    client::get(stubr.uri()).await.is_error_iso().expect_status(400);
}

#[stubr::iso_test("resp/status/500.json")]
async fn should_map_response_status_500() {
    client::get(stubr.uri()).await.is_error_iso().expect_status(500);
}
