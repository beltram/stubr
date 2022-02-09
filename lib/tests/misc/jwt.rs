use asserhttp::*;
use surf::get;

mod eq {
    use super::*;

    const TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.NHVaYe26MbtOYhSKkoKYdFVomg4i8ZJd8_-RU8VNbftc4TSMb4bXP3l3YlNWACwyXPGffz5aXHc6lty1Y2t4SWRqGteragsVdZufDn5BlnJl9pdR_kdVFUsra2rWKEofkZeIC4yWytE58sMIihvo9H1ScmmVwBcQP6XETqYd0aSHp1gOa9RdUPDvoXQ5oqygTqVtxaDr6wUFKrKItgBMzWIdNZ6y7O9E0DhEPTbE9rfBo6KTFsHAZnMg4k68CDp2woYIaXbmYTWcvbzIuHO7_37GT79XdIwkm95QJ7hYC9RiwrV7mesbY4PAahERJawntho0my942XheVLmGwLMBkQ";

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_match_plain_token_by_equality() {
        get(stubr.uri())
            .header("Authorization", format!("Bearer {}", TOKEN)).await
            .expect_status_ok();
    }

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_fail_when_not_eq() {
        get(stubr.uri())
            .header("Authorization", format!("Bearer A{}", TOKEN)).await
            .expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_fail_when_missing_bearer_prefix() {
        get(stubr.uri())
            .header("Authorization", TOKEN).await
            .expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_fail_when_authorization_header_absent() {
        get(stubr.uri()).await.expect_status_not_found();
    }
}
