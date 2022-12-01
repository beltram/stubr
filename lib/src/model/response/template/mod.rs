use std::sync::RwLock;

use handlebars::Handlebars;
use serde::Serialize;
use wiremock::{Request, Respond, ResponseTemplate};

use data::HandlebarsData;
use helpers::{
    any::{
        alpha_numeric::AnyAlphaNumeric, boolean::AnyBoolean, date::AnyDate, datetime::AnyDatetime, email::AnyEmail, float::AnyFloat,
        hostname::AnyHostname, integer::AnyInteger, ip::AnyIp, iso_8601_datetime::AnyIso8601Datetime, non_blank::AnyNonBlank,
        non_empty::AnyNonEmpty, number::AnyNumber, of::AnyOf, regex::AnyRegex, time::AnyTime, uuid::AnyUuid,
    },
    base64::Base64Helper,
    datetime::NowHelper,
    json_path::JsonPathHelper,
    numbers::NumberHelper,
    size::SizeHelper,
    string::StringHelper,
    trim::TrimHelper,
    url_encode::UrlEncodingHelper,
};

use crate::cloud::hyper::SupersedeHyper;
use crate::{
    cloud::opentracing::OpenTracing,
    model::response::{template::data::RequestData, ResponseStub},
};

pub mod data;
mod helpers;
mod req_ext;
pub mod utils;
pub mod verify;

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
        handlebars.register_helper(AnyRegex::NAME, Box::new(AnyRegex));
        handlebars.register_helper(AnyNonBlank::NAME, Box::new(AnyNonBlank));
        handlebars.register_helper(AnyNonEmpty::NAME, Box::new(AnyNonEmpty));
        handlebars.register_helper(AnyAlphaNumeric::NAME, Box::new(AnyAlphaNumeric));
        handlebars.register_helper(AnyNumber::NAME, Box::new(AnyNumber));
        handlebars.register_helper(AnyFloat::NAME, Box::new(AnyFloat));
        handlebars.register_helper(AnyInteger::I64, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::U64, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::I32, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::U32, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::I16, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::U16, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::I8, Box::new(AnyInteger));
        handlebars.register_helper(AnyInteger::U8, Box::new(AnyInteger));
        handlebars.register_helper(AnyUuid::NAME, Box::new(AnyUuid));
        handlebars.register_helper(AnyBoolean::NAME, Box::new(AnyBoolean));
        handlebars.register_helper(AnyEmail::NAME, Box::new(AnyEmail));
        handlebars.register_helper(AnyIp::NAME, Box::new(AnyIp));
        handlebars.register_helper(AnyHostname::NAME, Box::new(AnyHostname));
        handlebars.register_helper(AnyDate::NAME, Box::new(AnyDate));
        handlebars.register_helper(AnyTime::NAME, Box::new(AnyTime));
        handlebars.register_helper(AnyDatetime::NAME, Box::new(AnyDatetime));
        handlebars.register_helper(AnyIso8601Datetime::NAME, Box::new(AnyIso8601Datetime));
        handlebars.register_helper(AnyOf::NAME, Box::new(AnyOf));
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
        resp = OpenTracing(req).add_opentracing_header(resp, self.response.user_defined_header_keys());
        resp = SupersedeHyper::supersede_hyper_header(resp, self.response.user_defined_headers());
        if self.requires_templating {
            let data = HandlebarsData {
                request: &RequestData::from(req),
                response: None,
                stub_name: None,
                is_verify: false,
            };
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

    /// Template has to be registered first before being rendered here
    /// Better for performances
    fn render<T: Serialize>(&self, name: &str, data: &T) -> String {
        HANDLEBARS
            .read()
            .ok()
            .and_then(|it| it.render(name, data).ok())
            .unwrap_or_default()
    }

    /// Template does not have to be registered first
    /// Simpler
    fn render_template<T: Serialize>(&self, name: &str, data: &T) -> String {
        HANDLEBARS
            .read()
            .ok()
            .and_then(|it| it.render_template(name, data).ok())
            .unwrap_or_default()
    }
}
