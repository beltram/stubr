use std::convert::TryFrom;

use http_types::Url;
use itertools::Itertools;
use wiremock::matchers::{path, PathExactMatcher, query_param, QueryParamExactMatcher};

use super::HttpUrlDto;

pub struct ExactPathAndQueryMatcher(pub PathExactMatcher, pub Vec<QueryParamExactMatcher>);

impl TryFrom<&HttpUrlDto> for ExactPathAndQueryMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrlDto) -> anyhow::Result<Self> {
        Url::try_from(http_url)
            .map(Self::from)
            .map_err(anyhow::Error::msg)
    }
}

impl TryFrom<&HttpUrlDto> for Url {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrlDto) -> anyhow::Result<Self> {
        http_url.url.as_ref()
            .map(|it| format!("http://localhost{}", it))
            .and_then(|it| Url::parse(&it).ok())
            .ok_or_else(|| anyhow::Error::msg("No 'url'"))
    }
}

impl From<Url> for ExactPathAndQueryMatcher {
    fn from(url: Url) -> Self {
        let query_matchers = url.query_pairs()
            .map(|(k, v)| query_param(k, v))
            .collect_vec();
        Self(path(url.path()), query_matchers)
    }
}