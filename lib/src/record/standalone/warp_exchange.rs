use http_types::{
    headers::HeaderName as HttpHeaderName, headers::HeaderValue as HttpHeaderValue, headers::HeaderValues as HttpHeaderValues,
    Body as HttpBody, Method as HttpMethod, Request as HttpRequest, Response as HttpResponse, Url,
};

use super::super::{RecordedExchange, RecordedRequest, RecordedResponse};

pub struct WarpRequest {
    pub method: warp::http::Method,
    pub addr: String,
    pub path: String,
    pub queries: Option<String>,
    pub headers: warp::http::HeaderMap,
    pub body: warp::hyper::body::Bytes,
}

pub struct WarpResponse(
    pub warp::http::StatusCode,
    pub warp::http::HeaderMap,
    pub warp::hyper::body::Bytes,
);

pub struct WarpExchange(pub WarpRequest, pub WarpResponse);

impl From<WarpExchange> for RecordedExchange {
    fn from(WarpExchange { 0: req, 1: resp }: WarpExchange) -> Self {
        Self(req.into(), resp.into())
    }
}

impl From<WarpRequest> for RecordedRequest {
    fn from(req: WarpRequest) -> Self {
        use std::str::FromStr as _;
        let method = HttpMethod::from_str(req.method.as_str()).unwrap_or(HttpMethod::Get);
        let path = req.path;
        let path = path.strip_prefix('/').unwrap_or(path.as_str());
        let addr = req.addr;
        let addr = addr.strip_suffix('/').unwrap_or(addr.as_str());
        let queries = req.queries.map(|q| format!("?{q}")).unwrap_or_default();
        let url = Url::from_str(&format!("{addr}/{path}{queries}")).unwrap();
        let mut http_req = HttpRequest::new(method, url.as_str());
        req.headers
            .iter()
            .filter_map(|(k, v)| {
                let k = HttpHeaderName::from_str(k.as_str()).ok();
                let v = v
                    .to_str()
                    .ok()
                    .map(|it| {
                        it.split(',')
                            .map(|s| s.trim())
                            .filter_map(|i| HttpHeaderValue::from_str(i).ok())
                    })
                    .map(HttpHeaderValues::from_iter);
                k.zip(v)
            })
            .for_each(|(k, v)| http_req.append_header(k, &v));
        if !req.body.is_empty() {
            http_req.set_body(HttpBody::from(req.body.as_ref()));
        }
        Self(http_req)
    }
}

impl From<WarpResponse> for RecordedResponse {
    fn from(resp: WarpResponse) -> Self {
        let status = resp.0.as_u16();
        let mut http_resp = HttpResponse::new(status);
        http_resp.set_body(resp.2.as_ref());
        resp.1
            .iter()
            .filter_map(|(k, v)| {
                use std::str::FromStr as _;
                let k = HttpHeaderName::from_str(k.as_str()).ok();
                let v = v
                    .to_str()
                    .ok()
                    .map(|it| {
                        it.split(',')
                            .map(|s| s.trim())
                            .filter_map(|i| HttpHeaderValue::from_str(i).ok())
                    })
                    .map(HttpHeaderValues::from_iter);
                k.zip(v)
            })
            .for_each(|(k, v)| {
                http_resp.insert_header(k, &v);
            });
        Self(http_resp)
    }
}

#[cfg(test)]
mod http_tests {
    use super::*;
    use std::str::FromStr as _;
    use warp::hyper::body::Bytes;

    impl Default for WarpRequest {
        fn default() -> Self {
            Self {
                method: warp::http::Method::GET,
                addr: String::from("http://localhost/"),
                path: String::default(),
                queries: None,
                headers: warp::http::HeaderMap::default(),
                body: Bytes::default(),
            }
        }
    }

    mod method {
        use http_types::Method;

        use super::*;

        #[test]
        fn should_map_method_get() {
            let input = WarpRequest {
                method: warp::http::Method::GET,
                ..Default::default()
            };
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Get)
        }

