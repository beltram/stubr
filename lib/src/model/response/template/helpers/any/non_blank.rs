use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::{gen::regex::RegexRndGenerator, StubrResult};

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyNonBlank;

impl AnyNonBlank {
    pub const NAME: &'static str = "anyNonBlankString";
    const NON_BLANK_REGEX: &'static str = "[A-Za-z0-9]+";
    const REASON: &'static str = "be a non blank string";
}

impl AnyTemplate for AnyNonBlank {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        RegexRndGenerator(Self::NON_BLANK_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        assert!(
            !response.is_empty() && !response.iter().all(|&c| c == 32),
            "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
            ctx.stub_name(),
            Self::REASON,
            std::str::from_utf8(&response[..])?
        );
        Ok(())
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(Self::REASON.to_string())
    }
}

impl HelperDef for AnyNonBlank {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
