use crate::wiremock_rs::ResponseTemplate;
use crate::StubrResult;
use serde_json::{Map, Value};

use super::{
    template::{data::HandlebarsData, HandlebarTemplatable},
    ResponseAppender,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct HttpRespHeadersStub {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Map<String, Value>>,
}

impl HttpRespHeadersStub {
    fn _render_response_template(&self, mut resp: ResponseTemplate, data: &HandlebarsData) -> StubrResult<ResponseTemplate> {
        if let Some(headers) = self.headers.as_ref() {
            for (k, v) in headers {
                if let Some(v) = v.as_str() {
                    let rendered = self.render(v, data).unwrap_or_default();
                    resp = resp.insert_header(k.as_str(), rendered.as_str())
                }
            }
        }
        Ok(resp)
    }
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

    #[cfg(not(feature = "grpc"))]
    fn render_response_template(&self, resp: ResponseTemplate, data: &HandlebarsData) -> StubrResult<ResponseTemplate> {
        self._render_response_template(resp, data)
    }

    #[cfg(feature = "grpc")]
    fn render_response_template(
        &self, resp: ResponseTemplate, data: &HandlebarsData, _md: Option<&protobuf::reflect::MessageDescriptor>,
    ) -> StubrResult<ResponseTemplate> {
        self._render_response_template(resp, data)
    }
}
