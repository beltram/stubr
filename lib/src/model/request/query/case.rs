use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpQueryParamsStub, super::matcher::RequestMatcherStub};

pub struct QueryCaseInsensitiveMatcher(String, String);

impl Match for QueryCaseInsensitiveMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.url.query_pairs()
            .find(|(k, _)| k == self.0.as_str())
            .map(|(_, v)| v.eq_ignore_ascii_case(self.1.as_str()))
            .unwrap_or_default()
    }
}

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryCaseInsensitiveMatcher> {
    type Error = anyhow::Error;

    fn try_from(queries: &HttpQueryParamsStub) -> anyhow::Result<Self> {
        queries.get_queries()
            .ok_or_else(|| anyhow::Error::msg(""))
            .map(|iter| {
                iter.filter(|q| q.is_case_insensitive())
                    .filter_map(|it| QueryCaseInsensitiveMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryCaseInsensitiveMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherStub) -> anyhow::Result<Self> {
        query.equal_to_as_str()
            .filter(|_| query.is_case_insensitive())
            .map(|it| QueryCaseInsensitiveMatcher(query.key.to_string(), it))
            .ok_or_else(|| anyhow::Error::msg("No case insensitive query matcher found"))
    }
}
