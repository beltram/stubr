use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::{gen::regex::RegexRndGenerator, StubrResult};

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyTime;

impl AnyTime {
    pub const NAME: &'static str = "anyTime";
    pub const TIME_RGX: &'static str = "(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])";
    const REASON: &'static str = "be a valid time (hh:mm:ss)";
}

lazy_static! {
    pub(crate) static ref TIME_REGEX: Regex = Regex::new(&format!("^{}$", AnyTime::TIME_RGX)).expect("Implementation error");
}

impl AnyTemplate for AnyTime {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        RegexRndGenerator(Self::TIME_RGX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        let resp = std::str::from_utf8(&response[..])?;
        assert!(
            TIME_REGEX.is_match(resp),
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

impl HelperDef for AnyTime {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
