use itertools::Itertools;
use wiremock::{Match, Request};

use super::{HttpQueryParamsStub, super::matcher::RequestMatcherStub};

pub struct QueryAbsentMatcher(String, bool);

impl Match for QueryAbsentMatcher {
    fn matches(&self, req: &Request) -> bool {
        let is_absent = req.url.query_pairs().all(|(k, _)| k.ne(&self.0));
        is_absent == self.1
    }
}

impl TryFrom<&HttpQueryParamsStub> for Vec<QueryAbsentMatcher> {
    type Error = anyhow::Error;

    fn try_from(queries: &HttpQueryParamsStub) -> anyhow::Result<Self> {
        queries.get_queries()
            .ok_or_else(|| anyhow::Error::msg(""))
            .map(|iter| {
                iter.filter(|it| it.is_absent())
                    .filter_map(|it| QueryAbsentMatcher::try_from(&it).ok())
                    .collect_vec()
            })
    }
}

impl TryFrom<&RequestMatcherStub> for QueryAbsentMatcher {
    type Error = anyhow::Error;

    fn try_from(query: &RequestMatcherStub) -> anyhow::Result<Self> {
        query.value.as_ref()
            .filter(|_| query.is_absent())
            .map(|it| QueryAbsentMatcher(query.key.to_string(), it.absent.unwrap_or_default()))
            .ok_or_else(|| anyhow::Error::msg("No query absent matcher found"))
    }
}
