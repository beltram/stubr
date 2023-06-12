use std::hash::{Hash, Hasher};

use crate::wiremock::ResponseTemplate;

use crate::model::response::delay::RandomDelay;
use body::BodyStub;
use headers::HttpRespHeadersStub;

pub mod body;
mod body_file;
pub mod default;
pub mod delay;
pub mod headers;
pub mod template;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStub {
    /// HTTP response status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// delay in milliseconds to apply to the response
    #[serde(skip_serializing)]
    pub fixed_delay_milliseconds: Option<u64>,
    /// random delay accepting different distributions
    #[serde(skip_serializing)]
    pub delay_distribution: Option<RandomDelay>,
    /// HTTP response body
    #[serde(flatten)]
    pub body: BodyStub,
    /// HTTP response headers
    #[serde(flatten)]
    pub headers: HttpRespHeadersStub,
    /// Mostly used for enabling response templating
    #[serde(default, skip_serializing)]
    pub transformers: Vec<String>,
}

impl ResponseStub {
    const RESPONSE_TEMPLATE: &'static str = "response-template";

    pub(crate) fn requires_response_templating(&self) -> bool {
        self.transformers.iter().any(|it| it == Self::RESPONSE_TEMPLATE)
    }

    pub fn user_defined_header_keys(&self) -> Option<impl Iterator<Item = &str>> {
        self.headers.headers.as_ref().map(|headers| headers.keys().map(String::as_str))
    }

    pub fn user_defined_headers(&self) -> Option<impl Iterator<Item = (&str, &str)>> {
        self.headers.headers.as_ref().map(|headers| {
            headers
                .iter()
                .filter_map(|(k, v)| v.as_str().map(|v| (k, v)))
                .map(|(k, v)| (k.as_str(), v))
        })
    }

    pub fn status(&self) -> u16 {
        self.status.unwrap_or(200)
    }
}

impl Hash for ResponseStub {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // we do not need response hash for recorded stub file name
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
        let resp = ResponseStub {
            transformers: vec![String::from(ResponseStub::RESPONSE_TEMPLATE)],
            ..Default::default()
        };
        assert!(resp.requires_response_templating());
    }

    #[test]
    fn requires_response_templating_should_be_false_when_absent() {
        let resp = ResponseStub {
            transformers: vec![String::from("other")],
            ..Default::default()
        };
        assert!(resp.requires_response_templating().not());
    }

    #[test]
    fn requires_response_templating_should_be_false_when_transformers_empty() {
        let resp = ResponseStub {
            transformers: vec![],
            ..Default::default()
        };
        assert!(resp.requires_response_templating().not());
    }
}
