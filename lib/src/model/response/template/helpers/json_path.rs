use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, PathAndJson, RenderContext};
use serde_json::Value;

use crate::model::response::body::BodyStub;

pub struct JsonPathHelper;

impl JsonPathHelper {
    pub const NAME: &'static str = "jsonPath";
    const SUPPORTED_PATH: &'static str = "request.body";

    fn is_supported_helper(input: &PathAndJson) -> bool {
        input.relative_path().map(|it| it.as_str()) == Some(Self::SUPPORTED_PATH)
    }

    fn get_json_path<'a>(params: &'a [PathAndJson]) -> Option<&'a str> {
        params.get(1)
            .and_then(|it| it.relative_path())
            .map(|it| it.trim_start_matches('\'').trim_end_matches('\''))
    }

    fn extract(request_body: &Value, jsonpath: &str) -> Option<Value> {
        jsonpath_lib::select(request_body, jsonpath).ok()
            .and_then(|values| values.get(0).map(|&it| it.to_owned()))
    }
}

impl HelperDef for JsonPathHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        if let Some(input) = h.params().get(0) {
            if Self::is_supported_helper(input) {
                let rendered = Self::get_json_path(h.params())
                    .and_then(|jsonpath| Self::extract(input.value(), jsonpath));
                if let Some(r_str) = rendered.as_ref().and_then(|it| it.as_str()) {
                    out.write(r_str).unwrap();
                } else if let Some(r_obj) = rendered.as_ref().and_then(|it| it.as_object()) {
                    out.write(&format!("{}{}", serde_json::to_string(r_obj).unwrap(), BodyStub::OBJECT_IDENTIFIER)).unwrap();
                } else if let Some(r_array) = rendered.as_ref().and_then(|it| it.as_array()) {
                    out.write(&format!("{}{}", serde_json::to_string(r_array).unwrap(), BodyStub::ARRAY_IDENTIFIER)).unwrap();
                } else if let Some(r_bool) = rendered.as_ref().and_then(|it| it.as_bool()) {
                    out.write(&format!("{}{}", r_bool, BodyStub::BOOL_IDENTIFIER)).unwrap();
                } else if let Some(r_number) = rendered.as_ref().and_then(|it| it.as_i64()) {
                    out.write(&format!("{}{}", r_number, BodyStub::NUMBER_IDENTIFIER)).unwrap();
                } else if let Some(r_float) = rendered.as_ref().and_then(|it| it.as_f64()) {
                    out.write(&format!("{}{}", r_float, BodyStub::FLOAT_IDENTIFIER)).unwrap();
                } else if let Some(_) = rendered.as_ref().and_then(|it| it.as_null()) {
                    out.write(&BodyStub::FLOAT_IDENTIFIER).unwrap();
                }
            }
        }
        Ok(())
    }
}