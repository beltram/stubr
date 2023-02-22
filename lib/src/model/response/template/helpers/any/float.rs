use crate::StubrResult;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyFloat;

impl AnyFloat {
    pub const NAME: &'static str = "anyFloat";
    const REASON: &'static str = "be a float";
}

impl AnyTemplate for AnyFloat {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> StubrResult<String> {
        Ok(rand::random::<f32>().to_string())
    }

    fn verify<'reg: 'rc, 'rc>(
        &self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>,
    ) -> StubrResult<()> {
        let resp = std::str::from_utf8(response.as_slice())?;
        let is_float = resp.parse::<f64>().is_ok();
        let is_int = resp.parse::<i64>().is_ok();
        assert!(
            !response.is_empty() && is_float && !is_int,
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

impl HelperDef for AnyFloat {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        Ok(self.render(h, ctx, rc, out)?)
    }
}
