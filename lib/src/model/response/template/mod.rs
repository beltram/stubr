use std::sync::RwLock;

use handlebars::Handlebars;
use serde::Serialize;
use wiremock::{Request, Respond, ResponseTemplate};

use data::HandlebarsData;
use helpers::{
    base64::Base64Helper,
    datetime::NowHelper,
    json_path::JsonPathHelper,
    numbers::NumberHelper,
    size::SizeHelper,
    string::StringHelper,
    trim::TrimHelper,
    url_encode::UrlEncodingHelper,
};

use crate::{cloud::opentracing::OpenTracing, model::response::ResponseStub};

pub mod data;
mod req_ext;
mod helpers;

lazy_static! {
    pub(crate) static ref HANDLEBARS: RwLock<Handlebars<'static>> = {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper(JsonPathHelper::NAME, Box::new(JsonPathHelper));
        handlebars.register_helper(NowHelper::NAME, Box::new(NowHelper));
        handlebars.register_helper(NumberHelper::IS_EVEN, Box::new(NumberHelper));
        handlebars.register_helper(NumberHelper::IS_ODD, Box::new(NumberHelper));
        handlebars.register_helper(NumberHelper::STRIPES, Box::new(NumberHelper));
        handlebars.register_helper(TrimHelper::NAME, Box::new(TrimHelper));
        handlebars.register_helper(Base64Helper::NAME, Box::new(Base64Helper));
        handlebars.register_helper(UrlEncodingHelper::NAME, Box::new(UrlEncodingHelper));
        handlebars.register_helper(StringHelper::CAPITALIZE, Box::new(StringHelper));
        handlebars.register_helper(StringHelper::DECAPITALIZE, Box::new(StringHelper));
        handlebars.register_helper(StringHelper::UPPER, Box::new(StringHelper));
        handlebars.register_helper(StringHelper::LOWER, Box::new(StringHelper));
        handlebars.register_helper(SizeHelper::NAME, Box::new(SizeHelper));
        RwLock::new(handlebars)
    };
}

pub struct StubTemplate {
    pub(crate) template: ResponseTemplate,
    pub(crate) response: ResponseStub,
    pub(crate) requires_templating: bool,
}

impl Respond for StubTemplate {
    fn respond(&self, req: &Request) -> ResponseTemplate {
        let mut resp = self.template.clone();
        resp = OpenTracing(req, self.response.defined_header_keys()).add_opentracing_header(resp);
        if self.requires_templating {
            let data = HandlebarsData::from(req);
            resp = self.response.body.render_response_template(resp, &data);
            resp = self.response.headers.render_response_template(resp, &data);
        }
        resp
    }
}

pub trait HandlebarTemplatable {
    fn register_template(&self);
    fn render_response_template(&self, template: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate;

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