use chrono::prelude::*;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

pub struct NowHelper;

impl NowHelper {
    pub const NAME: &'static str = "now";
    const FORMAT: &'static str = "format";
    const QUOTE: char = '\'';

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn fmt_with_custom_format(now: DateTime<Utc>, h: &Helper) -> String {
        if let Some(format) = Self::get_custom_format(h) {
            simpledateformat::fmt(format).unwrap().format(&now)
        } else {
            now.to_rfc3339_opts(SecondsFormat::Secs, true)
        }
    }

    fn get_custom_format<'a>(h: &'a Helper) -> Option<&'a str> {
        h.hash_get(Self::FORMAT)
            .and_then(|it| it.relative_path())
            .map(|it| it.trim_start_matches(Self::QUOTE).trim_end_matches(Self::QUOTE))
    }
}

impl HelperDef for NowHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let now = Self::now();
        let now = Self::fmt_with_custom_format(now, h);
        out.write(now.as_str()).unwrap();
        Ok(())
    }
}