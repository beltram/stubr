use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use wiremock::{Mock, MockBuilder, Respond, ResponseTemplate};

use request::RequestStub;
use response::{default::WiremockIsoResponse, delay::Delay, ResponseAppender, ResponseStub, template::{HandlebarTemplatable, StubTemplate}};

use crate::Config;

pub mod request;
pub mod response;

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct JsonStub {
    #[serde(skip_serializing)]
    pub uuid: Option<String>,
    pub request: RequestStub,
    pub response: ResponseStub,
}

impl JsonStub {
    pub(crate) fn try_creating_from(self, config: &Config) -> anyhow::Result<Mock> {
        Ok(MockBuilder::try_from(&self.request)?.respond_with(self.into_respond(config)))
    }

    pub fn into_respond<'a>(self, config: &Config) -> impl Respond + 'a {
        let mut template = ResponseTemplate::new(self.response.status.unwrap_or(200));
        template = WiremockIsoResponse(&self).add(template);
        template = Delay(&self, config).add(template);
        if self.response.requires_response_templating() {
            self.response.headers.register_template();
            self.response.body.register_template();
            StubTemplate { template, response: self.response, requires_templating: true }
        } else {
            template = self.response.headers.add(template);
            template = self.response.body.add(template);
            StubTemplate { template, response: self.response, requires_templating: false }
        }
    }
}
