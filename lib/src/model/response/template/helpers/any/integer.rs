use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyInteger;

impl AnyInteger {
    pub const NAME: &'static str = "anyInt";
    const INT_REGEX: &'static str = "[+-]?[0-9]+";
}

impl AnyTemplate for AnyInteger {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::INT_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let is_int = from_utf8(response.as_slice()).ok().and_then(|s| s.parse::<i64>().ok()).is_some();
        assert!(!response.is_empty() && is_int,
                "Verification failed for stub '{}'. Expected response body to match '{}' but was '{}'",
                ctx.stub_name(), self.expected(h, rc),
                from_utf8(response.as_slice()).unwrap_or_default()
        );
    }
}

impl HelperDef for AnyInteger {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}