use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyUuid;

impl AnyUuid {
    pub const NAME: &'static str = "anyUuid";
    pub const UUID_RGX: &'static str = "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}";
    const REASON: &'static str = "be a valid uuid";
}

lazy_static! {
    pub(crate) static ref UUID_REGEX: Regex = Regex::new(&format!("^{}$", AnyUuid::UUID_RGX)).unwrap();
}

impl AnyTemplate for AnyUuid {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::UUID_RGX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        if let Ok(resp) = from_utf8(response.as_slice()) {
            assert!(
                UUID_REGEX.is_match(resp),
                "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
                ctx.stub_name(),
                Self::REASON,
                resp
            )
        }
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        Self::REASON.to_string()
    }
}

impl HelperDef for AnyUuid {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}
