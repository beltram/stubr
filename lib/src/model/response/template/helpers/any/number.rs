use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::gen::regex::RegexRndGenerator;

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyNumber;

impl AnyNumber {
    pub const NAME: &'static str = "anyNumber";
    const NUMBER_REGEX: &'static str = "[+-]?([0-9]*[.])?[0-9]+";
    const REASON: &'static str = "be a number";
}

impl AnyTemplate for AnyNumber {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::NUMBER_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let resp = from_utf8(response.as_slice()).ok();
        let is_float = resp.and_then(|s| s.parse::<f64>().ok()).is_some();
        let is_int = resp.and_then(|s| s.parse::<i64>().ok()).is_some();
        assert!(
            !response.is_empty() && (is_float || is_int),
            "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
            ctx.stub_name(),
            Self::REASON,
            from_utf8(response.as_slice()).unwrap_or_default()
        );
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        Self::REASON.to_string()
    }
}

impl HelperDef for AnyNumber {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}
