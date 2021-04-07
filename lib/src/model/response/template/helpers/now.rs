use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Duration, prelude::*};
use chrono_tz::Tz;
use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use humantime::parse_duration;

use super::traits::ValueExt;

pub struct NowHelper;

impl NowHelper {
    pub const NAME: &'static str = "now";
    const FORMAT: &'static str = "format";
    const OFFSET: &'static str = "offset";
    const TIMEZONE: &'static str = "timezone";
    const EPOCH: &'static str = "epoch";
    const UNIX: &'static str = "unix";

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn fmt_with_custom_format(now: DateTime<Utc>, h: &Helper) -> String {
        if let Some(format) = Self::get_hash(h, Self::FORMAT) {
            match format {
                Self::EPOCH => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string(),
                Self::UNIX => SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
                _ => simpledateformat::fmt(format).unwrap().format(&now)
            }
        } else {
            now.to_rfc3339_opts(SecondsFormat::Secs, true)
        }
    }

    fn get_hash<'a>(h: &'a Helper, key: &str) -> Option<&'a str> {
        h.hash_get(key)?.relative_path()
            .map(|it| it.escape_single_quotes())
    }

    fn apply_offset<'a>(now: DateTime<Utc>, h: &'a Helper) -> DateTime<Utc> {
        Self::get_hash(h, Self::OFFSET)
            .map(|it| it.replace(' ', ""))
            .and_then(|offset| Self::compute_offset(now, offset))
            .unwrap_or(now)
    }

    fn compute_offset(now: DateTime<Utc>, offset: String) -> Option<DateTime<Utc>> {
        let is_negative = offset.starts_with('-');
        let offset = if is_negative { offset.trim_start_matches('-') } else { offset.as_str() };
        parse_duration(offset).ok()
            .and_then(|it| Duration::from_std(it).ok())
            .map(|rhs| if is_negative { now - rhs } else { now + rhs })
    }

    fn apply_timezone<'a>(now: DateTime<Utc>, h: &'a Helper) -> DateTime<Utc> {
        Self::get_hash(h, Self::TIMEZONE)
            .and_then(|timezone| timezone.parse().ok())
            .map(|tz: Tz| {
                let offset_seconds = tz.offset_from_utc_datetime(&now.naive_utc()).fix().local_minus_utc();
                Duration::seconds(offset_seconds.into())
            })
            .map(|offset: Duration| now + offset)
            .unwrap_or(now)
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
        let now = Self::apply_timezone(now, h);
        let now = Self::apply_offset(now, h);
        let now = Self::fmt_with_custom_format(now, h);
        out.write(now.as_str()).unwrap();
        Ok(())
    }
}