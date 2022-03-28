use anyhow::anyhow;
use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

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
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        match h.name() {
            Self::ANY_REGEX => {
                Self::read_regex(h)
                    .ok_or_else(|| anyhow!("Missing regex for '{}' helper", h.name()))
                    .and_then(|r| RegexRndGenerator(r.as_str()).try_generate())
            }
            Self::ANY_NON_BLANK => RegexRndGenerator(Self::NON_BLANK_REGEX).try_generate(),
            Self::ANY_NON_EMPTY => RegexRndGenerator(Self::NON_EMPTY_REGEX).try_generate(),
            _ => Err(anyhow!("Unexpected error"))
        }
            .map_err(|e| RenderError::new(e.to_string()))
            .map(Value::from)
            .map(ScopedJson::from)
    }
}