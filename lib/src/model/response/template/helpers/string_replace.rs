use super::ValueExt;
use crate::model::response::template::helpers::HelperExt;
use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

pub struct StringReplaceHelper;

impl StringReplaceHelper {
    pub const REPLACE: &'static str = "replace";
}

impl HelperDef for StringReplaceHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let value = h.get_first_str_value().ok_or(RenderError::new(
            "Missing value after 'replace' helper e.g. {{replace request.body ...}}",
        ))?;
        let (placeholder, replacer) = h
            .param(1)
            .zip(h.param(2))
            .and_then(|(p, r)| p.relative_path().zip(r.relative_path()))
            .map(|(p, r)| (p.escape_single_quotes(), r.escape_single_quotes()))
            .ok_or(RenderError::new(
                "Missing values after 'replace' helper e.g. {{replace request.body 'apple' 'peach'}}",
            ))?;
        let replaced = value.replace(placeholder, replacer);
        Ok(Value::from(replaced).into())
    }
}
