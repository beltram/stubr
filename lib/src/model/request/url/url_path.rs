use std::convert::TryFrom;

use wiremock::matchers::{path, PathExactMatcher};

use super::HttpUrlStub;

impl TryFrom<&HttpUrlStub> for PathExactMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrlStub) -> anyhow::Result<Self> {
        http_url.url_path.as_ref()
            .map(|it| path(it.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No 'urlPath'"))
    }
}
