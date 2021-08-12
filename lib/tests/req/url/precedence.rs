use asserhttp::*;
use surf::get;

use crate::utils::*;

mod url {
    use super::*;

    #[async_std::test]
    #[stubr::mock("req/url-precedence/url-and-url-path.json")]
    async fn should_prefer_url_over_url_path() {
        get(stubr.path_query("/api/url", "age", "42")).await.expect_status_ok();
        get(stubr.path("/api/exact-url")).await.expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("req/url-precedence/url-and-url-pattern.json")]
    async fn should_prefer_url_over_url_pattern() {
        get(stubr.path_query("/api/url", "age", "42")).await.expect_status_ok();
        get(stubr.path_query("/api/pattern", "one", "abcd")).await.expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("req/url-precedence/url-and-url-path-pattern.json")]
    async fn should_prefer_url_over_url_path_pattern() {
        get(stubr.path_query("/api/url", "age", "42")).await.expect_status_ok();
        get(stubr.path("/api/regex-uri/abcd")).await.expect_status_not_found();
    }
}

mod url_path {
    use super::*;

    #[async_std::test]
    #[stubr::mock("req/url-precedence/url-path-and-url-pattern.json")]
    async fn should_prefer_url_path_over_url_pattern() {
        get(stubr.path("/api/exact-url")).await.expect_status_ok();
        get(stubr.path_query("/api/pattern", "one", "abcd")).await.expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("req/url-precedence/url-path-and-url-path-pattern.json")]
    async fn should_prefer_url_path_over_url_path_pattern() {
        get(stubr.path("/api/exact-url")).await.expect_status_ok();
        get(stubr.path("/api/regex-uri/abcd")).await.expect_status_not_found();
    }
}

mod url_pattern {
    use super::*;

    #[async_std::test]
    #[stubr::mock("req/url-precedence/url-pattern-and-url-path-pattern.json")]
    async fn should_prefer_url_pattern_over_url_path_pattern() {
        get(stubr.path_query("/api/pattern", "one", "abcd")).await.expect_status_ok();
        get(stubr.path("/api/regex-uri/abcd")).await.expect_status_not_found();
    }
}
