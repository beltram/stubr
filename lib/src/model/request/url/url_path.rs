use crate::wiremock_rs::matchers::{path, PathExactMatcher};
use crate::{StubrError, StubrResult};

use super::HttpUrlStub;

impl TryFrom<&HttpUrlStub> for PathExactMatcher {
    type Error = StubrError;

    fn try_from(http_url: &HttpUrlStub) -> StubrResult<Self> {
        http_url.url_path.as_deref().map(path).ok_or_else(|| StubrError::QuietError)
    }
}
