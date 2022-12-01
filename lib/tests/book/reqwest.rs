use asserhttp::*;
use stubr::Record as _;

#[test]
#[stubr::mock("ping.json")] // 👈 spawn a mock server
fn record_reqwest_trait() {
    // recording unfortunately requires using reqwest's builder hence the syntax is a bit verbose
    let req = reqwest::blocking::ClientBuilder::new().build().unwrap()
        .get(stubr.uri())
        // 👇 this will intercept and dump all http traffic going through this client
        .record() // or `record_with()` for configuring it
        .build().unwrap();
    reqwest::blocking::Client::default().execute(req).unwrap().expect_status_ok();
}
