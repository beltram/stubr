use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyDatetime;

impl AnyDatetime {
    pub const NAME: &'static str = "anyDatetime";
    pub const DATETIME_RGX: &'static str = r"([0-9]{4})-(1[0-2]|0[1-9])-(3[01]|0[1-9]|[12][0-9])T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])";
    const REASON: &'static str = "be a valid datetime (yyyy-mm-ddThh:mm:ss)";
}

lazy_static! {
    pub(crate) static ref DATETIME_REGEX: Regex = Regex::new(&format!("^{}$", AnyDatetime::DATETIME_RGX)).unwrap();
}

impl AnyTemplate for AnyDatetime {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::DATETIME_RGX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        if let Some(resp) = from_utf8(response.as_slice()).ok() {
            assert!(DATETIME_REGEX.is_match(resp),
                    "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
                    ctx.stub_name(), Self::REASON, resp)
        }
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        Self::REASON.to_string()
    }
}

impl HelperDef for AnyDatetime {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}