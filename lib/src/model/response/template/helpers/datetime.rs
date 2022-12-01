use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{prelude::*, Duration};
use chrono_tz::Tz;
use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use humantime::parse_duration;
use serde_json::Value;

use super::utils_str::ValueExt;

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

    fn fmt_with_custom_format(now: DateTime<Utc>, h: &Helper) -> Option<String> {
        if let Some(format) = Self::get_hash(h, Self::FORMAT) {
            match format {
                Self::EPOCH => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_millis().to_string())
                    .ok(),
                Self::UNIX => SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs().to_string())
                    .ok(),
                _ => simpledateformat::fmt(format).map(|d| d.format(&now)).ok(),
            }
        } else {
            Some(now.to_rfc3339_opts(SecondsFormat::Secs, true))
        }
    }

    fn get_hash<'a>(h: &'a Helper, key: &str) -> Option<&'a str> {
        h.hash_get(key)?.relative_path().map(String::escape_single_quotes)
    }

    fn apply_offset<'a>(now: DateTime<Utc>, h: &'a Helper) -> DateTime<Utc> {
        Self::get_hash(h, Self::OFFSET)
            .map(|it| it.replace(' ', ""))
            .and_then(|offset| Self::compute_offset(now, offset))
            .unwrap_or(now)
    }

    fn compute_offset(now: DateTime<Utc>, offset: String) -> Option<DateTime<Utc>> {
        let is_negative = offset.starts_with('-');
        let offset = if is_negative {
            offset.trim_start_matches('-')
        } else {
            offset.as_str()
        };
        parse_duration(offset)
            .ok()
            .and_then(|it| Duration::from_std(it).ok())
            .map(|rhs| if is_negative { now - rhs } else { now + rhs })
    }

    fn apply_timezone<'a>(now: DateTime<Utc>, h: &'a Helper) -> DateTime<Utc> {
        Self::get_hash(h, Self::TIMEZONE)
            .and_then(|timezone| timezone.parse().ok())
            .map(|tz: Tz| tz.offset_from_utc_datetime(&now.naive_utc()).fix().local_minus_utc())
            .map(i64::from)
            .map(Duration::seconds)
            .map(|offset: Duration| now + offset)
            .unwrap_or(now)
    }
}

impl HelperDef for NowHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let now = Self::now();
        let now = Self::apply_timezone(now, h);
        let now = Self::apply_offset(now, h);
        Self::fmt_with_custom_format(now, h)
            .map(Value::from)
            .map(ScopedJson::from)
            .ok_or_else(|| RenderError::new("Failed rendering now with custom format"))
    }
}
