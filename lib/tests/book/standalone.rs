#[tokio::test(flavor = "multi_thread")]
async fn record_standalone() {
    // 👇 spawn the recording proxy
    let proxy = stubr::Stubr::record();
    // or use `record_with()` for configuring it
    let _proxy_uri = proxy.uri();
    // ☝️ then use this uri to configure your http client
}
