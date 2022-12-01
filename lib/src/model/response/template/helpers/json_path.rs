use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, PathAndJson, RenderContext, RenderError};
use serde_json::Value;

use crate::model::response::body::BodyStub;

pub struct JsonPathHelper;

impl JsonPathHelper {
    pub const NAME: &'static str = "jsonPath";
    const SUPPORTED_PATH: &'static str = "request.body";

    fn is_supported_helper(input: &PathAndJson) -> bool {
        input.relative_path().map(String::as_str) == Some(Self::SUPPORTED_PATH)
    }

    fn get_json_path<'a>(params: &'a [PathAndJson]) -> Option<&'a str> {
        params
            .get(1)
            .and_then(PathAndJson::relative_path)
            .map(|it| it.trim_start_matches('\'').trim_end_matches('\''))
    }

    fn extract(request_body: &Value, jsonpath: &str) -> Option<Value> {
        jsonpath_lib::select(request_body, jsonpath)
            .ok()
            .and_then(|values| values.get(0).map(|&v| v.to_owned()))
    }
}

impl HelperDef for JsonPathHelper {
    fn call<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output,
    ) -> HelperResult {
        h.params()
            .get(0)
            .filter(|param| Self::is_supported_helper(param))
            .and_then(|param| Self::get_json_path(h.params()).and_then(|p| Self::extract(param.value(), p)))
            .ok_or_else(|| RenderError::new("Invalid jsonpath response template"))
            .map(|rendered| match rendered {
                Value::Null => "null".to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Number(n) => n.to_string(),
                Value::String(s) => s,
                Value::Array(a) => serde_json::to_string(&a).unwrap_or_default() + BodyStub::ARRAY_IDENTIFIER,
                Value::Object(o) => serde_json::to_string(&o).unwrap_or_default() + BodyStub::OBJECT_IDENTIFIER,
            })
            .and_then(|v| out.write(&v).map_err(RenderError::from))
    }
}
