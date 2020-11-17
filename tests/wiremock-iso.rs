use surf::get;

use crate::utils::*;

mod utils;

#[async_std::test]
async fn should_always_return_server_header() {
    let srv = given("ping");
    let expected = format!("stubr({})", env!("CARGO_PKG_VERSION"));
    get(&srv.uri()).await.unwrap()
        .assert_ok()
        .assert_header("Server", &expected);
}