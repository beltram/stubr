use asserhttp::*;
use surf::get;

use crate::utils::*;

#[async_std::test]
#[stubr::mock]
async fn should_publish_probes_when_started() {
    get(stubr.path("/healtz")).await.expect_status_ok();
}