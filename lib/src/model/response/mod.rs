use serde::Deserialize;
use wiremock::ResponseTemplate;

use body::BodyDto;
use headers::HttpRespHeadersDto;

use super::StubDto;
use itertools::Itertools;

mod body;
mod body_file;
mod headers;
pub mod default;
pub mod delay;
pub mod template;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDto {
    /// HTTP response status
    pub(crate) status: Option<u16>,
    /// delay in milliseconds to apply to the response
    fixed_delay_milliseconds: Option<u64>,
    /// HTTP response body
    #[serde(flatten)]
    pub(crate) body: BodyDto,
    /// HTTP response headers
    #[serde(flatten)]
    pub(crate) headers: HttpRespHeadersDto,
    /// Mostly used for enabling response templating
    #[serde(default)]
    transformers: Vec<String>,
}

impl ResponseDto {
    const RESPONSE_TEMPLATE: &'static str = "response-template";

    pub(crate) fn requires_response_templating(&self) -> bool {
        self.transformers.iter().any(|it| it == Self::RESPONSE_TEMPLATE)
    }

    pub fn defined_header_keys(&self) -> Vec<&str> {
        self.headers.headers.as_ref()
            .map(|headers| headers.keys().map(|it| it.as_str()).collect_vec())
            .unwrap_or_default()
    }
}

pub(crate) trait ResponseAppender {
    fn add(&self, resp: ResponseTemplate) -> ResponseTemplate;
}

#[cfg(test)]
mod response_dto_tests {
    use std::ops::Not;

    use super::*;

    #[test]
    fn requires_response_templating_should_be_true_when_present() {
        let resp = ResponseDto { transformers: vec![String::from(ResponseDto::RESPONSE_TEMPLATE)], ..Default::default() };
        assert!(resp.requires_response_templating());
    }

    #[test]
    fn requires_response_templating_should_be_false_when_absent() {
        let resp = ResponseDto { transformers: vec![String::from("other")], ..Default::default() };
        assert!(resp.requires_response_templating().not());
    }

    #[test]
    fn requires_response_templating_should_be_false_when_transformers_empty() {
        let resp = ResponseDto { transformers: vec![], ..Default::default() };
        assert!(resp.requires_response_templating().not());
    }
}