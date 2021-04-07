use std::ops::Add;

use chrono::Duration;
use chrono::prelude::*;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use humantime::parse_duration;

pub struct NowHelper;

impl NowHelper {
    pub const NAME: &'static str = "now";
    const FORMAT: &'static str = "format";
    const OFFSET: &'static str = "offset";
    const QUOTE: char = '\'';

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn fmt_with_custom_format(now: DateTime<Utc>, h: &Helper) -> String {
        if let Some(format) = Self::get_hash(h, Self::FORMAT) {
            simpledateformat::fmt(format).unwrap().format(&now)
        } else {
            now.to_rfc3339_opts(SecondsFormat::Secs, true)
        }
    }

    fn get_hash<'a>(h: &'a Helper, key: &str) -> Option<&'a str> {
        h.hash_get(key)?.relative_path()
            .map(|it| it.trim_start_matches(Self::QUOTE).trim_end_matches(Self::QUOTE))
    }

    fn apply_offset<'a>(now: DateTime<Utc>, h: &'a Helper) -> DateTime<Utc> {
        Self::get_hash(h, Self::OFFSET)
            .map(|it| it.replace(' ', ""))
            .and_then(|it| parse_duration(&it).ok())
            .and_then(|it| Duration::from_std(it).ok())
            .map(|it| now.add(it))
            .unwrap_or_else(|| now)
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
        let now = Self::apply_offset(now, h);
        let now = Self::fmt_with_custom_format(now, h);
        out.write(now.as_str()).unwrap();
        Ok(())
    }
}