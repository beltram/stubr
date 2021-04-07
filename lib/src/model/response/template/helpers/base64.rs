use std::str::from_utf8;

use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

use super::traits::ValueExt;

pub struct Base64Helper;

impl Base64Helper {
    pub const NAME: &'static str = "base64";
    pub const DECODE: &'static str = "decode";
    pub const PADDING: &'static str = "padding";

    fn value<'a>(h: &'a Helper) -> &'a str {
        h.params().get(0)
            .and_then(|it| it.value().as_str().or_else(|| it.relative_path().map(|p| p.as_str())))
            .map(|it| it.escape_single_quotes())
            .unwrap()
    }

    fn base64_encode(h: &Helper) -> String {
        let value = Self::value(h);
        if Self::with_padding(h) {
            base64::encode(value)
        } else {
            base64::encode_config(value, base64::STANDARD_NO_PAD)
        }
    }

    fn base64_decode(h: &Helper) -> String {
        let value = Self::value(h);
        base64::decode(value).ok()
            .and_then(|it| from_utf8(it.as_slice()).map(|s| s.to_string()).ok())
            .unwrap_or_else(|| value.to_string())
    }

    fn is_decode(h: &Helper) -> bool {
        h.hash_get(Self::DECODE)
            .and_then(|it| it.value().as_bool())
            .unwrap_or_default()
    }

    fn with_padding(h: &Helper) -> bool {
        h.hash_get(Self::PADDING)
            .and_then(|it| it.value().as_bool())
            .unwrap_or(true)
    }
}

impl HelperDef for Base64Helper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let rendered = if Self::is_decode(h) { Self::base64_decode(h) } else { Self::base64_encode(h) };
        out.write(rendered.as_str()).unwrap();
        Ok(())
    }
}