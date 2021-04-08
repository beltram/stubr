use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};

use super::traits::ValueExt;

pub struct UrlEncodingHelper;

impl UrlEncodingHelper {
    pub const NAME: &'static str = "urlEncode";
    pub const DECODE: &'static str = "decode";

    fn value<'a>(h: &'a Helper) -> &'a str {
        h.params().get(0)
            .and_then(|it| it.value().as_str().or_else(|| it.relative_path().map(|p| p.as_str())))
            .map(|it| it.escape_single_quotes())
            .unwrap()
    }

    fn url_encode(h: &Helper) -> String {
        utf8_percent_encode(Self::value(h), NON_ALPHANUMERIC).to_string()
    }

    fn url_decode(h: &Helper) -> String {
        percent_decode_str(Self::value(h)).decode_utf8().unwrap().to_string()
    }

    fn is_decode(h: &Helper) -> bool {
        h.hash_get(Self::DECODE)
            .and_then(|it| it.value().as_bool())
            .unwrap_or_default()
    }
}

impl HelperDef for UrlEncodingHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let rendered = if Self::is_decode(h) { Self::url_decode(h) } else { Self::url_encode(h) };
        out.write(rendered.as_str()).unwrap();
        Ok(())
    }
}