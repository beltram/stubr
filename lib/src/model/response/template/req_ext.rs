use std::{borrow::Cow, collections::HashMap, iter, str::from_utf8};

use itertools::Itertools;
use serde_json::Value;
use wiremock::Request;

pub(crate) type Queries<'a> = HashMap<Cow<'a, str>, Value>;
pub(crate) type Headers<'a> = HashMap<&'a str, Value>;

pub(crate) trait RequestExt {
    fn url(&self) -> &str;
    fn path_segments(&self) -> Option<Vec<&str>>;
    fn body(&self) -> Option<Value>;
    fn queries(&self) -> Option<Queries<'_>>;
    fn headers(&self) -> Option<Headers<'_>>;
}

impl RequestExt for Request {
    fn url(&self) -> &str {
        self.url.host_str()
            .and_then(|host| self.url.as_str().split(host).last())
            .unwrap_or_else(|| self.url.as_str())
    }

    fn path_segments(&self) -> Option<Vec<&str>> {
        self.url.path_segments()
            .map(|it| it.collect_vec())
            .filter(|it| it.get(0) != Some(&""))
    }

    fn body(&self) -> Option<Value> {
        if !self.body.is_empty() {
            serde_json::from_slice::<Value>(self.body.as_slice()).ok()
                .or_else(|| {
                    from_utf8(self.body.as_slice()).ok()
                        .map(|body_str| Value::String(body_str.to_string()))
                })
        } else { None }
    }

    fn queries(&self) -> Option<Queries<'_>> {
        let queries = self.url.query_pairs().into_group_map().into_iter()
            .map(|(k, v)| {
                let mut iter = v.iter();
                let values = if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        iter::once(first)
                            .chain(iter::once(second))
                            .chain(iter)
                            .map(|it| it.as_ref())
                            .collect()
                    } else { Value::from(first.as_ref()) }
                } else { Value::Null };
                (k, values)
            })
            .collect::<Queries>();
        if queries.is_empty() { None } else { Some(queries) }
    }

    fn headers(&self) -> Option<Headers> {
        let headers = self.headers.iter()
            .map(|(k, v)| {
                let mut iter = v.iter();
                let values = if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        iter::once(first)
                            .chain(iter::once(second))
                            .chain(iter)
                            .map(|it| it.as_str())
                            .collect()
                    } else { Value::from(first.as_str()) }
                } else { Value::Null };
                (k.as_str(), values)
            })
            .collect::<Headers>();
        if headers.is_empty() { None } else { Some(headers) }
    }
}