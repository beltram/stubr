use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;
use crate::{StubrError, StubrResult};

use super::{
    super::{verify::VerifyDetect, ValueExt},
    AnyTemplate,
};

pub struct AnyRegex;

impl AnyRegex {
    pub const NAME: &'static str = "anyRegex";

    fn read_regex<'a>(h: &'a Helper) -> StubrResult<&'a str> {
        h.params()
            .get(0)
            .and_then(|pj| pj.relative_path())
            .map(|s| s.escape_single_quotes())
            .ok_or(StubrError::InvalidTemplate(
                Self::NAME,
                "no value supplied. Should look like '{{anyRegex '[0-9]{5}'}}'",
            ))
    }
}

impl AnyTemplate for AnyRegex {
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Self::read_regex(h)
            .map(RegexRndGenerator)
            .and_then(RegexRndGenerator::try_generate)
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        let regex = Regex::new(Self::read_regex(h)?)?;
        let resp = std::str::from_utf8(&response[..])?;
        assert!(
            regex.is_match(resp),
            "Verification failed for stub '{}'. Expected response body to match '{}' but was '{resp}'",
            ctx.stub_name(),
            regex.as_str(),
        );
        Ok(())
    }

    fn expected<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(format!("match '{}'", Self::read_regex(h)?))
    }
}

impl HelperDef for AnyRegex {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
