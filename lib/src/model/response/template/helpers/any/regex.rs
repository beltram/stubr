use std::str::from_utf8;

use anyhow::anyhow;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use regex::Regex;

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::{utils_str::ValueExt, verify::VerifyDetect}};

pub struct AnyRegex;

impl AnyRegex {
    pub const NAME: &'static str = "anyRegex";

    fn read_regex<'a>(h: &'a Helper) -> Option<&'a str> {
        h.params().get(0)?.relative_path().map(|s| s.escape_single_quotes())
    }
}

impl AnyTemplate for AnyRegex {
    fn generate<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        Self::read_regex(h)
            .ok_or_else(|| anyhow!("Missing regex for '{}' helper", h.name()))
            .and_then(|r| RegexRndGenerator(r).try_generate())
    }

    fn verify<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, ctx: &'rc Context, _: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        let regex = Self::read_regex(h).map(Regex::new);
        let resp = from_utf8(response.as_slice()).ok();
        if let Some((Ok(regex), resp)) = regex.zip(resp) {
            assert!(regex.is_match(resp),
                    "Verification failed for stub '{}'. Expected response body to match '{}' but was '{}'",
                    ctx.stub_name(), regex.as_str(), resp)
        }
    }

    fn expected<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &mut RenderContext<'reg, 'rc>) -> String {
        format!("match '{}'", Self::read_regex(h).unwrap_or_default().to_string())
    }
}

impl HelperDef for AnyRegex {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}