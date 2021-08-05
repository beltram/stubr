use reqwest::blocking::{RequestBuilder as ReqwestRequestBuilder, Request, Response};
use std::{
    iter::FromIterator,
    str::FromStr
};

use crate::{
    RecordConfig,
    model::JsonStub,
    record::{
        core::Record,
        writer::StubWriter,
        http::{RecordedRequest, RecordedResponse, RecordedExchange},
    },
};

use http_types::{
    Body as HttpBody,
    headers::HeaderName as HttpHeaderName,
    headers::HeaderValue as HttpHeaderValue,
    headers::HeaderValues as HttpHeaderValues,
    Method as HttpMethod,
    Request as HttpRequest,
    Response as HttpResponse,
    Url,
};


impl Record for ReqwestRequestBuilder {
    fn record_with(&mut self, cfg: RecordConfig) -> &mut Self {
        let req = RecordedRequest::from(self.try_clone().and_then(|it| it.build().ok()).unwrap());
        let resp  = RecordedResponse::from(self.try_clone().and_then(|it| it.send().ok()).unwrap());
        let host = req.0.clone().url().host_str().unwrap().to_string();
        let mut exchange = RecordedExchange(req, resp);

        let stub = JsonStub::from((&mut exchange, cfg.clone()));
        let writer = StubWriter { stub };
        writer.write(&host, cfg.output).unwrap();
        self
    }
}

impl From<Request> for RecordedRequest {
    fn from(req: Request) -> Self {
        let method = HttpMethod::from_str(req.method().as_str()).unwrap_or(HttpMethod::Get);
        let path = req.url().path();
        let scheme = req.url().scheme();
        let host = req.url().host_str().unwrap_or_else(|| "localhost");
        let queries = req.url().query().unwrap_or_default();
        let url = Url::from_str(&format!("{}://{}{}?{}", scheme, host, path, queries)).unwrap();
        let mut http_req = HttpRequest::new(method, url.as_str());
        req.headers().iter()
            .filter_map(|(k, v)| {
                let k = HttpHeaderName::from_str(k.as_str()).ok();
                let v = v.to_str().ok()
                    .map(|it| it.split(',').map(|s| s.trim()).filter_map(|i| HttpHeaderValue::from_str(i).ok()))
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
        resp.headers().iter()
            .filter_map(|(k, v)| {
                let k = HttpHeaderName::from_str(k.as_str()).ok();
                let v = v.to_str().ok()
                    .map(|it| it.split(',').map(|s| s.trim()).filter_map(|i| HttpHeaderValue::from_str(i).ok()))
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


