#[tokio::test]
#[stubr::mock(full_path = "tests/book/hello.json", port = 1234, verify = true)]
async fn getting_started() {}