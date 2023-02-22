use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::{gen::regex::RegexRndGenerator, StubrResult};

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyAlphaNumeric;

impl AnyAlphaNumeric {
    pub const NAME: &'static str = "anyAlphaNumeric";
    const ALPHA_NUMERIC_REGEX: &'static str = "[A-Za-z0-9]+";
    const REASON: &'static str = "be an alphanumeric";
}

impl AnyTemplate for AnyAlphaNumeric {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        RegexRndGenerator(Self::ALPHA_NUMERIC_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        assert!(
            !response.is_empty() && response.iter().all(u8::is_ascii_alphanumeric),
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

impl HelperDef for AnyAlphaNumeric {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
