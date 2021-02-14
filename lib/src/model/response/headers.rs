use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::ResponseTemplate;

use super::ResponseAppender;

#[derive(Deserialize, Debug, Default)]
pub struct HttpRespHeadersDto {
    headers: Option<Map<String, Value>>,
}

impl ResponseAppender for HttpRespHeadersDto {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(headers) = self.headers.as_ref() {
            for (k, v) in headers {
                if let Some(v) = v.as_str() {
                    resp = resp.insert_header(k.as_str(), v)
                }
            }
        }
        resp
    }
}