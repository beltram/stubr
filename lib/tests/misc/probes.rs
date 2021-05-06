use surf::get;

use crate::utils::*;

#[async_std::test]
async fn should_publish_probes_when_started() {
    let srv = given("ping");
    get(&srv.path("/healtz")).await.unwrap().assert_ok();
}