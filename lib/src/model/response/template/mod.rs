use serde::Serialize;
use wiremock::{Request, Respond, ResponseTemplate};

use data::HandlebarsData;

use crate::model::{response::ResponseDto, stub::HANDLEBARS};

pub mod data;
mod req_ext;

pub struct StubTemplate {
    pub(crate) template: ResponseTemplate,
    pub(crate) response: ResponseDto,
    pub(crate) requires_templating: bool,
}

impl Respond for StubTemplate {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        if self.requires_templating {
            let mut template = self.template.clone();
            let data = HandlebarsData::from(request);
            template = self.response.body.into_response_template(template, &data);
            template = self.response.headers.into_response_template(template, &data);
            template
        } else { self.template.clone() }
    }
}

pub trait HandlebarTemplatable {
    fn register_template(&self);
    fn into_response_template(&self, template: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate;

    fn register<S: AsRef<str>>(&self, name: &str, content: S) {
        if let Ok(mut handlebars) = HANDLEBARS.write() {
            handlebars.register_template_string(name, content).unwrap_or_default();
        }
    }

    fn render<T: Serialize>(&self, name: &str, data: &T) -> String {
        HANDLEBARS.read().ok()
            .and_then(|it| it.render(name, data).ok())
            .unwrap_or_default()
    }
}