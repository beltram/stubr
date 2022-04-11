use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyBoolean;

impl AnyBoolean {
    pub const NAME: &'static str = "anyBoolean";
    const REASON: &'static str = "be a boolean";
}

impl AnyTemplate for AnyBoolean {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        Ok(rand::random::<bool>().to_string())
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let resp = from_utf8(response.as_slice()).ok();
        let is_bool = resp.and_then(|s| s.parse::<bool>().ok()).is_some();
        assert!(!response.is_empty() && is_bool,
                "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
                ctx.stub_name(), Self::REASON, from_utf8(response.as_slice()).unwrap_or_default()
        );
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        Self::REASON.to_string()
    }
}

impl HelperDef for AnyBoolean {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}