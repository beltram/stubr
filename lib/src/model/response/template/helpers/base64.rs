use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, PathAndJson, RenderContext, RenderError};
use serde_json::Value;

use super::utils_str::ValueExt;

pub struct Base64Helper;

impl Base64Helper {
    pub const NAME: &'static str = "base64";
    pub const DECODE: &'static str = "decode";
    pub const PADDING: &'static str = "padding";

    fn value<'a>(h: &'a Helper) -> Option<&'a str> {
        h.params().get(0)
            .and_then(|it| it.value().as_str().or_else(|| it.relative_path().map(String::as_str)))
            .map(str::escape_single_quotes)
    }

    fn base64_encode(value: &str, with_padding: bool) -> String {
        if with_padding {
            base64::encode(value)
        } else {
            base64::encode_config(value, base64::STANDARD_NO_PAD)
        }
    }

    fn base64_decode(value: &str) -> String {
        base64::decode(value).ok()
            .and_then(|it| from_utf8(it.as_slice()).map(str::to_string).ok())
            .unwrap_or_else(|| value.to_string())
    }

    fn is_decode(h: &Helper) -> bool {
        h.hash_get(Self::DECODE)
            .map(PathAndJson::value)
            .and_then(Value::as_bool)
            .unwrap_or_default()
    }

    fn with_padding(h: &Helper) -> bool {
        h.hash_get(Self::PADDING)
            .map(PathAndJson::value)
            .and_then(Value::as_bool)
            .unwrap_or(true)
    }
}

impl HelperDef for Base64Helper {
    fn call<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>, out: &mut dyn Output) -> HelperResult {
        Self::value(h)
            .map(|value| if Self::is_decode(h) { Self::base64_decode(value) } else { Self::base64_encode(value, Self::with_padding(h)) })
            .ok_or_else(|| RenderError::new("Failed templating base 64 (de)encoding"))
            .and_then(|s| out.write(&s).map_err(RenderError::from))
    }
}