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

mod alg {
    use super::*;

    const RS_256_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.e30.upI8kdqCUNUUgd1IrUpjNDDiif7yJZT_pI03g_DW6-aFIxEZD_kszt6E33_cjiUv6tWkutqTDgLr8XfKzFVfBKUTA9QDhpY9Imavnu-CW5k6xSUdiSiwo5b7EyMGBO7bRPN9b0L3OL2CzqowqOalYiqY0lldy1IDUgD_n5Cm0CFLpMOipb_vGf2KJFYmR8T_oZOAJzf6FYbZKFhjujeXiVLah2kj2qIZMIws9Q5t485udznl_gNlQwcnVB3bqEd6_msgUOo0ZRkyctQz9rZ70-JBviwXzhiqoeDGeiqJeRbaWLOjhmpWlwc6DJgRgP1H59dzV9htOWjST6cs8vpG2A";
    const HS_256_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.e30.Et9HFtf9R3GEMA0IICOfFMVXY7kkTX1wr4qCyhIf58U";

    mod eq {
        use super::*;

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/rs-256.json")]
        async fn should_match_alg() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN)).await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/rs-256.json")]
        async fn should_fail_when_different_alg() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN)).await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/rs-256.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", RS_256_TOKEN).await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/rs-256.json")]
        async fn should_fail_when_missing_authorization_header() {
            get(stubr.uri()).await.expect_status_not_found();
        }
    }

    mod one_of {
        use super::*;

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-rs-hs-256.json")]
        async fn should_match_one_of_alg() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN)).await
                .expect_status_ok();
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN)).await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-rs-256.json")]
        async fn should_fail_when_absent() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN)).await
                .expect_status_ok();
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN)).await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-empty.json")]
        async fn should_fail_when_one_of_empty() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN)).await
                .expect_status_not_found();
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN)).await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-rs-hs-256.json")]
        async fn should_fail_when_authorization_header_absent() {
            get(stubr.uri()).await.expect_status_not_found();
        }


        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-rs-hs-256.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", RS_256_TOKEN).await
                .expect_status_not_found();
        }
    }
}
