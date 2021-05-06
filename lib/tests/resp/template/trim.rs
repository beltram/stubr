use surf::post;

use crate::utils::*;

#[async_std::test]
async fn should_trim() {
    let srv = given("resp/template/trim/single");
    post(&srv.uri()).body("   a b ").await.unwrap()
        .assert_ok()
        .assert_body_text("a b")
        .assert_content_type_text();
}