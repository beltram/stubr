use std::str::FromStr;

use actix_http::body::{Body as ActixBody, ResponseBody as ActixResponseBody};
use actix_web::dev::ServiceResponse as ActixServiceResponse;
use http_types::{
    headers::HeaderName as HttpHeaderName,
    headers::HeaderValue as HttpHeaderValue,
    headers::HeaderValues as HttpHeaderValues,
    Response,
};

use super::super::mapping::resp::StdResponse;

impl From<ActixServiceResponse> for StdResponse {
    fn from(mut resp: ActixServiceResponse) -> Self {
        let status = resp.status();
        let mut std_resp = Response::new(status.as_u16());
        resp.headers().into_iter()
            .filter_map(|(k, v)| {
                let k = HttpHeaderName::from_str(k.as_str()).ok();
                let v = v.to_str().ok()
                    .map(|it| it.split(',').map(|s| s.trim()).filter_map(|i| HttpHeaderValue::from_str(i).ok()))
                    .map(HttpHeaderValues::from_iter);
                k.zip(v)
            })
            .for_each(|(k, v)| std_resp.append_header(k, &v));
        if let ActixResponseBody::Body(ActixBody::Bytes(body)) = resp.take_body() {
            std_resp.set_body(body.as_ref())
        }
        Self(std_resp)
    }
}

#[cfg(test)]
mod actix_resp_mapping_tests {
    use std::str::FromStr;

    use actix_http::Response as ActixResponse;
    use actix_web::{HttpRequest, test::TestRequest};

    use super::*;

    fn req() -> HttpRequest {
        TestRequest::get().to_http_request()
    }

    mod status {
        use super::*;

        #[test]
        fn should_map_status_success() {
            let resp = ActixResponse::Ok().finish();
            let std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.status(), 200);
        }

        #[test]
        fn should_map_status_error() {
            let resp = ActixResponse::BadRequest().finish();
            let std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.status(), 400);
        }
    }

    mod headers {
        use http_types::headers::{HeaderValue, HeaderValues};

        use super::*;

        #[test]
        fn should_map_single_header() {
            let resp = ActixResponse::Ok().header("a", "b").finish();
            let std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.header("a").unwrap().get(0), HeaderValue::from_str("b").ok().as_ref());
        }

        #[test]
        fn should_map_many_header() {
            let resp = ActixResponse::Ok().header("a", "b").header("c", "d").finish();
            let std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.header("a").unwrap().get(0), HeaderValue::from_str("b").ok().as_ref());
            assert_eq!(std_resp.header("c").unwrap().get(0), HeaderValue::from_str("d").ok().as_ref());
        }

        #[test]
        fn should_not_fail_when_no_headers() {
            let resp = ActixResponse::Ok().finish();
            let std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.header_names().into_iter().count(), 0);
        }

        #[test]
        fn should_map_multi_header() {
            let resp = ActixResponse::Ok().header("a", "b, c").finish();
            let std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            let expected = HeaderValues::from_iter(vec!["b".try_into().unwrap(), "c".try_into().unwrap()]);
            assert!(std_resp.header("a").unwrap().iter().eq(expected.iter()));
        }
    }

    mod body {
        use serde_json::{json, Value};

        use super::*;

        #[async_std::test]
        async fn should_map_json_body() {
            let resp = ActixResponse::Ok().body(json!({"a": "b"}));
            let mut std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.body_json::<Value>().await.unwrap(), json!({"a": "b"}));
        }

        #[async_std::test]
        async fn should_map_text_body() {
            let resp = ActixResponse::Ok().body("hello");
            let mut std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert_eq!(std_resp.body_string().await.unwrap(), String::from("hello"));
        }

        #[async_std::test]
        async fn should_map_missing_body() {
            let resp = ActixResponse::Ok().finish();
            let mut std_resp = StdResponse::from(ActixServiceResponse::new(req(), resp)).0;
            assert!(std_resp.body_json::<Value>().await.ok().is_none());
        }
    }
}