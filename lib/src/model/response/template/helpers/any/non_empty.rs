use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use crate::gen::regex::RegexRndGenerator;

use super::{AnyTemplate, super::verify::VerifyDetect};

pub struct AnyNonEmpty;

impl AnyNonEmpty {
    pub const NAME: &'static str = "anyNonEmptyString";
    pub const NON_EMPTY_REGEX: &'static str = "[A-Za-z0-9 ]+";
}

impl AnyTemplate for AnyNonEmpty {
    fn generate<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> anyhow::Result<String> {
        RegexRndGenerator(Self::NON_EMPTY_REGEX).try_generate()
    }

    fn verify<'reg: 'rc, 'rc>(&self, _: &Helper<'reg, 'rc>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, response: Vec<u8>) {
        assert!(!response.is_empty(),
                "Verification failed for stub '{}'. Expected response body to match '{}' but was ''",
                ctx.stub_name(), rc.get_root_template_name().map(String::as_str).unwrap_or_default());
    }
}

impl HelperDef for AnyNonEmpty {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, ctx: &'rc Context, rc: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        self.render(h, ctx, rc, out)
    }
}