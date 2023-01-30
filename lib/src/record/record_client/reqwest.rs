use std::str::FromStr;

use http_types::{
    headers::HeaderName as HttpHeaderName, headers::HeaderValue as HttpHeaderValue, headers::HeaderValues as HttpHeaderValues,
    Body as HttpBody, Method as HttpMethod, Request as HttpRequest, Response as HttpResponse, Url,
};
use reqwest::blocking::{Request, RequestBuilder as ReqwestRequestBuilder, Response};

use crate::model::JsonStub;

use super::super::{core::Record, writer::StubWriter, RecordConfig, RecordedExchange, RecordedRequest, RecordedResponse};

impl Record for ReqwestRequestBuilder {
    fn record_with(self, cfg: RecordConfig) -> Self {
        let req = RecordedRequest::from(self.try_clone().and_then(|it| it.build().ok()).unwrap());
        let resp = RecordedResponse::from(self.try_clone().and_then(|it| it.send().ok()).unwrap());
        let host = req.0.url().host_str().unwrap().to_string();
        let mut exchange = RecordedExchange(req, resp);

        let stub = JsonStub::from((&mut exchange, &cfg));
        let writer = StubWriter { stub };
        writer.write(&host, cfg.output.as_ref()).unwrap();
        self
    }
}

impl From<Request> for RecordedRequest {
    fn from(req: Request) -> Self {
        let method = HttpMethod::from_str(req.method().as_str()).unwrap_or(HttpMethod::Get);
        let path = req.url().path();
        let scheme = req.url().scheme();
        let host = req.url().host_str().unwrap_or("localhost");
        let queries = req.url().query().unwrap_or_default();
        let mut url = Url::from_str(&format!("{scheme}://{host}{path}?{queries}")).unwrap();
        url.set_port(req.url().port()).unwrap();
        let mut http_req = HttpRequest::new(method, url.as_str());
        req.headers()
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
        if let Some(body) = req.body() {
            let body = body.as_bytes().unwrap_or_default();
            http_req.set_body(HttpBody::from(body))
        }
        Self(http_req)
    }
}

impl From<Response> for RecordedResponse {
    fn from(resp: Response) -> Self {
        let status = resp.status().as_u16();
        let mut http_resp = HttpResponse::new(status);
        resp.headers()
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
            .for_each(|(k, v)| http_resp.append_header(k, &v));
        if let Ok(body) = resp.bytes() {
            http_resp.set_body(HttpBody::from(body.as_ref()));
        }
        Self(http_resp)
    }
}

#[cfg(test)]
mod http_tests {
    use reqwest::blocking::ClientBuilder as ReqwestBlockingClientBuilder;
    use reqwest::Url;

    use super::*;

    mod method {
        use http_types::Method;

        use super::*;

