use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError};

use crate::gen::regex::RegexRndGenerator;

pub struct AnyRegex;

impl AnyRegex {
    pub const NAME: &'static str = "anyRegex";

    fn value<'a>(h: &'a Helper) -> Option<&'a String> {
        h.params().get(0)?.relative_path()
    }
}

impl HelperDef for AnyRegex {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        Self::value(h)
            .and_then(|r| RegexRndGenerator::try_from(r.as_str()).ok())
            .and_then(|g| g.try_generate().ok())
            .and_then(|regex| out.write(&regex).ok())
            .ok_or_else(|| RenderError::new(&format!("Failed rendering '{}' helper", Self::NAME)))
    }
}