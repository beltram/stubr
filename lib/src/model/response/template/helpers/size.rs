use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, PathAndJson, RenderContext};
use serde_json::Value;

pub struct SizeHelper;

impl SizeHelper {
    pub const NAME: &'static str = "size";

    fn value<'a>(h: &'a Helper) -> String {
        h.params().get(0)
            .map(Self::infer_len)
            .unwrap_or_default()
            .to_string()
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
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        out.write(Self::value(h).as_str()).unwrap();
        Ok(())
    }
}