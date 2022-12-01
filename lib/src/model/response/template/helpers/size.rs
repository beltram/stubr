use handlebars::{Context, Handlebars, Helper, HelperDef, PathAndJson, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

pub struct SizeHelper;

impl SizeHelper {
    pub const NAME: &'static str = "size";

    fn value<'a>(h: &'a Helper) -> Option<String> {
        h.params().get(0).map(Self::infer_len).map(|s| s.to_string())
    }

    fn infer_len(json: &PathAndJson) -> usize {
        match json.value() {
            Value::String(s) => s.len(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.keys().len(),
            _ => 0,
        }
    }
}

impl HelperDef for SizeHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Self::value(h)
            .ok_or_else(|| RenderError::new("Invalid size response template"))
            .map(Value::from)
            .map(ScopedJson::from)
    }
}
