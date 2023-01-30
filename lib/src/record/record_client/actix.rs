use std::{io, str::FromStr};

use actix_http::{body::MessageBody, Payload};
use actix_service::{Service, Transform};
use actix_web::{
    dev::{forward_ready, ServiceRequest as ActixRequest, ServiceRequest, ServiceResponse as ActixServiceResponse},
    error::Error as ActixError,
    web::Bytes,
};
use futures::executor::block_on;
use futures_util::{future::ok, future::LocalBoxFuture, future::Ready, AsyncReadExt, TryStreamExt};
use http::uri::Scheme;
use http_types::{
    headers::{HeaderName as HttpHeaderName, HeaderValue as HttpHeaderValue, HeaderValues as HttpHeaderValues},
    Method as HttpMethod, Request as HttpRequest, Response as HttpResponse, Url,
};

use crate::model::JsonStub;

use super::super::{config::RecordConfig, writer::StubWriter, RecordedExchange, RecordedRequest, RecordedResponse};

#[derive(Default)]
pub struct ActixRecord(pub RecordConfig);

impl<S> Transform<S, ActixRequest> for ActixRecord
where
    S: Service<ActixRequest, Response = ActixServiceResponse, Error = ActixError>,
    S::Future: 'static,
{
    type Response = ActixServiceResponse;
    type Error = ActixError;
    type Transform = ActixRecordMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ActixRecordMiddleware {
            service,
            cfg: self.0.to_owned(),
        })
    }
}

pub struct ActixRecordMiddleware<S> {
    service: S,
    cfg: RecordConfig,
}

impl<S> Service<ActixRequest> for ActixRecordMiddleware<S>
where
    S: Service<ActixRequest, Response = ActixServiceResponse, Error = ActixError>,
    S::Future: 'static,
{
    type Response = ActixServiceResponse;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ActixRequest) -> Self::Future {
        let cfg = self.cfg.clone();
        let (http_req, payload) = req.into_parts();
        let RecordedRequestPair(rec_req, payload) = RecordedRequestPair::from((&http_req, payload));
        let fut = self.service.call(ServiceRequest::from_parts(http_req, payload));
        Box::pin(async move {
            let resp: ActixServiceResponse = fut.await?;
            let host = rec_req.0.url().host_str().unwrap().to_string();
            let RecordedResponsePair(resp, rec_resp) = RecordedResponsePair::from(resp);
            let mut exchange = RecordedExchange(rec_req, rec_resp);
            let stub = JsonStub::from((&mut exchange, &cfg));
            let writer = StubWriter { stub };
            writer.write(&host, cfg.output.as_ref()).unwrap();
            Ok(resp)
        })
    }
}

struct RecordedRequestPair(RecordedRequest, Payload);

impl From<(&actix_web::HttpRequest, Payload)> for RecordedRequestPair {
    fn from((req, rec_payload): (&actix_web::HttpRequest, Payload)) -> Self {
        let method = HttpMethod::from_str(req.method().as_str()).unwrap_or(HttpMethod::Get);
        let path = req.uri().path();
        let scheme = req.uri().scheme().unwrap_or(&Scheme::HTTP);
        let host = req.uri().host().unwrap_or("localhost");
        let queries = req.uri().query().unwrap_or_default();
        let mut url = Url::from_str(&format!("{scheme}://{host}{path}?{queries}")).unwrap();
        url.set_port(req.uri().port_u16()).unwrap();
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
        if let Payload::H1 { payload } = rec_payload {
            let mut buf = Vec::new();
            let mut reader = payload.map_err(|_| io::Error::from(io::ErrorKind::NotFound)).into_async_read();
            block_on(async { reader.read_to_end(&mut buf).await.unwrap() });
            let buf = Box::leak(Box::new(buf)).as_slice();
            http_req.set_body(buf);
            let (mut payload_sender, payload) = actix_http::h1::Payload::create(true);
            payload_sender.feed_data(Bytes::from(buf));
            Self(RecordedRequest(http_req), Payload::from(payload))
        } else {
            Self(RecordedRequest(http_req), Payload::None)
        }
    }
}

struct RecordedResponsePair(ActixServiceResponse, RecordedResponse);

impl From<ActixServiceResponse> for RecordedResponsePair {
    fn from(mut resp: ActixServiceResponse) -> Self {
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
        let mut resp_cpy = actix_web::HttpResponse::build(resp.response().status());
        resp.response().headers().iter().for_each(|h| {
            resp_cpy.insert_header(h);
        });
        let req = resp.request().to_owned();
        let bytes = resp.into_body().try_into_bytes().unwrap_or_default();
        resp = ActixServiceResponse::new(req, resp_cpy.body(bytes.clone()));
        http_resp.set_body(bytes.as_ref());
        RecordedResponsePair(resp, RecordedResponse(http_resp))
    }
}

#[cfg(test)]
mod http_tests {
    use actix_web::{test::TestRequest, HttpResponse as ActixResponse};
    use async_std::task::block_on;
    use http::status::StatusCode as ActixStatus;
    use itertools::Itertools;

    use super::*;

    mod method {
        use http_types::Method;

        use super::*;

