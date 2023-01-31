use async_std::io::ReadExt;
use std::{borrow::Cow, collections::HashMap, iter, str::from_utf8};

use crate::wiremock::Request;
use futures::executor::block_on;
use itertools::Itertools;
use serde_json::Value;

pub(crate) type Queries<'a> = HashMap<Cow<'a, str>, Value>;
pub(crate) type Headers<'a> = HashMap<&'a str, Value>;

pub(crate) trait RequestExt {
    fn uri(&self) -> &str;
    fn path(&self) -> &str {
        self.uri()
    }
    fn path_segments(&self) -> Option<Vec<&str>>;
    fn body(&self) -> Option<Value> {
        None
    }
    fn body_mut(&mut self) -> Option<Value> {
        None
    }
    fn queries(&self) -> Option<Queries<'_>>;
    fn headers(&self) -> Option<Headers<'_>>;
}

impl RequestExt for Request {
    fn uri(&self) -> &str {
        self.url
            .host_str()
            .and_then(|host| self.url.as_str().split(host).last())
            .unwrap_or_else(|| self.url.as_str())
    }

    fn path_segments(&self) -> Option<Vec<&str>> {
        self.url
            .path_segments()
            .map(|it| it.collect_vec())
            .filter(|it| it.first() != Some(&""))
    }

    fn body(&self) -> Option<Value> {
        if !self.body.is_empty() {
            serde_json::from_slice::<Value>(self.body.as_slice())
                .ok()
                .or_else(|| from_utf8(self.body.as_slice()).ok().map(|s| Value::String(s.to_string())))
        } else {
            None
        }
    }

    fn queries(&self) -> Option<Queries<'_>> {
        let queries = self
            .url
            .query_pairs()
            .into_group_map()
            .into_iter()
            .map(|(k, v)| {
                let mut iter = v.iter();
                let values = if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        iter::once(first)
                            .chain(iter::once(second))
                            .chain(iter)
                            .map(|it| it.as_ref())
                            .collect()
                    } else {
                        Value::from(first.as_ref())
                    }
                } else {
                    Value::Null
                };
                (k, values)
            })
            .collect::<Queries>();
        if queries.is_empty() {
            None
        } else {
            Some(queries)
        }
    }

    fn headers(&self) -> Option<Headers> {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| {
                let mut iter = v.iter();
                let values = if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        iter::once(first)
                            .chain(iter::once(second))
                            .chain(iter)
                            .map(|it| it.as_str())
                            .collect()
                    } else {
                        Value::from(first.as_str())
                    }
                } else {
                    Value::Null
                };
                (k.as_str(), values)
            })
            .collect::<Headers>();
        if headers.is_empty() {
            None
        } else {
            Some(headers)
        }
    }
}

impl RequestExt for http_types::Request {
    fn uri(&self) -> &str {
        self.url()
            .host_str()
            .and_then(|host| self.url().as_str().split(host).last())
            .unwrap_or_else(|| self.url().as_str())
    }

    fn path(&self) -> &str {
        self.url().path()
    }

    fn path_segments(&self) -> Option<Vec<&str>> {
        self.url()
            .path_segments()
            .map(|it| it.collect_vec())
            .filter(|it| it.first() != Some(&""))
    }

    fn body_mut(&mut self) -> Option<Value> {
        block_on(async {
            let mut bytes = vec![];
            self.read_to_end(&mut bytes)
                .await
                .ok()
                .map(|_| bytes)
                .filter(|b| !b.is_empty())
                .and_then(|b| {
                    serde_json::from_slice::<Value>(b.as_slice())
                        .ok()
                        .or_else(|| from_utf8(b.as_slice()).ok().map(|s| Value::String(s.to_string())))
                })
        })
    }

    fn queries(&self) -> Option<Queries<'_>> {
        let queries = self
            .url()
            .query_pairs()
            .into_group_map()
            .into_iter()
            .map(|(k, v)| {
                let mut iter = v.iter();
                let values = if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        iter::once(first)
                            .chain(iter::once(second))
                            .chain(iter)
                            .map(|it| it.as_ref())
                            .collect()
                    } else {
                        Value::from(first.as_ref())
                    }
                } else {
                    Value::Null
                };
                (k, values)
            })
            .collect::<Queries>();
        if queries.is_empty() {
            None
        } else {
            Some(queries)
        }
    }

    fn headers(&self) -> Option<Headers<'_>> {
        let headers = self
            .header_names()
            .filter_map(|k| self.header(k).map(|v| (k, v)))
            .map(|(k, v)| {
                let mut iter = v.iter();
                let values = if let Some(first) = iter.next() {
                    if let Some(second) = iter.next() {
                        iter::once(first)
                            .chain(iter::once(second))
                            .chain(iter)
                            .map(|it| it.as_str())
                            .collect()
                    } else {
                        Value::from(first.as_str())
                    }
                } else {
                    Value::Null
                };
                (k.as_str(), values)
            })
            .collect::<Headers>();
        if headers.is_empty() {
            None
        } else {
            Some(headers)
        }
    }
}
