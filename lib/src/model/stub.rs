use std::{convert::TryFrom, sync::RwLock};

use handlebars::Handlebars;
use serde::Deserialize;
use wiremock::{Mock, MockBuilder, Respond, ResponseTemplate};

use super::{
    request::RequestDto,
    response::{default::WiremockIsoResponse, delay::Delay, ResponseAppender, ResponseDto, template::{HandlebarTemplatable, StubTemplate}},
};

lazy_static! {
    pub(crate) static ref HANDLEBARS: RwLock<Handlebars<'static>> = {
        let mut handlebars = Handlebars::new();
        handlebars.source_map_enabled(false);
        RwLock::new(handlebars)
    };
}

#[derive(Deserialize, Debug)]
pub struct StubDto {
    pub uuid: Option<String>,
    request: RequestDto,
    pub response: ResponseDto,
}

impl StubDto {
    pub fn into_respond<'a>(self) -> impl Respond + 'a {
        let mut template = ResponseTemplate::new(self.response.status);
        template = WiremockIsoResponse(&self).add(template);
        template = Delay(&self).add(template);
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

impl TryFrom<StubDto> for Mock {
    type Error = anyhow::Error;

    fn try_from(stub: StubDto) -> anyhow::Result<Self> {
        Ok(MockBuilder::try_from(&stub.request)?.respond_with(stub.into_respond()))
    }
}
