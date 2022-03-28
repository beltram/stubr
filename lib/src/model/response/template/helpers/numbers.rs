use std::ops::Not;

use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

use super::traits::ValueExt;

pub struct NumberHelper;

impl NumberHelper {
    pub const IS_ODD: &'static str = "isOdd";
    pub const IS_EVEN: &'static str = "isEven";
    pub const STRIPES: &'static str = "stripes";

    fn value(h: &Helper) -> Option<i64> {
        h.params().get(0)?.value().as_i64()
    }

    fn stripes_value<'a>(h: &'a Helper, is_odd: bool) -> Option<&'a str> {
        let index = if is_odd { 2 } else { 1 };
        h.params().get(index)
            ?.relative_path()
            .map(String::escape_single_quotes)
    }
}

impl HelperDef for NumberHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Self::value(h)
            .map(|n| n % 2 == 1)
            .and_then(|is_odd| {
                match h.name() {
                    Self::STRIPES => Self::stripes_value(h, is_odd).map(str::to_string),
                    Self::IS_ODD => Some(is_odd.to_string()),
                    Self::IS_EVEN => Some(is_odd.not().to_string()),
                    _ => panic!("Unexpected error"),
                }
            })
            .map(Value::from)
            .map(ScopedJson::from)
            .ok_or_else(|| RenderError::new("Invalid stripes defined in template"))
    }
}