        #[test]
        fn should_map_method_get() {
            assert_eq!(
                RecordedRequestPair::from((&TestRequest::get().to_http_request(), Payload::None))
                    .0
                     .0
                    .method(),
                Method::Get
            )
        }

        #[test]
        fn should_map_method_post() {
            assert_eq!(
                RecordedRequestPair::from((&TestRequest::post().to_http_request(), Payload::None))
                    .0
                     .0
                    .method(),
                Method::Post
            )
        }

        #[test]
        fn should_map_method_put() {
            assert_eq!(
                RecordedRequestPair::from((&TestRequest::put().to_http_request(), Payload::None))
                    .0
                     .0
                    .method(),
                Method::Put
            )
        }

        #[test]
        fn should_map_method_patch() {
            assert_eq!(
                RecordedRequestPair::from((&TestRequest::patch().to_http_request(), Payload::None))
                    .0
                     .0
                    .method(),
                Method::Patch
            )
        }

        #[test]
        fn should_map_method_delete() {
            assert_eq!(
                RecordedRequestPair::from((&TestRequest::delete().to_http_request(), Payload::None))
                    .0
                     .0
                    .method(),
                Method::Delete
            )
        }
    }

    mod uri {
        use super::*;

        #[test]
        fn should_map_scheme() {
            let input = TestRequest::get().uri("https://github.com:8080").to_http_request();
            assert_eq!(RecordedRequestPair::from((&input, Payload::None)).0 .0.url().scheme(), "https")
        }

        #[test]
        fn should_map_host() {
            let input = TestRequest::get().uri("https://github.com:8080").to_http_request();
            assert_eq!(
                RecordedRequestPair::from((&input, Payload::None)).0 .0.url().host_str(),
                Some("github.com")
            )
        }

        #[test]
        fn should_map_port() {
            let input = TestRequest::get().uri("https://github.com:8080").to_http_request();
            assert_eq!(RecordedRequestPair::from((&input, Payload::None)).0 .0.url().port(), Some(8080))
        }

        #[test]
        fn should_not_fail_when_port_missing() {
            let input = TestRequest::get().uri("https://github.com").to_http_request();
            assert!(RecordedRequestPair::from((&input, Payload::None)).0 .0.url().port().is_none())
        }

        #[test]
        fn should_map_path() {
            let input = TestRequest::get().uri("https://github.com:8080/api/colors").to_http_request();
            assert_eq!(
                RecordedRequestPair::from((&input, Payload::None)).0 .0.url().path(),
                "/api/colors"
            )
        }

        #[test]
        fn should_not_fail_when_path_missing() {
            let input = TestRequest::get().uri("https://github.com:8080").to_http_request();
            assert_eq!(RecordedRequestPair::from((&input, Payload::None)).0 .0.url().path(), "/")
        }
    }

    mod query {
        use std::borrow::Cow;

        use super::*;

