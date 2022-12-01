use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wiremock::{matchers::HeaderExactMatcher, MockBuilder};

use absent::HeaderAbsentMatcher;
use case::HeaderCaseInsensitiveMatcher;
use contains::HeaderContainsMatcher;
use matches::HeaderRegexMatcher;

use super::{super::request::MockRegistrable, matcher::RequestMatcherStub};

mod absent;
mod case;
mod contains;
mod exact;
mod matches;

#[derive(Serialize, Deserialize, Debug, Default, Eq)]
pub struct HttpReqHeadersStub {
    // matches all request http headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Map<String, Value>>,
}

impl MockRegistrable for HttpReqHeadersStub {
    fn register(&self, mut mock: MockBuilder) -> MockBuilder {
        if let Ok(matchers) = Vec::<HeaderExactMatcher>::try_from(self) {
            for exact in matchers {
                mock = mock.and(exact);
            }
        }
        if let Ok(matchers) = Vec::<HeaderCaseInsensitiveMatcher>::try_from(self) {
            for case_insensitive in matchers {
                mock = mock.and(case_insensitive);
            }
        }
        if let Ok(matchers) = Vec::<HeaderContainsMatcher>::try_from(self) {
            for contains in matchers {
                mock = mock.and(contains);
            }
        }
        if let Ok(matchers) = Vec::<HeaderRegexMatcher>::try_from(self) {
            for matches in matchers {
                mock = mock.and(matches);
            }
        }
        if let Ok(matchers) = Vec::<HeaderAbsentMatcher>::try_from(self) {
            for absent in matchers {
                mock = mock.and(absent);
            }
        }
        mock
    }
}

impl HttpReqHeadersStub {
    pub fn get_headers(&self) -> Option<impl Iterator<Item = RequestMatcherStub> + '_> {
        self.headers
            .as_ref()
            .map(|h| h.iter().filter_map(|it| RequestMatcherStub::try_from(it).ok()))
    }
}

impl PartialEq for HttpReqHeadersStub {
    fn eq(&self, other: &Self) -> bool {
        self.headers.as_ref().eq(&other.headers.as_ref())
    }
}

impl Hash for HttpReqHeadersStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(headers) = self.headers.as_ref() {
            headers.iter().for_each(|(k, v)| {
                k.hash(state);
                v.is_string().hash(state);
            })
        }
    }
}
