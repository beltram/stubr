use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::matchers::{path, path_regex, PathExactMatcher, PathRegexMatcher};
use wiremock::MockBuilder;

#[derive(Deserialize, Debug, Default)]
pub struct HttpUrl {
    // exact match on path only
    #[serde(rename(deserialize = "urlPath"))]
    url_path: Option<String>,
    // regex match on path only
    #[serde(rename(deserialize = "urlPathPattern"))]
    url_path_pattern: Option<String>,
    // exact match on path and query
    url: Option<String>,
    // regex match on path and query
    #[serde(rename(deserialize = "urlPattern"))]
    url_pattern: Option<String>,
}

impl HttpUrl {
    pub fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(exact) = PathExactMatcher::try_from(self) {
            mock = mock.and(exact);
        }
        if let Ok(regex) = PathRegexMatcher::try_from(self) {
            mock = mock.and(regex);
        }
        mock
    }
}

impl TryFrom<&HttpUrl> for PathExactMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrl) -> Result<Self, Self::Error> {
        http_url.url_path.as_ref()
            .map(|it| path(it.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No 'urlPath'"))
    }
}

impl TryFrom<&HttpUrl> for PathRegexMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrl) -> Result<Self, Self::Error> {
        http_url.url_path_pattern.as_ref()
            .map(|it| path_regex(it.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No 'urlPathPattern'"))
    }
}