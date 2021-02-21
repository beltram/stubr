use serde::Deserialize;
use serde_json::{Map, Value};
use wiremock::ResponseTemplate;

use super::ResponseAppender;
use super::template::{data::HandlebarsData, HandlebarTemplatable};

#[derive(Deserialize, Debug, Default, Clone)]
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

impl HandlebarTemplatable for HttpRespHeadersDto {
    fn register_template(&self) {
        if let Some(headers) = self.headers.as_ref() {
            for (_, v) in headers {
                if let Some(v) = v.as_str() {
                    self.register(v, v)
                }
            }
        }
    }

    fn into_response_template(&self, mut template: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate {
        if let Some(headers) = self.headers.as_ref() {
            for (k, v) in headers {
                if let Some(v) = v.as_str() {
                    let rendered = self.render(v, data);
                    template = template.insert_header(k.as_str(), rendered.as_str())
                }
            }
        }
        template
    }
}