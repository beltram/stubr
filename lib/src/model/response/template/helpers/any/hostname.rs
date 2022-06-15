use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyHostname;

impl AnyHostname {
    pub const NAME: &'static str = "anyHostname";
    pub const HOST_RGX_GEN: &'static str = r"((http[s]?|ftp):/)/?([a-z]+)(:[0-9]{1,5})?";
    pub const HOST_RGX_VERIFY: &'static str = r"((http[s]?|ftp):/)/?([^:/\s]+)(:[0-9]{1,5})?";
    const REASON: &'static str = "be a valid hostname";
}

lazy_static! {
    pub(crate) static ref HOST_REGEX: Regex = Regex::new(&format!("^{}$", AnyHostname::HOST_RGX_VERIFY)).unwrap();
}

impl AnyTemplate for AnyHostname {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::HOST_RGX_GEN).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        if let Ok(resp) = from_utf8(response.as_slice()) {
            assert!(HOST_REGEX.is_match(resp),
                    "Verification failed for stub '{}'. Expected response body to {} but was '{}'",
                    ctx.stub_name(), Self::REASON, resp)
        }
    }

    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        Self::REASON.to_string()
    }
}

impl HelperDef for AnyHostname {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}