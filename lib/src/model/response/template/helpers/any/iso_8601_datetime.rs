use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyIso8601Datetime;

impl AnyIso8601Datetime {
    pub const NAME: &'static str = "anyIso8601";
    pub const ISO_8601_DATETIME_RGX: &'static str = r"([0-9]{4})-(1[0-2]|0[1-9])-(3[01]|0[1-9]|[12][0-9])T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\.\d+)?(Z|[+-][01]\d:[0-5]\d)";
    const REASON: &'static str = "be a valid iso 8601 datetime (yyyy-mm-ddThh:mm:ss)";
}

lazy_static! {
    pub(crate) static ref ISO_8601_DATETIME_REGEX: Regex = Regex::new(&format!("^{}$", AnyIso8601Datetime::ISO_8601_DATETIME_RGX)).unwrap();
}

impl AnyTemplate for AnyIso8601Datetime {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::ISO_8601_DATETIME_RGX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        if let Ok(resp) = from_utf8(response.as_slice()) {
            assert!(ISO_8601_DATETIME_REGEX.is_match(resp),
                    "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
                    ctx.stub_name(), Self::REASON, resp)
        }
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        Self::REASON.to_string()
    }
}

impl HelperDef for AnyIso8601Datetime {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}