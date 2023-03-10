use std::sync::RwLock;

use crate::{
    model::response::template::data::RequestData,
    wiremock::{Request, Respond, ResponseTemplate},
    StubrResult,
};
use handlebars::Handlebars;
use serde::Serialize;

use crate::model::response::ResponseStub;
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

#[derive(Debug, Clone, Default)]
pub struct StubTemplate {
    pub(crate) template: ResponseTemplate,
    #[allow(dead_code)]
    pub(crate) response: Option<crate::model::response::ResponseStub>,
    #[cfg(feature = "grpc")]
    pub(crate) grpc_response: Option<crate::model::grpc::response::GrpcResponseStub>,
    #[cfg(feature = "grpc")]
    pub(crate) md: Option<protobuf::reflect::MessageDescriptor>,
    #[allow(dead_code)]
    pub(crate) requires_templating: bool,
}

impl StubTemplate {
    #[cfg(not(feature = "grpc"))]
    fn http_respond(&self, mut resp: ResponseTemplate, req: &Request, response: &ResponseStub) -> ResponseTemplate {
        resp = crate::cloud::opentracing::OpenTracing(req).add_opentracing_header(resp, response.user_defined_header_keys());
        resp = crate::cloud::hyper::SupersedeHyper::supersede_hyper_header(resp, response.user_defined_headers());
        if self.requires_templating {
            let data = HandlebarsData {
                request: &RequestData::from(req),
                response: None,
                stub_name: None,
                is_verify: false,
            };
            resp = response.body.render_response_template(resp, &data);
            resp = response.headers.render_response_template(resp, &data);
        }
        resp
    }

    #[cfg(feature = "grpc")]
    fn http_respond(&self, mut resp: ResponseTemplate, req: &Request, response: &ResponseStub) -> StubrResult<ResponseTemplate> {
        resp = crate::cloud::opentracing::OpenTracing(req).add_opentracing_header(resp, response.user_defined_header_keys());
        resp = crate::cloud::hyper::SupersedeHyper::supersede_hyper_header(resp, response.user_defined_headers());
        if self.requires_templating {
            let data = HandlebarsData {
                request: &RequestData::from(req),
                response: None,
                stub_name: None,
                is_verify: false,
            };
            resp = response.body.render_response_template(resp, &data, None)?;
            resp = response.headers.render_response_template(resp, &data, None)?;
        }
        Ok(resp)
    }

    #[cfg(feature = "grpc")]
    fn grpc_respond(
        &self, req: &Request, mut resp: ResponseTemplate, response: &crate::model::grpc::response::GrpcResponseStub,
    ) -> StubrResult<ResponseTemplate> {
        if response.requires_response_templating() {
            let request = self
                .md
                .as_ref()
                .map(|md| RequestData::try_from_grpc_request(req, md))
                .transpose()?
                .unwrap_or_default();
            let data = HandlebarsData {
                request: &request,
                response: None,
                stub_name: None,
                is_verify: false,
            };
            resp = response.render_response_template(resp, &data, self.md.as_ref())?;
        }
        Ok(resp)
    }
}

impl Respond for StubTemplate {
    fn respond(&self, req: &Request) -> StubrResult<ResponseTemplate> {
        let resp = self.template.clone();
        #[cfg(not(feature = "grpc"))]
        if let Some(response) = self.response.as_ref() {
            Ok(self.http_respond(resp, req, response)?)
        } else {
            Ok(resp)
        }
        #[cfg(feature = "grpc")]
        if let Some(response) = self.response.as_ref() {
            Ok(self.http_respond(resp, req, response)?)
        } else if let Some(response) = self.grpc_response.as_ref() {
            Ok(self.grpc_respond(req, resp, response)?)
        } else {
            Ok(resp)
        }
    }
}

pub trait HandlebarTemplatable {
    fn register_template(&self);

    #[cfg(not(feature = "grpc"))]
    fn render_response_template(&self, template: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate;

    #[cfg(feature = "grpc")]
    fn render_response_template(
        &self, template: ResponseTemplate, data: &HandlebarsData, md: Option<&protobuf::reflect::MessageDescriptor>,
    ) -> StubrResult<ResponseTemplate>;

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