        #[test]
        fn should_map_one_query_param() {
            let input = TestRequest::get().uri("https://github.com:8080?a=1").to_http_request();
            let output = RecordedRequestPair::from((&input, Payload::None));
            let mut queries = output.0 .0.url().query_pairs();
            assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("1"))))
        }

        #[test]
        fn should_not_fail_when_no_query_param() {
            let input = TestRequest::get().uri("https://github.com:8080").to_http_request();
            assert!(RecordedRequestPair::from((&input, Payload::None))
                .0
                 .0
                .url()
                .query_pairs()
                .next()
                .is_none())
        }

        #[test]
        fn should_map_many_query_param() {
            let input = TestRequest::get().uri("https://github.com:8080?a=1&b=2").to_http_request();
            let output = RecordedRequestPair::from((&input, Payload::None));
            let mut queries = output.0 .0.url().query_pairs();
            assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("1"))));
            assert_eq!(queries.next(), Some((Cow::Borrowed("b"), Cow::Borrowed("2"))))
        }
    }

    mod req_headers {
        use super::*;

        #[test]
        fn should_map_one_req_header() {
            let input = TestRequest::get().insert_header(("x-a", "a")).to_http_request();
            let output = RecordedRequestPair::from((&input, Payload::None));
            let ha = output.0 .0.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
        }

        #[test]
        fn should_not_fail_when_no_req_header() {
            let input = TestRequest::get().to_http_request();
            let mut output = RecordedRequestPair::from((&input, Payload::None)).0 .0;
            output.remove_header("content-type");
            assert!(output.header_names().collect_vec().is_empty());
            assert!(output.header_values().collect_vec().is_empty());
        }

        #[test]
        fn should_map_many_req_header() {
            let input = TestRequest::get()
                .insert_header(("x-a", "a"))
                .insert_header(("x-b", "b"))
                .to_http_request();
            let output = RecordedRequestPair::from((&input, Payload::None)).0 .0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
            let hb = output.header("x-b").unwrap().get(0);
            assert_eq!(hb.unwrap().as_str(), "b");
        }

        #[test]
        fn should_map_multi_req_header() {
            let input = TestRequest::get().insert_header(("x-m", "a, b")).to_http_request();
            let output = RecordedRequestPair::from((&input, Payload::None)).0 .0;
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
            let (req, payload) = TestRequest::post().set_json(&input_body).to_http_parts();
            let mut output = RecordedRequestPair::from((&req, payload)).0 .0;
            let body = block_on(async move { output.body_json::<Value>().await.unwrap() });
            assert_eq!(body, input_body);
        }

        #[test]
        fn should_map_text_req_body() {
            let (req, payload) = TestRequest::post().set_payload("Hello World!").to_http_parts();
            let mut output = RecordedRequestPair::from((&req, payload)).0 .0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert_eq!(&body, b"Hello World!");
        }

        #[test]
        fn should_not_fail_when_req_body_empty() {
            let (req, payload) = TestRequest::post().set_payload(String::new()).to_http_parts();
            let mut output = RecordedRequestPair::from((&req, payload)).0 .0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert!(body.is_empty());
        }
    }

    mod status {
        use http_types::StatusCode;

        use super::*;

        #[test]
        fn should_map_continue_100() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::CONTINUE));
            assert_eq!(RecordedResponsePair::from(input).1 .0.status(), StatusCode::Continue)
        }

        #[test]
        fn should_map_ok_200() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::OK));
            assert_eq!(RecordedResponsePair::from(input).1 .0.status(), StatusCode::Ok)
        }

        #[test]
        fn should_map_moved_permanently_301() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::MOVED_PERMANENTLY));
            assert_eq!(RecordedResponsePair::from(input).1 .0.status(), StatusCode::MovedPermanently)
        }

        #[test]
        fn should_map_bad_request_400() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::BAD_REQUEST));
            assert_eq!(RecordedResponsePair::from(input).1 .0.status(), StatusCode::BadRequest)
        }

        #[test]
        fn should_map_server_error_500() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::INTERNAL_SERVER_ERROR));
            assert_eq!(RecordedResponsePair::from(input).1 .0.status(), StatusCode::InternalServerError)
        }
    }

    mod resp_headers {
        use super::*;

        #[test]
        fn should_map_one_resp_header() {
            let input = block_on(async move { ActixResponse::Ok().insert_header(("x-a", "a")).await }).unwrap();
            let input = ActixServiceResponse::new(req(), input);
            let output = RecordedResponsePair::from(input).1 .0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
        }

        #[test]
        fn should_not_fail_when_no_resp_header() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::OK));
            let output = RecordedResponsePair::from(input).1 .0;
            assert_eq!(output.header_names().collect_vec().len(), 1);
        }

        #[test]
        fn should_map_many_resp_header() {
            let input = block_on(async move {
                ActixResponse::Ok()
                    .insert_header(("x-a", "a"))
                    .insert_header(("x-b", "b"))
                    .await
            })
            .unwrap();
            let input = ActixServiceResponse::new(req(), input);
            let output = RecordedResponsePair::from(input).1 .0;
            let ha = output.header("x-a").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "a");
            let ha = output.header("x-b").unwrap().get(0);
            assert_eq!(ha.unwrap().as_str(), "b");
        }

        #[test]
        fn should_map_multi_resp_header() {
            let input = block_on(async move { ActixResponse::Ok().insert_header(("x-m", "a, b")).await }).unwrap();
            let input = ActixServiceResponse::new(req(), input);
            let output = RecordedResponsePair::from(input).1 .0;
            let multi = output.header("x-m").unwrap();
            assert_eq!(multi.get(0).unwrap().as_str(), "a");
            assert_eq!(multi.get(1).unwrap().as_str(), "b");
        }
    }

    mod resp_body {
        use serde_json::{json, Value};

        use super::*;

        #[test]
        fn should_map_json_resp_body() {
            let input_body = json!({"a": "b"});
            let input = ActixServiceResponse::new(req(), ActixResponse::Ok().body(input_body.to_string()));
            let mut output = RecordedResponsePair::from(input).1 .0;
            let body = block_on(async move { output.body_json::<Value>().await.unwrap() });
            assert_eq!(body, input_body);
        }

        #[test]
        fn should_map_binary_resp_body() {
            let input_body = "Hello World!";
            let input = ActixServiceResponse::new(req(), ActixResponse::Ok().body(input_body));
            let mut output = RecordedResponsePair::from(input).1 .0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert_eq!(body.as_slice(), input_body.as_bytes());
        }

        #[test]
        fn should_not_fail_when_body_empty() {
            let input = ActixServiceResponse::new(req(), ActixResponse::new(ActixStatus::OK));
            let mut output = RecordedResponsePair::from(input).1 .0;
            let body = block_on(async move { output.body_bytes().await.unwrap() });
            assert!(body.is_empty());
        }

        #[test]
        fn should_not_alter_remote_content_type() {
            let input = ActixResponse::Ok().insert_header(("content-type", "application/xml")).body("a");
            let input = ActixServiceResponse::new(req(), input);
            let output = RecordedResponsePair::from(input).1 .0;
            let content_type = output.header("content-type").unwrap().get(0);
            assert_eq!(content_type.unwrap().as_str(), "application/xml");
        }
    }

    fn req() -> actix_web::HttpRequest {
        TestRequest::get().to_http_request()
    }
}
