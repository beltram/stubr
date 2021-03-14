use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::{Mock, MockBuilder, Respond, ResponseTemplate};

use crate::Config;

use super::{
    request::RequestDto,
    response::{default::WiremockIsoResponse, delay::Delay, ResponseAppender, ResponseDto, template::{HandlebarTemplatable, StubTemplate}},
};

#[derive(Deserialize, Debug)]
pub struct StubDto {
    pub uuid: Option<String>,
    request: RequestDto,
    pub response: ResponseDto,
}

impl StubDto {
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
