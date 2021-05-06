use std::convert::TryFrom;

use wiremock::matchers::{path_regex, PathRegexMatcher};

use super::HttpUrlStub;

impl TryFrom<&HttpUrlStub> for PathRegexMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrlStub) -> anyhow::Result<Self> {
        http_url.url_path_pattern.as_ref()
            .map(|it| path_regex(it.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No 'urlPathPattern'"))
    }
}
