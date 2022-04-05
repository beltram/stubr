use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyFloat;

impl AnyFloat {
    pub const NAME: &'static str = "anyFloat";
    const FLOAT_REGEX: &'static str = "[+-]?[0-9]+[.][0-9]+";
}

impl AnyTemplate for AnyFloat {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::FLOAT_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let resp = from_utf8(response.as_slice()).ok();
        let is_float = resp.and_then(|s| s.parse::<f64>().ok()).is_some();
        let is_int = resp.and_then(|s| s.parse::<i64>().ok()).is_some();
        assert!(!response.is_empty() && is_float && !is_int,
                "Verification failed for stub '{}'. Expected response body to match '{}' but was '{}'",
                ctx.stub_name(), self.expected(h, rc),
                from_utf8(response.as_slice()).unwrap_or_default()
        );
    }
}

impl HelperDef for AnyFloat {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}