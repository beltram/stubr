// works for async as well
#[stubr::record] // 👈 this spawns the proxy and creates a 'recorder' binding in the function
// #[stubr::record(port = 1234)] for setting a port
#[test]
fn record_standalone() {
    let _proxy_uri = recorder.uri();
    // ☝️ then use this uri to configure your http client
}