        #[test]
        fn should_map_method_post() {
            let input = WarpRequest {
                method: warp::http::Method::POST,
                ..Default::default()
            };
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Post)
        }

        #[test]
        fn should_map_method_put() {
            let input = WarpRequest {
                method: warp::http::Method::PUT,
                ..Default::default()
            };
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Put)
        }

        #[test]
        fn should_map_method_patch() {
            let input = WarpRequest {
                method: warp::http::Method::PATCH,
                ..Default::default()
            };
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Patch)
        }

        #[test]
        fn should_map_method_delete() {
            let input = WarpRequest {
                method: warp::http::Method::DELETE,
                ..Default::default()
            };
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Delete)
        }
    }

    mod uri {
        use super::*;

        #[test]
        fn should_map_scheme() {
            let input = WarpRequest {
                addr: String::from("https://github.com:8080"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().scheme(), "https")
        }

        #[test]
        fn should_map_host() {
            let input = WarpRequest {
                addr: String::from("http://github.com:8080"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().host_str(), Some("github.com"))
        }

        #[test]
        fn should_map_port() {
            let input = WarpRequest {
                addr: String::from("http://github.com:8080"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().port(), Some(8080))
        }

        #[test]
        fn should_not_fail_when_port_missing() {
            let input = WarpRequest {
                addr: String::from("http://github.com"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert!(output.url().port().is_none())
        }

        #[test]
        fn should_map_path() {
            let input = WarpRequest {
                path: String::from("/api/colors"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().path(), "/api/colors")
        }

        #[test]
        fn should_not_fail_when_path_missing() {
            let input = WarpRequest::default();
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().path(), "/")
        }

        #[test]
        fn should_strip_trailing_host_slash() {
            let input = WarpRequest {
                addr: String::from("http://github.com/"),
                path: String::from("/api/colors"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().as_str(), "http://github.com/api/colors");

            let input = WarpRequest {
                addr: String::from("http://github.com/"),
                path: String::from("api/colors"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().as_str(), "http://github.com/api/colors");
        }

        #[test]
        fn should_strip_leading_path_slash() {
            let input = WarpRequest {
                addr: String::from("http://github.com/"),
                path: String::from("/api/colors"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().as_str(), "http://github.com/api/colors");

            let input = WarpRequest {
                addr: String::from("http://github.com"),
                path: String::from("/api/colors"),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().as_str(), "http://github.com/api/colors");
        }
    }

    mod query {
        use std::borrow::Cow;

        use super::*;

        #[test]
        fn should_map_one_query_param() {
            let input = WarpRequest {
                queries: Some("a=1".to_string()),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            let mut queries = output.url().query_pairs();
            assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("1"))))
        }

        #[test]
        fn should_not_fail_when_no_query_param() {
            let input = WarpRequest {
                queries: None,
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert!(output.url().query_pairs().next().is_none())
        }

        #[test]
        fn should_map_many_query_param() {
            let input = WarpRequest {
                queries: Some("a=1&b=2".to_string()),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            let mut queries = output.url().query_pairs();
            assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("1"))));
            assert_eq!(queries.next(), Some((Cow::Borrowed("b"), Cow::Borrowed("2"))))
        }
    }

    mod req_headers {
        use itertools::Itertools;
        use warp::http::header::{HeaderName as WarpHeaderName, HeaderValue as WarpHeaderValue};

        use super::*;

        #[test]
        fn should_map_one_req_header() {
            let (ka, va) = (
                WarpHeaderName::from_str("x-a").unwrap(),
                WarpHeaderValue::from_str("a").unwrap(),
            );
            let input = WarpRequest {
                headers: warp::http::HeaderMap::from_iter(vec![(ka, va)]),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
        }

        #[test]
        fn should_not_fail_when_no_req_header() {
            let input = WarpRequest {
                headers: warp::http::HeaderMap::new(),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            assert!(output.header_names().collect_vec().is_empty());
            assert!(output.header_values().collect_vec().is_empty());
        }

        #[test]
        fn should_map_many_req_header() {
            let (ka, va) = (
                WarpHeaderName::from_str("x-a").unwrap(),
                WarpHeaderValue::from_str("a").unwrap(),
            );
            let (kb, vb) = (
                WarpHeaderName::from_str("x-b").unwrap(),
                WarpHeaderValue::from_str("b").unwrap(),
            );
            let input = WarpRequest {
                headers: warp::http::HeaderMap::from_iter(vec![(ka, va), (kb, vb)]),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
            let hb = output.header("x-b").unwrap().get(0);
            assert_eq!(hb.unwrap().as_str(), "b");
        }

        #[test]
        fn should_map_multi_req_header() {
            let (ka, va) = (
                WarpHeaderName::from_str("x-m").unwrap(),
                WarpHeaderValue::from_str("a, b").unwrap(),
            );
            let input = WarpRequest {
                headers: warp::http::HeaderMap::from_iter(vec![(ka, va)]),
                ..Default::default()
            };
            let output = RecordedRequest::from(input).0;
            let multi = output.header("x-m").unwrap();
            assert_eq!(multi.get(0).unwrap().as_str(), "a");
            assert_eq!(multi.get(1).unwrap().as_str(), "b");
        }
    }

    mod req_body {
        use async_std::task::block_on;
        use serde_json::{json, Value};

        use super::*;

        #[test]
        fn should_map_json_req_body() {
            let input_body = json!({"a": "b"});
            let input = WarpRequest {
                body: Bytes::from(input_body.to_string()),
                ..Default::default()
            };
            let mut output = RecordedRequest::from(input).0;
            let body = block_on(async move { output.body_json::<Value>().await.unwrap() });
            assert_eq!(body, input_body);
        }

        #[test]
        fn should_map_text_req_body() {
            let input = WarpRequest {
                body: Bytes::from(String::from("Hello World!")),
                ..Default::default()
            };
            let mut output = RecordedRequest::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert_eq!(&body, b"Hello World!");
        }

        #[test]
        fn should_not_fail_when_req_body_empty() {
            let input = WarpRequest {
                body: Bytes::new(),
                ..Default::default()
            };
            let mut output = RecordedRequest::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert!(body.is_empty());
        }
    }

    mod status {
        use http_types::StatusCode;

        use super::*;

        #[async_std::test]
        async fn should_map_continue_100() {
            let resp = warp::http::Response::builder()
                .status(100)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                resp.status(),
                resp.headers().clone(),
                warp::hyper::body::to_bytes(resp.into_body()).await.unwrap(),
            );
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::Continue);
        }

        #[async_std::test]
        async fn should_map_ok_200() {
            let resp = warp::http::Response::builder()
                .status(200)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                resp.status(),
                resp.headers().clone(),
                warp::hyper::body::to_bytes(resp.into_body()).await.unwrap(),
            );
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::Ok);
        }

        #[async_std::test]
        async fn should_map_moved_permanently_301() {
            let resp = warp::http::Response::builder()
                .status(301)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                resp.status(),
                resp.headers().clone(),
                warp::hyper::body::to_bytes(resp.into_body()).await.unwrap(),
            );
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::MovedPermanently);
        }

        #[async_std::test]
        async fn should_map_bad_request_400() {
            let resp = warp::http::Response::builder()
                .status(400)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                resp.status(),
                resp.headers().clone(),
                warp::hyper::body::to_bytes(resp.into_body()).await.unwrap(),
            );
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::BadRequest);
        }

        #[async_std::test]
        async fn should_map_server_error_500() {
            let resp = warp::http::Response::builder()
                .status(500)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                resp.status(),
                resp.headers().clone(),
                warp::hyper::body::to_bytes(resp.into_body()).await.unwrap(),
            );
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::InternalServerError);
        }
    }

    mod resp_headers {
        use itertools::Itertools;

        use super::*;

        #[async_std::test]
        async fn should_map_one_resp_header() {
            let input = warp::http::Response::builder()
                .status(200)
                .header("x-a", "a")
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
        }

        #[async_std::test]
        async fn should_not_fail_when_no_resp_header() {
            let input = warp::http::Response::builder()
                .status(200)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            // has 'content-type' by default
            assert_eq!(output.header_names().collect_vec().len(), 1);
            assert_eq!(output.header_values().collect_vec().len(), 1);
        }

        #[async_std::test]
        async fn should_map_many_resp_header() {
            let input = warp::http::Response::builder()
                .status(200)
                .header("x-a", "a")
                .header("x-b", "b")
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
            let ha = output.header("x-b").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "b");
        }

        #[async_std::test]
        async fn should_map_multi_resp_header() {
            let input = warp::http::Response::builder()
                .status(200)
                .header("x-m", "a, b")
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            let multi = output.header("x-m").unwrap();
            assert_eq!(multi.get(0).unwrap().as_str(), "a");
            assert_eq!(multi.get(1).unwrap().as_str(), "b");
        }
    }

    mod resp_body {
        use async_std::task::block_on;
        use serde_json::{json, Value};

        use super::*;

        #[async_std::test]
        async fn should_map_json_resp_body() {
            let input_body = json!({"a": "b"});
            let input = warp::http::Response::builder()
                .status(200)
                .body(warp::hyper::body::Body::from(input_body.to_string()))
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let mut output = RecordedResponse::from(input).0;
            let body = block_on(async move { output.body_json::<Value>().await.unwrap() });
            assert_eq!(body, input_body);
        }

        #[async_std::test]
        async fn should_map_binary_resp_body() {
            let input_body = "Hello World!";
            let input = warp::http::Response::builder()
                .status(200)
                .body(warp::hyper::body::Body::from(input_body.to_string()))
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let mut output = RecordedResponse::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert_eq!(body.as_slice(), input_body.as_bytes());
        }

        #[async_std::test]
        async fn should_not_fail_when_body_empty() {
            let input = warp::http::Response::builder()
                .status(200)
                .body(warp::hyper::body::Body::empty())
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let mut output = RecordedResponse::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert!(body.is_empty());
        }

        #[async_std::test]
        async fn should_not_alter_remote_content_type() {
            let input = warp::http::Response::builder()
                .status(200)
                .header("content-type", "application/xml")
                .body(warp::hyper::body::Body::from("a".to_string()))
                .unwrap();
            let input = WarpResponse(
                input.status(),
                input.headers().clone(),
                warp::hyper::body::to_bytes(input.into_body()).await.unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            let content_type = output.header("content-type").unwrap().get(0);
            assert_eq!(content_type.unwrap().as_str(), "application/xml");
        }
    }
}
