use asserhttp::*;

#[tokio::test]
async fn getting_started() {
    // run a mock server with the stub 👇
    let stubr = stubr::Stubr::start("tests/stubs/hello.json").await;
    // or use 'start_blocking' for a non-async version

    // the mock server started on a random port e.g. '127.0.0.1:43125'
    // so we use the stub instance 'path' (or 'uri') method to get the address back
    let uri = stubr.path("/hello");
    reqwest::get(uri).await
        // (optional) use asserhttp for assertions
        .expect_status_ok()
        .expect_content_type_text()
        .expect_body_text_eq("Hello stubr");
}