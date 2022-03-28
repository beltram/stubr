use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError};

use crate::gen::regex::RegexRndGenerator;

pub struct AnyRegex;

impl AnyRegex {
    pub const ANY_REGEX: &'static str = "anyRegex";
    pub const ANY_NON_BLANK: &'static str = "anyNonBlankString";
    pub const ANY_NON_EMPTY: &'static str = "anyNonEmptyString";
    pub const NON_BLANK_REGEX: &'static str = "[A-Za-z0-9]+";
    pub const NON_EMPTY_REGEX: &'static str = "[A-Za-z0-9 ]+";

    fn read_regex<'a>(h: &'a Helper) -> Option<&'a String> {
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
        match h.name() {
            Self::ANY_REGEX => Self::read_regex(h).and_then(|r| RegexRndGenerator(r.as_str()).try_generate().ok()),
            Self::ANY_NON_BLANK => RegexRndGenerator(Self::NON_BLANK_REGEX).try_generate().ok(),
            Self::ANY_NON_EMPTY => RegexRndGenerator(Self::NON_EMPTY_REGEX).try_generate().ok(),
            _ => None
        }
            .and_then(|regex| out.write(&regex).ok())
            .ok_or_else(|| RenderError::new(&format!("Failed rendering '{}' helper", h.name())))
    }
}