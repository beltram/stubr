use asserhttp::*;

mod smoke {
    use super::*;

    #[test]
    #[stubr::mock]
    fn should_succeed() {
        isahc::get(stubr.uri()).expect_status_ok();
    }

    #[should_panic]
    #[stubr::mock]
    #[test]
    fn should_fail() {
        isahc::get(stubr.uri()).expect_status_server_error();
    }

    #[async_std::test]
    #[stubr::mock]
    async fn async_should_succeed() {
        isahc::get_async(stubr.uri()).await.expect_status_ok();
    }

    #[should_panic]
    #[stubr::mock]
    #[async_std::test]
    async fn async_should_fail() {
        isahc::get_async(stubr.uri()).await.expect_status_server_error();
    }
}

mod path {
    use super::*;

    #[async_std::test]
    #[stubr::mock("macros/delete.json")]
    async fn should_append_tests_stubs_path() {
        isahc::delete_async(stubr.uri()).await.expect_status_ok();
        isahc::get_async(stubr.uri()).await.expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("macros/get.json", "macros/delete.json")]
    async fn should_append_many_tests_stubs_path() {
        isahc::get_async(stubr.uri()).await.expect_status_ok();
        isahc::delete_async(stubr.uri()).await.expect_status_ok();
        isahc::post_async(stubr.uri(), ()).await.expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock(full_path = "tests/stubs/macros/delete.json")]
    async fn should_accept_full_path() {
        isahc::delete_async(stubr.uri()).await.expect_status_ok();
        isahc::get_async(stubr.uri()).await.expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("macros/get.json", full_path = "tests/stubs/macros/delete.json")]
    async fn full_path_should_have_precedence_over_path() {
        isahc::delete_async(stubr.uri()).await.expect_status_ok();
        isahc::get_async(stubr.uri()).await.expect_status_not_found();
    }
}

mod port {
    use super::*;

    #[async_std::test]
    #[stubr::mock(port = 1234)]
    async fn should_start_on_port() {
        isahc::get_async("http://localhost:1234").await.expect_status_ok();
    }

    #[should_panic]
    #[stubr::mock(port = 1235)]
    #[async_std::test]
    async fn port_should_fail_when_using_wrong_port() {
        isahc::get_async("http://localhost:1236").await.expect_status_ok();
    }

    #[async_std::test]
    #[stubr::mock(port = 4321)]
    async fn port_should_work_with_provided_stubr_instance() {
        isahc::get_async(stubr.uri()).await.expect_status_ok();
    }
}
