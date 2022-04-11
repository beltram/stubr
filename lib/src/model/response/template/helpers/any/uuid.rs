use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyUuid;

lazy_static! {
    pub(crate) static ref UUID_REGEX: Regex = Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap();
}

impl AnyUuid {
    pub const NAME: &'static str = "anyUuid";
}

impl AnyTemplate for AnyUuid {
    fn expected<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        UUID_REGEX.as_str().to_string()
    }

    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(UUID_REGEX.as_str()).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        if let Some(resp) = from_utf8(response.as_slice()).ok() {
            assert!(UUID_REGEX.is_match(resp),
                    "Verification failed for stub '{}'. Expected response body to match '{}' but was '{}'",
                    ctx.stub_name(), UUID_REGEX.as_str(), resp)
        }
    }
}

impl HelperDef for AnyUuid {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}