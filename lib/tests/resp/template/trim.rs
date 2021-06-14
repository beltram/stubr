use asserhttp::*;
use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_trim() {
    let srv = given("resp/template/trim/single");
    post(&srv.uri()).body("   a b ").await
        .expect_status_ok()
        .expect_body_text_eq("a b")
        .expect_content_type_text();
}