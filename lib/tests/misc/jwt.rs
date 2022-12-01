use asserhttp::*;
use surf::get;

mod eq {
    use super::*;

    const TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.NHVaYe26MbtOYhSKkoKYdFVomg4i8ZJd8_-RU8VNbftc4TSMb4bXP3l3YlNWACwyXPGffz5aXHc6lty1Y2t4SWRqGteragsVdZufDn5BlnJl9pdR_kdVFUsra2rWKEofkZeIC4yWytE58sMIihvo9H1ScmmVwBcQP6XETqYd0aSHp1gOa9RdUPDvoXQ5oqygTqVtxaDr6wUFKrKItgBMzWIdNZ6y7O9E0DhEPTbE9rfBo6KTFsHAZnMg4k68CDp2woYIaXbmYTWcvbzIuHO7_37GT79XdIwkm95QJ7hYC9RiwrV7mesbY4PAahERJawntho0my942XheVLmGwLMBkQ";

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_match_plain_token_by_equality() {
        get(stubr.uri())
            .header("Authorization", format!("Bearer {}", TOKEN))
            .await
            .expect_status_ok();
    }

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_fail_when_not_eq() {
        get(stubr.uri())
            .header("Authorization", format!("Bearer A{}", TOKEN))
            .await
            .expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("req/jwt/eq.json")]
    async fn should_fail_when_missing_bearer_prefix() {
        get(stubr.uri()).header("Authorization", TOKEN).await.expect_status_not_found();
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
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/rs-256.json")]
        async fn should_fail_when_different_alg() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/rs-256.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", RS_256_TOKEN)
                .await
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
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN))
                .await
                .expect_status_ok();
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-rs-256.json")]
        async fn should_fail_when_absent() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN))
                .await
                .expect_status_ok();
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/alg/one-of-empty.json")]
        async fn should_fail_when_one_of_empty() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", RS_256_TOKEN))
                .await
                .expect_status_not_found();
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", HS_256_TOKEN))
                .await
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
                .header("Authorization", RS_256_TOKEN)
                .await
                .expect_status_not_found();
        }
    }
}

mod payload {
    use super::*;

    const JDOE_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Im5hbWUiOiJqZG9lIn19.FSVsnc7d62ZzcwwrgcVbSzZdbOYUoGJat264XovRFQ2bmnYZcmdg-WLsNZ0L0evgVMMW2YN0R7QemUuFF9BFH9Joev9fwPtufui1QkyGAsm1mMaC2rBAfK6OW8DLBP9sJSXUBqDPvZ2oMGm7wpBvh3gwpvh088ZlPOUq_iWl0q2SHARA0nUBMDgkVkffP2lQbKIFt5fxV30yQlmcxeDJBHibSJk73D9qGUXLXJDZ5KUXP6__yNKz1ghcP8e_491TwJ-2nVyKnPmgQYGbyw2EiA9P7v67A_tBOH-BVAynhJ2bi9WLvZrJBWoAtbsQvuoGEMrrqu7hwsq9ihctcr6iDw";
    const ADOE_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Im5hbWUiOiJhZG9lIn19.A9O_XKDbQ1iVNzIWbPSXHFmyJPjUb-MIal0WNqS9sZ13Ex5W8KgEzQLbZWJK_C0910XHGF4o3xDWgTZ9uznEs3Wp73eN47BdjZ-VNU9vPwJZB7BqQIUeWa7mbsqEx1nNQLV-hMb1sG8xFN4iBjrtVgnBeTcNlM3IiSnoYrbx3cz-_SvjXZg7uIIHl0dZPRqaTRaQENyN_t4xFETL02EIerxLZB0laxxCKda32B-mXaLpQsAIK3A_XQvEY6uGB1EWrqWz0SPl_s72isfak2AF4yZr2n7xDqLCUnRe8LkXCsxPup76MeaGMFNyIgSppuR7y4mhDNw3OqpFj2nSt0sOQQ";
    const FRENCH_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Im5vbSI6Impkb2UifX0.XB_p_8wpsmIpYfFKarpBopxROFT1l8R11KzXW2_8whPqQT-oZvyvAfjujcKDZrbrbeVsRxeV2HTnjX7noQhdeXp7TgZizTmw-66vKpdkEx5VOgYohOL4hYtZivuYZS9g6PqGJBcpfyZh-CWcb0YK7GUdVkHr2bVYcveTlVaLq1T7vFWbo3vXhrWDf37VKMvAH2HFrtsAuBD9yDTwq-O8mH6m3PVgOXlRv-RXlRXjA84uM4AkIB3PdVUELuvNKqmNqedWoqwA4JlpGRCDcE5kM4jhJFYCWOXJ0n0peK70etVJ4gk-nFckY8sBN_qzBOgKItTqQBLrmRdyptJ0mtaHDg";
    const VOID_TOKEN: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.e30.upI8kdqCUNUUgd1IrUpjNDDiif7yJZT_pI03g_DW6-aFIxEZD_kszt6E33_cjiUv6tWkutqTDgLr8XfKzFVfBKUTA9QDhpY9Imavnu-CW5k6xSUdiSiwo5b7EyMGBO7bRPN9b0L3OL2CzqowqOalYiqY0lldy1IDUgD_n5Cm0CFLpMOipb_vGf2KJFYmR8T_oZOAJzf6FYbZKFhjujeXiVLah2kj2qIZMIws9Q5t485udznl_gNlQwcnVB3bqEd6_msgUOo0ZRkyctQz9rZ70-JBviwXzhiqoeDGeiqJeRbaWLOjhmpWlwc6DJgRgP1H59dzV9htOWjST6cs8vpG2A";

