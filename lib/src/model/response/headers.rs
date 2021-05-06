use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wiremock::ResponseTemplate;

use super::{ResponseAppender, template::{data::HandlebarsData, HandlebarTemplatable}};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct HttpRespHeadersStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Map<String, Value>>,
}

impl ResponseAppender for HttpRespHeadersStub {
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

impl HandlebarTemplatable for HttpRespHeadersStub {
    fn register_template(&self) {
        if let Some(headers) = self.headers.as_ref() {
            for (_, v) in headers {
                if let Some(v) = v.as_str() {
                    self.register(v, v)
                }
            }
        }
    }

    fn render_response_template(&self, mut resp: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate {
        if let Some(headers) = self.headers.as_ref() {
            for (k, v) in headers {
                if let Some(v) = v.as_str() {
                    let rendered = self.render(v, data);
                    resp = resp.insert_header(k.as_str(), rendered.as_str())
                }
            }
        }
        resp
    }
}