use std::convert::TryFrom;

use wiremock::matchers::{path, PathExactMatcher};

use super::HttpUrlDto;

impl TryFrom<&HttpUrlDto> for PathExactMatcher {
    type Error = anyhow::Error;

    fn try_from(http_url: &HttpUrlDto) -> anyhow::Result<Self> {
        http_url.url_path.as_ref()
            .map(|it| path(it.as_str()))
            .ok_or_else(|| anyhow::Error::msg("No 'urlPath'"))
    }
}