    mod eq {
        use super::*;

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/eq/obj.json")]
        async fn should_match_payload_by_equality() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", JDOE_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/eq/obj.json")]
        async fn should_fail_when_payload_does_not_match() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", ADOE_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/eq/obj.json")]
        async fn should_fail_when_payload_empty() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", VOID_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/eq/obj.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", JDOE_TOKEN)
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/eq/obj.json")]
        async fn should_fail_when_missing_authorization_header() {
            get(stubr.uri()).await.expect_status_not_found();
        }
    }

    mod json_path {
        use super::*;

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/eq.json")]
        async fn should_match_payload_by_json_path() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", JDOE_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/eq.json")]
        async fn should_match_payload_by_json_path_regardless_json_value() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", ADOE_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/eq.json")]
        async fn should_fail_when_json_path_does_not_match() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", FRENCH_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/eq.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", JDOE_TOKEN)
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/eq.json")]
        async fn should_fail_when_missing_authorization_header() {
            get(stubr.uri()).await.expect_status_not_found();
        }
    }

    mod json_path_eq {
        use super::*;

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-eq.json")]
        async fn should_match_payload_by_equality() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", JDOE_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-eq.json")]
        async fn should_fail_when_payload_does_not_match() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", ADOE_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-eq.json")]
        async fn should_fail_when_payload_empty() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", VOID_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-eq.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", JDOE_TOKEN)
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-eq.json")]
        async fn should_fail_when_missing_authorization_header() {
            get(stubr.uri()).await.expect_status_not_found();
        }
    }

    mod json_path_contains {
        use super::*;

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-contains.json")]
        async fn should_match_payload_when_contains() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", JDOE_TOKEN))
                .await
                .expect_status_ok();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-contains.json")]
        async fn should_fail_when_payload_does_not_match() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", ADOE_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-contains.json")]
        async fn should_fail_when_payload_empty() {
            get(stubr.uri())
                .header("Authorization", format!("Bearer {}", VOID_TOKEN))
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-contains.json")]
        async fn should_fail_when_missing_bearer_prefix() {
            get(stubr.uri())
                .header("Authorization", JDOE_TOKEN)
                .await
                .expect_status_not_found();
        }

        #[async_std::test]
        #[stubr::mock("req/jwt/payload/json-path/path-contains.json")]
        async fn should_fail_when_missing_authorization_header() {
            get(stubr.uri()).await.expect_status_not_found();
        }
    }
}
