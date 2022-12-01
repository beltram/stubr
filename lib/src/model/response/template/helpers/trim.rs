use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

pub struct TrimHelper;

impl TrimHelper {
    pub const NAME: &'static str = "trim";

    fn value<'a>(h: &'a Helper) -> Option<&'a str> {
        h.params().get(0)?.value().as_str()
    }
}

impl HelperDef for TrimHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Self::value(h)
            .ok_or_else(|| RenderError::new("Invalid trim response template"))
            .map(str::trim)
            .map(Value::from)
            .map(ScopedJson::from)
    }
}
