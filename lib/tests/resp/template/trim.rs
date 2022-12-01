use asserhttp::*;
use surf::post;

#[async_std::test]
#[stubr::mock("resp/template/trim/single.json")]
async fn should_trim() {
    post(stubr.uri())
        .body("   a b ")
        .await
        .expect_status_ok()
        .expect_body_text_eq("a b")
        .expect_content_type_text();
}
