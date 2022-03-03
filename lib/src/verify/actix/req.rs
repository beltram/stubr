use std::str::FromStr;

use actix_web::test::TestRequest;
use async_std::task::block_on;
use http::Method as HttpMethod;

use super::super::mapping::req::StdRequest;

impl From<&mut StdRequest> for TestRequest {
    fn from(req: &mut StdRequest) -> Self {
        let method = HttpMethod::from_str(req.0.method().as_ref()).expect("Unknown http method");
        let mut test_req = Self::default();
        let original_headers = req.0.header_names().into_iter()
            .filter_map(|k| req.0.header(k).map(|v| (k, v)));
        for (k, v) in original_headers {
            test_req = test_req.insert_header((k.as_str(), v.as_str()));
        }
        test_req
            .method(method)
            .uri(req.0.url().as_str())
            .set_payload(block_on(async move { req.0.body_bytes().await.unwrap() }))
    }
}

#[cfg(test)]
mod actix_req_mapping_tests {
    use actix_http::Method;
    use http_types::Request;

    use super::*;

    mod method {
        use super::*;

        #[test]
        fn should_map_get() {
            let mut req = StdRequest(Request::get("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::GET)
        }

        #[test]
        fn should_map_post() {
            let mut req = StdRequest(Request::post("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::POST)
        }

        #[test]
        fn should_map_put() {
            let mut req = StdRequest(Request::put("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::PUT)
        }

        #[test]
        fn should_map_delete() {
            let mut req = StdRequest(Request::delete("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::DELETE)
        }

        #[test]
        fn should_map_patch() {
            let mut req = StdRequest(Request::patch("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::PATCH)
        }

        #[test]
        fn should_map_options() {
            let mut req = StdRequest(Request::options("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::OPTIONS)
        }

        #[test]
        fn should_map_head() {
            let mut req = StdRequest(Request::head("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::HEAD)
        }

        #[test]
        fn should_map_trace() {
            let mut req = StdRequest(Request::trace("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::TRACE)
        }

        #[test]
        fn should_map_connect() {
            let mut req = StdRequest(Request::connect("http://localhost/"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().method(), Method::CONNECT)
        }
    }

    mod url {
        use http::uri::Scheme;

        use super::*;

        #[test]
        fn should_map_scheme() {
            let mut req = StdRequest(Request::get("http://github.com:8080/api/url"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().uri().scheme(), Some(&Scheme::HTTP));
        }

        #[test]
        fn should_map_host() {
            let mut req = StdRequest(Request::get("http://github.com:8080/api/url"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().uri().host(), Some("github.com"));
        }

        #[test]
        fn should_map_port() {
            let mut req = StdRequest(Request::get("http://github.com:8080/api/url"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().uri().port_u16(), Some(8080));
        }

        #[test]
        fn should_map_path() {
            let mut req = StdRequest(Request::get("http://github.com:8080/api/url"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().uri().path(), "/api/url");
        }
    }

    mod query {
        use super::*;

        #[test]
        fn should_map_one() {
            let mut req = StdRequest(Request::get("http://localhost?a=b"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().uri().query(), Some("a=b"));
        }

        #[test]
        fn should_map_many() {
            let mut req = StdRequest(Request::get("http://localhost?a=b&c=d"));
            assert_eq!(TestRequest::from(&mut req).to_http_request().uri().query(), Some("a=b&c=d"));
        }

        #[test]
        fn should_not_fail_when_none() {
            let mut req = StdRequest(Request::get("http://localhost"));
            assert!(TestRequest::from(&mut req).to_http_request().uri().query().is_none());
        }
    }

    mod header {
        use actix_http::header::HeaderValue;

        use super::*;

        #[test]
        fn should_map_one() {
            let mut req = Request::get("http://localhost");
            req.append_header("x-a", "b");
            let test_req = TestRequest::from(&mut StdRequest(req)).to_http_request();
            assert_eq!(test_req.headers().get("x-a"), Some(&HeaderValue::from_str("b").unwrap()));
        }

        #[test]
        fn should_map_many() {
            let mut req = Request::get("http://localhost");
            req.append_header("x-a", "b");
            req.append_header("x-c", "d");
            let test_req = TestRequest::from(&mut StdRequest(req)).to_http_request();
            assert_eq!(test_req.headers().get("x-a"), Some(&HeaderValue::from_str("b").unwrap()));
            assert_eq!(test_req.headers().get("x-c"), Some(&HeaderValue::from_str("d").unwrap()));
        }

        #[test]
        fn should_map_multi() {
            let mut req = Request::get("http://localhost");
            req.append_header("x-a", "b, c");
            let test_req = TestRequest::from(&mut StdRequest(req)).to_http_request();
            let mut values = test_req.headers().get_all("x-a");
            assert_eq!(values.next(), Some(&HeaderValue::from_str("b, c").unwrap()));
        }


        #[test]
        fn should_not_fail_when_none() {
            let mut req = StdRequest(Request::get("http://localhost"));
            let test_req = TestRequest::from(&mut req).to_http_request();
            assert!(test_req.headers().is_empty());
        }
    }

    mod body {
        use actix_http::HttpMessage;
        use futures::TryStreamExt;

        use super::*;

        #[async_std::test]
        async fn should_map_body() {
            let input = "ACAB".as_bytes();
            let mut req = Request::post("http://localhost");
            req.set_body(input);
            let mut test_req = TestRequest::from(&mut StdRequest(req)).to_srv_request();
            let mut body = vec![];
            while let Ok(Some(chunk)) = test_req.take_payload().try_next().await {
                body.append(&mut chunk.as_ref().to_vec());
            }
            assert_eq!(body.as_slice(), input);
        }

        #[async_std::test]
        async fn should_not_fail_when_no_body() {
            let req = Request::get("http://localhost");
            let mut test_req = TestRequest::from(&mut StdRequest(req)).to_srv_request();
            let mut body = vec![];
            while let Ok(Some(chunk)) = test_req.take_payload().try_next().await {
                body.append(&mut chunk.as_ref().to_vec());
            }
            assert!(body.as_slice().is_empty());
        }
    }
}