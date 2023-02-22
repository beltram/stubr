use crate::wiremock::matchers::{path_regex, PathRegexMatcher};
use crate::{StubrError, StubrResult};

use super::HttpUrlStub;

impl TryFrom<&HttpUrlStub> for PathRegexMatcher {
    type Error = StubrError;

    fn try_from(http_url: &HttpUrlStub) -> StubrResult<Self> {
        http_url
            .url_path_pattern
            .as_deref()
            .map(path_regex)
            .ok_or_else(|| StubrError::QuietError)
    }
}
