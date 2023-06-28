use crate::model::response::template::helpers::HelperExt;
use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

pub struct TrimHelper;

impl TrimHelper {
    pub const NAME: &'static str = "trim";
}

impl HelperDef for TrimHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        h.get_first_str_value()
            .ok_or_else(|| RenderError::new("Invalid trim response template"))
            .map(str::trim)
            .map(Value::from)
            .map(ScopedJson::from)
    }
}
