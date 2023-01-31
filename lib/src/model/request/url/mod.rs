use crate::wiremock::{
    matchers::{PathExactMatcher, PathRegexMatcher},
    MockBuilder,
};

use just_url::ExactPathAndQueryMatcher;
use url_pattern::UrlPatternMatcher;

use super::MockRegistrable;

mod just_url;
mod url_path;
mod url_path_pattern;
mod url_pattern;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpUrlStub {
    // exact match on path only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
    // regex match on path only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path_pattern: Option<String>,
    // exact match on path and query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // regex match on path and query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_pattern: Option<String>,
}

impl MockRegistrable for HttpUrlStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(ExactPathAndQueryMatcher(path, queries)) = ExactPathAndQueryMatcher::try_from(self) {
            mock = mock.and(path);
            for query in queries {
                mock = mock.and(query);
            }
        } else if let Ok(exact) = PathExactMatcher::try_from(self) {
            mock = mock.and(exact);
        } else if let Ok(url_pattern_matcher) = UrlPatternMatcher::try_from(self) {
            mock = mock.and(url_pattern_matcher);
        } else if let Ok(regex) = PathRegexMatcher::try_from(self) {
            mock = mock.and(regex);
        }
        mock
    }
}
