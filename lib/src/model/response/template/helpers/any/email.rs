use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::{gen::regex::RegexRndGenerator, StubrResult};

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyEmail;

impl AnyEmail {
    pub const NAME: &'static str = "anyEmail";
    pub const EMAIL_RGX_GEN: &'static str = "[a-z.]+@[a-z.]+\\.[a-z]{2,6}";
    pub const EMAIL_RGX_VERIFY: &'static str = "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,6}";
    const REASON: &'static str = "be a valid email address";
}

lazy_static! {
    pub(crate) static ref EMAIL_REGEX: Regex = Regex::new(&format!("^{}$", AnyEmail::EMAIL_RGX_VERIFY)).expect("Implementation error");
}

impl AnyTemplate for AnyEmail {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        RegexRndGenerator(Self::EMAIL_RGX_GEN).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        let resp = std::str::from_utf8(&response[..])?;
        assert!(
            EMAIL_REGEX.is_match(resp),
            "Verification failed for stub '{}'. Expected response body to {} but was '{resp}'",
            ctx.stub_name(),
            Self::REASON,
        );
        Ok(())
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(Self::REASON.to_string())
    }
}

impl HelperDef for AnyEmail {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
