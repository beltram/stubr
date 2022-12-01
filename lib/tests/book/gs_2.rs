use asserhttp::*;

#[tokio::test]
#[stubr::mock("hello.json")] // 👈 this starts the mock server
async fn getting_started() {
    // a local binding 'stubr' has been created, equivalent to the one before
    let uri = stubr.path("/hello");
    reqwest::get(uri)
        .await
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("Hello stubr");
}
