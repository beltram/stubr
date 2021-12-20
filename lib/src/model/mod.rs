use std::{convert::TryFrom, fs::OpenOptions, path::PathBuf};

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
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub uuid: Option<String>,
    #[serde(skip_serializing)]
    pub priority: Option<u8>,
    pub request: RequestStub,
    pub response: ResponseStub,
}

impl JsonStub {
    pub const DEFAULT_PRIORITY: u8 = 5;

    pub(crate) fn try_creating_from(self, config: &Config) -> anyhow::Result<Mock> {
        Ok(MockBuilder::try_from(&self.request)?.respond_with(self.into_respond(config)))
    }

    pub fn into_respond<'a>(self, config: &Config) -> impl Respond + 'a {
        let mut template = ResponseTemplate::new(self.response.status());
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

impl TryFrom<&PathBuf> for JsonStub {
    type Error = anyhow::Error;

    fn try_from(maybe_stub: &PathBuf) -> anyhow::Result<Self> {
        let file = OpenOptions::new().read(true).open(&maybe_stub)?;
        serde_json::from_reader(file).map_err(anyhow::Error::msg)
    }
}

impl Default for JsonStub {
    fn default() -> Self {
        Self {
            id: Option::default(),
            uuid: Option::default(),
            priority: Some(Self::DEFAULT_PRIORITY),
            request: RequestStub::default(),
            response: ResponseStub::default(),
        }
    }
}