use asserhttp::*;

#[tokio::test(flavor = "multi_thread")] // 👈 required by recording proxy
#[stubr::mock("ping.json")] // 👈 spawn a mock server
async fn record_reqwest() {
    // 👇 spawn the recording proxy
    stubr::Stubr::record() // or `record_with()` for configuring it
        // 👇 builds a reqwest client with proxy configured
        .reqwest_client()
        .get(stubr.uri())
        .send()
        .await
        .expect_status_ok();
}
