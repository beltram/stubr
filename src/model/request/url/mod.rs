use std::convert::TryFrom;

use serde::Deserialize;
use wiremock::matchers::{PathExactMatcher, PathRegexMatcher};
use wiremock::MockBuilder;

use just_url::ExactPathAndQueryMatcher;

use super::MockRegistrable;

mod url_path;
mod url_path_pattern;
mod just_url;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HttpUrl {
    // exact match on path only
    url_path: Option<String>,
    // regex match on path only
    url_path_pattern: Option<String>,
    // exact match on path and query
    url: Option<String>,
    // regex match on path and query
    url_pattern: Option<String>,
}

impl MockRegistrable for HttpUrl {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(exact) = PathExactMatcher::try_from(self) {
            mock = mock.and(exact);
        }
        if let Ok(regex) = PathRegexMatcher::try_from(self) {
            mock = mock.and(regex);
        }
        if let Ok(ExactPathAndQueryMatcher(path, queries)) = ExactPathAndQueryMatcher::try_from(self) {
            mock = mock.and(path);
            for query in queries {
                mock = mock.and(query);
            }
        }
        mock
    }
}