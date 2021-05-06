use async_std::task::block_on;
use http_types::{Request as HttpRequest, Response as HttpResponse};
#[cfg(test)]
use http_types::StatusCode;
use itertools::Itertools;
use warp::{http::Response as WarpResponse, hyper::Body as WarpBody, Reply};

pub struct RecordedRequest(pub HttpRequest);

pub struct RecordedResponse(pub HttpResponse);

pub struct RecordedExchange(pub RecordedRequest, pub RecordedResponse);

impl RecordedExchange {
    const DEFAULT_HOST: &'static str = "localhost";

    pub fn req(&self) -> &HttpRequest { &self.0.0 }
    pub fn resp(&self) -> &HttpResponse { &self.1.0 }
    pub fn host(&self) -> &str { self.req().host().unwrap_or(Self::DEFAULT_HOST) }
}

impl Reply for RecordedResponse {
    fn into_response(mut self) -> WarpResponse<WarpBody> {
        let mut builder = WarpResponse::builder().status(self.0.status());
        let headers = self.0.header_names().into_iter()
            .filter_map(|k| self.0.header(k).map(|v| (k, v)))
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect_vec();
        for (k, v) in headers {
            builder = builder.header(k, v);
        }
        let bytes = block_on(async move { self.0.body_bytes().await.unwrap() });
        builder.body(WarpBody::from(bytes)).unwrap()
    }
}

#[cfg(test)]
impl Default for RecordedExchange {
    fn default() -> Self {
        let req = HttpRequest::get("http://localhost");
        let resp = HttpResponse::new(StatusCode::Ok);
        Self(RecordedRequest(req), RecordedResponse(resp))
    }
}

#[cfg(test)]
mod http_tests {
    use itertools::Itertools;
    use warp::hyper::body::HttpBody;

    use super::*;

    #[test]
    fn reply_should_map_status() {
        assert_eq!(RecordedResponse(HttpResponse::new(200)).into_response().status().as_u16(), 200);
        assert_eq!(RecordedResponse(HttpResponse::new(400)).into_response().status().as_u16(), 400);
        assert_eq!(RecordedResponse(HttpResponse::new(500)).into_response().status().as_u16(), 500);
    }

    #[async_std::test]
    async fn reply_should_map_body() {
        let body = String::from("Hello");
        let mut input = HttpResponse::new(200);
        input.set_body(body.clone());
        let input_response = RecordedResponse(input).into_response();
        let mut input_body = input_response.into_body();
        assert_eq!(input_body.data().await.unwrap().unwrap(), WarpBody::from(body).data().await.unwrap().unwrap());
    }

    #[async_std::test]
    async fn reply_should_map_headers() {
        let mut input = HttpResponse::new(200);
        input.append_header("x-a", "a");
        input.append_header("x-b", "b");
        let input_response = RecordedResponse(input).into_response();
        let input_headers = input_response.headers().iter()
            .map(|(k, v)| (k.as_str(), v.to_str().unwrap()))
            .collect_vec();
        assert!(input_headers.contains(&("x-a", "a")));
        assert!(input_headers.contains(&("x-b", "b")));
    }

    #[async_std::test]
    async fn reply_should_map_multi_headers() {
        let mut input = HttpResponse::new(200);
        input.append_header("x-m", "a,b");
        let input_response = RecordedResponse(input).into_response();
        let input_headers = input_response.headers().iter()
            .map(|(k, v)| (k.as_str(), v.to_str().unwrap()))
            .collect_vec();
        assert!(input_headers.contains(&("x-m", "a,b")));
    }
}