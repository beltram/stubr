use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::gen::regex::RegexRndGenerator;
use crate::StubrResult;

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyNonEmpty;

impl AnyNonEmpty {
    pub const NAME: &'static str = "anyNonEmptyString";
    const NON_EMPTY_REGEX: &'static str = "[A-Za-z0-9 ]+";
    const REASON: &'static str = "be a non empty string";
}

impl AnyTemplate for AnyNonEmpty {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        RegexRndGenerator(Self::NON_EMPTY_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        assert!(
            !response.is_empty(),
            "Verification failed for stub '{}'. Expected response body to {} but was ''",
            ctx.stub_name(),
            Self::REASON
        );
        Ok(())
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(Self::REASON.to_string())
    }
}

impl HelperDef for AnyNonEmpty {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
