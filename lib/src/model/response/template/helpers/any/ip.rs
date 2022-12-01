use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{super::verify::VerifyDetect, AnyTemplate};

pub struct AnyIp;

impl AnyIp {
    pub const NAME: &'static str = "anyIpAddress";
    pub const IP_RGX: &'static str =
        r"([01]?\d\d?|2[0-4]\d|25[0-5])\.([01]?\d\d?|2[0-4]\d|25[0-5])\.([01]?\d\d?|2[0-4]\d|25[0-5])\.([01]?\d\d?|2[0-4]\d|25[0-5])";
    const REASON: &'static str = "be a valid ip address";
}

lazy_static! {
    pub(crate) static ref IP_REGEX: Regex = Regex::new(&format!("^{}$", AnyIp::IP_RGX)).unwrap();
}

impl AnyTemplate for AnyIp {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::IP_RGX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        if let Ok(resp) = from_utf8(response.as_slice()) {
            assert!(
                IP_REGEX.is_match(resp),
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

impl HelperDef for AnyIp {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}
