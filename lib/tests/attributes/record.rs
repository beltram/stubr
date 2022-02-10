use asserhttp::*;

use crate::utils::*;

mod smoke {
    use super::*;

    #[stubr::record]
    #[stubr::mock("record/attributes/ok.json")]
    #[test]
    fn should_succeed() {
        recorder.isahc_client().get(stubr.path("/smoke/ok")).expect_status_ok();
        assert_recorded_stub_exists("smoke-ok-3885772517266612307")
    }

    #[should_panic]
    #[stubr::mock("record/attributes/ok.json")]
    #[stubr::record]
    #[test]
    fn should_fail() {
        recorder.isahc_client().get(stubr.path("/smoke/ok")).expect_status_server_error();
    }

    #[stubr::record]
    #[stubr::mock("record/attributes/ok-async.json")]
    #[test]
    async fn async_should_succeed() {
        recorder.reqwest_client().get(stubr.path("/smoke/async/ok"))
            .send().await
            .expect_status_ok();
        assert_recorded_stub_exists("smoke-async-ok-6996574416851633654")
    }

    #[should_panic]
    #[stubr::mock("record/attributes/ok-async.json")]
    #[stubr::record]
    #[test]
    async fn async_should_fail() {
        recorder.reqwest_client().get(stubr.path("/smoke/async/ok"))
            .send().await
            .expect_status_server_error();
    }
}

mod port {
    use isahc::{config::Configurable, HttpClient};

    use super::*;

    #[stubr::record(port = 3241)]
    #[stubr::mock("record/attributes/port.json")]
    #[test]
    fn should_succeed() {
        let client = HttpClient::builder().proxy("http://127.0.0.1:3241".parse().ok()).build().unwrap();
        client.get(stubr.path("/smoke/port")).expect_status_ok();
        assert_recorded_stub_exists("smoke-port-16104439783116088716")
    }

    #[should_panic]
    #[stubr::mock("record/attributes/port.json")]
    #[stubr::record(port = 4040)]
    #[test]
    fn should_fail() {
        let client = HttpClient::builder().proxy("http://127.0.0.1:8080".parse().ok()).build().unwrap();
        client.get(stubr.path("/smoke/port")).expect_status_ok();
    }
}