use asserhttp::*;

use crate::utils::*;

mod smoke {
    use super::*;

    #[stubr::record]
    #[stubr::mock("record/attributes/ok.json")]
    #[test]
    fn should_succeed() {
        recorder.isahc_client().get(stubr.path("/smoke/ok")).expect_status_ok();
        assert_recorded_stub_exists("smoke-ok-9931321115210409608")
    }

    #[should_panic]
    #[stubr::record]
    #[stubr::mock("record/attributes/ok.json")]
    #[test]
    fn should_fail() {
        recorder.isahc_client().get(stubr.path("/smoke/ok")).expect_status_server_error();
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
        assert_recorded_stub_exists("smoke-port-7681494402952634027")
    }

    #[should_panic]
    #[stubr::record(port = 4040)]
    #[stubr::mock("record/attributes/port.json")]
    #[test]
    fn should_fail() {
        let client = HttpClient::builder().proxy("http://127.0.0.1:8080".parse().ok()).build().unwrap();
        client.get(stubr.path("/smoke/port")).expect_status_ok();
    }
}