        #[test]
        fn should_map_method_get() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("http://url/test").unwrap())
                .build()
                .unwrap();
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Get)
        }

        #[test]
        fn should_map_method_post() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .post(Url::from_str("http://url/test").unwrap())
                .build()
                .unwrap();
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Post)
        }

        #[test]
        fn should_map_method_put() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .put(Url::from_str("http://url/test").unwrap())
                .build()
                .unwrap();
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Put)
        }

        #[test]
        fn should_map_method_patch() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .patch(Url::from_str("http://url/test").unwrap())
                .build()
                .unwrap();
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Patch)
        }

        #[test]
        fn should_map_method_delete() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .delete(Url::from_str("http://url/test").unwrap())
                .build()
                .unwrap();
            assert_eq!(RecordedRequest::from(input).0.method(), Method::Delete)
        }
    }

    mod uri {
        use super::*;

        #[test]
        fn should_map_scheme() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().scheme(), "https")
        }

        #[test]
        fn should_map_host() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("http://github.com:8080").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().host_str(), Some("github.com"))
        }

        #[test]
        fn should_map_port() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("http://github.com:8080").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().port(), Some(8080))
        }

        #[test]
        fn should_not_fail_when_port_missing() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert!(output.url().port().is_none())
        }

        #[test]
        fn should_map_path() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080/api/colors").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().path(), "/api/colors")
        }

        #[test]
        fn should_not_fail_when_path_missing() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert_eq!(output.url().path(), "/")
        }
    }

    mod query {
        use std::borrow::Cow;

        use super::*;

        #[test]
        fn should_map_one_query_param() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080?a=1").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            let mut queries = output.url().query_pairs();
            assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("1"))))
        }

        #[test]
        fn should_not_fail_when_no_query_param() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert!(output.url().query_pairs().next().is_none())
        }

        #[test]
        fn should_map_many_query_param() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080?a=1&b=2").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            let mut queries = output.url().query_pairs();
            assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("1"))));
            assert_eq!(queries.next(), Some((Cow::Borrowed("b"), Cow::Borrowed("2"))))
        }
    }

    mod req_headers {
        use itertools::Itertools;
        use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

        use super::*;

        #[test]
        fn should_map_one_req_header() {
            let (ka, va) = (HeaderName::from_str("x-a").unwrap(), HeaderValue::from_str("a").unwrap());
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .headers(HeaderMap::from_iter(vec![(ka, va)]))
                .build()
                .unwrap();

            let output = RecordedRequest::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
        }

        #[test]
        fn should_not_fail_when_no_req_header() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .build()
                .unwrap();
            let output = RecordedRequest::from(input).0;
            assert!(output.header_names().collect_vec().is_empty());
            assert!(output.header_values().collect_vec().is_empty());
        }

        #[test]
        fn should_map_many_req_header() {
            let (ka, va) = (HeaderName::from_str("x-a").unwrap(), HeaderValue::from_str("a").unwrap());
            let (kb, vb) = (HeaderName::from_str("x-b").unwrap(), HeaderValue::from_str("b").unwrap());
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .headers(HeaderMap::from_iter(vec![(ka, va), (kb, vb)]))
                .build()
                .unwrap();

            let output = RecordedRequest::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
            let hb = output.header("x-b").unwrap().get(0);
            assert_eq!(hb.unwrap().as_str(), "b");
        }

        #[test]
        fn should_map_multi_req_header() {
            let (ka, va) = (HeaderName::from_str("x-m").unwrap(), HeaderValue::from_str("a, b").unwrap());
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .headers(HeaderMap::from_iter(vec![(ka, va)]))
                .build()
                .unwrap();

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
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .json(&input_body)
                .build()
                .unwrap();
            let mut output = RecordedRequest::from(input).0;
            let body = block_on(async move { output.body_json::<Value>().await.unwrap() });
            assert_eq!(body, input_body);
        }

        #[test]
        fn should_map_text_req_body() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .body("Hello World!")
                .build()
                .unwrap();
            let mut output = RecordedRequest::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert_eq!(&body, b"Hello World!");
        }

        #[test]
        fn should_not_fail_when_req_body_empty() {
            let input = ReqwestBlockingClientBuilder::new()
                .build()
                .unwrap()
                .get(Url::from_str("https://github.com:8080").unwrap())
                .body(String::new())
                .build()
                .unwrap();
            let mut output = RecordedRequest::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert!(body.is_empty());
        }
    }

    mod status {
        use http::response::Builder;
        use http_types::StatusCode;

        use super::*;

        #[test]
        fn should_map_continue_100() {
            let input = Response::from(Builder::new().status(100).body("Hello!").unwrap());
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::Continue)
        }

        #[test]
        fn should_map_ok_200() {
            let input = Response::from(Builder::new().status(200).body("Hello!").unwrap());
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::Ok)
        }

        #[test]
        fn should_map_moved_permanently_301() {
            let input = Response::from(Builder::new().status(301).body("Hello!").unwrap());
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::MovedPermanently)
        }

        #[test]
        fn should_map_bad_request_400() {
            let input = Response::from(Builder::new().status(400).body("Hello!").unwrap());
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::BadRequest)
        }

        #[test]
        fn should_map_server_error_500() {
            let input = Response::from(Builder::new().status(500).body("Hello!").unwrap());
            assert_eq!(RecordedResponse::from(input).0.status(), StatusCode::InternalServerError)
        }
    }

    mod resp_headers {
        use http::response::Builder;
        use itertools::Itertools;

        use super::*;

        #[test]
        fn should_map_one_resp_header() {
            let input = Response::from(Builder::new().status(200).header("x-a", "a").body("Hello!").unwrap());
            let output = RecordedResponse::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
        }

        #[test]
        fn should_not_fail_when_no_resp_header() {
            let input = Response::from(Builder::new().status(200).body("Hello!").unwrap());
            let output = RecordedResponse::from(input).0;
            // has 'content-type' by default
            assert_eq!(output.header_names().collect_vec().len(), 1);
            assert_eq!(output.header_values().collect_vec().len(), 1);
        }

        #[test]
        fn should_map_many_resp_header() {
            let input = Response::from(
                Builder::new()
                    .status(200)
                    .header("x-a", "a")
                    .header("x-b", "b")
                    .body("Hello!")
                    .unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
            let ha = output.header("x-b").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "b");
        }

        #[test]
        fn should_map_multi_resp_header() {
            let input = Response::from(Builder::new().status(200).header("x-m", "a, b").body("Hello!").unwrap());
            let output = RecordedResponse::from(input).0;
            let multi = output.header("x-m").unwrap();
            assert_eq!(multi.get(0).unwrap().as_str(), "a");
            assert_eq!(multi.get(1).unwrap().as_str(), "b");
        }
    }

    mod resp_body {
        use async_std::task::block_on;
        use http::response::Builder;
        use serde_json::{json, Value};

        use super::*;

        #[test]
        fn should_map_json_resp_body() {
            let input_body = json!({"a": "b"});
            let str_body = input_body.to_string();
            let input = Response::from(Builder::new().status(200).body(str_body).unwrap());
            let mut output = RecordedResponse::from(input).0;
            let body = block_on(async move { output.body_json::<Value>().await.unwrap() });
            assert_eq!(body, input_body);
        }

        #[test]
        fn should_map_binary_resp_body() {
            let input_body = "Hello World!";
            let input = Response::from(Builder::new().status(200).body(input_body).unwrap());
            let mut output = RecordedResponse::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert_eq!(body.as_slice(), input_body.as_bytes());
        }

        #[test]
        fn should_not_fail_when_body_empty() {
            let input = Response::from(Builder::new().status(200).body("").unwrap());
            let mut output = RecordedResponse::from(input).0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert!(body.is_empty());
        }

        #[test]
        fn should_not_alter_remote_content_type() {
            let input = Response::from(
                Builder::new()
                    .status(200)
                    .header("content-type", "application/xml")
                    .body("a")
                    .unwrap(),
            );
            let output = RecordedResponse::from(input).0;
            let content_type = output.header("content-type").unwrap().get(0);
            assert_eq!(content_type.unwrap().as_str(), "application/xml");
        }
    }
}
