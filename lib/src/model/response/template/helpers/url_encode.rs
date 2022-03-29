use handlebars::{Context, Handlebars, Helper, HelperDef, PathAndJson, RenderContext, RenderError, ScopedJson};
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use serde_json::Value;

use super::utils_str::ValueExt;

pub struct UrlEncodingHelper;

impl UrlEncodingHelper {
    pub const NAME: &'static str = "urlEncode";
    pub const DECODE: &'static str = "decode";

    fn value<'a>(h: &'a Helper) -> Option<&'a str> {
        h.params().get(0)
            .and_then(|p| p.value().as_str().or_else(|| p.relative_path().map(String::as_str)))
            .map(str::escape_single_quotes)
    }

    fn url_encode(raw: &str) -> Option<String> {
        Some(utf8_percent_encode(raw, NON_ALPHANUMERIC).to_string())
    }

    fn url_decode(raw: &str) -> Option<String> {
        percent_decode_str(raw).decode_utf8()
            .map(|d| d.to_string())
            .ok()
    }

    fn is_decode(h: &Helper) -> bool {
        h.hash_get(Self::DECODE)
            .map(PathAndJson::value)
            .and_then(Value::as_bool)
            .unwrap_or_default()
    }
}

impl HelperDef for UrlEncodingHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Self::value(h)
            .and_then(|v| if Self::is_decode(h) { Self::url_decode(v) } else { Self::url_encode(v) })
            .ok_or_else(|| RenderError::new("Invalid url (de)encoding response template"))
            .map(Value::from)
            .map(ScopedJson::from)
    }
}