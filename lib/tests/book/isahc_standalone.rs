use asserhttp::*;

#[tokio::test(flavor = "multi_thread")] // 👈 required by recording proxy
#[stubr::mock("ping.json")] // 👈 spawn a mock server
async fn record_isahc() {
    // 👇 spawn the recording proxy
    stubr::Stubr::record() // or `record_with()` for configuring it
        // 👇 builds an isahc client with proxy configured
        .isahc_client()
        .get(stubr.uri())
        .expect_status_ok();
}