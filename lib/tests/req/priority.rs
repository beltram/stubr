use asserhttp::*;

#[async_std::test]
#[stubr::mock("req/priority/exact-high.json", "req/priority/regex-low.json")]
async fn should_prioritize_exact_as_highest_priority() {
    surf::get(stubr.path("/abcd")).await.expect_status_eq(200);
}

#[async_std::test]
#[stubr::mock("req/priority/exact-low.json", "req/priority/regex-high.json")]
async fn should_prioritize_regex_as_highest_priority() {
    surf::get(stubr.path("/abcd")).await.expect_status_eq(201);
}
