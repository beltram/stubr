use crate::{
    error::{StubrError, StubrResult},
    model::{request::RequestStub, JsonStub},
};
use http_types::{Method, Request, Url};

mod body;
mod header;
mod matcher;
mod method;
mod query;
mod url;

pub struct StdRequest(pub Request);

impl TryFrom<&JsonStub> for StdRequest {
    type Error = StubrError;

    fn try_from(stub: &JsonStub) -> StubrResult<Self> {
        let req = if let Some(req) = stub.http_request.as_ref() {
            Request::try_from(req)?
        } else {
            Request::try_from(&RequestStub::default())?
        };
        Ok(Self(req))
    }
}

impl TryFrom<&RequestStub> for Request {
    type Error = StubrError;

    fn try_from(stub: &RequestStub) -> StubrResult<Self> {
        let mut req = Request::new(Method::from(&stub.method), Url::try_from(stub)?);
        if let Ok(headers) = Vec::<(String, String)>::try_from(&stub.headers) {
            for (k, v) in headers {
                req.append_header(k.as_str(), v.as_str())
            }
        }
        req.set_body(Vec::<u8>::from(stub));
        Ok(req)
    }
}

#[cfg(test)]
mod verify_req_tests {
    use std::borrow::Cow;

    use serde_json::{Map, Value};

    use crate::model::request::{
        headers::HttpReqHeadersStub, matcher::MatcherValueStub, method::HttpMethodStub, query::HttpQueryParamsStub, url::HttpUrlStub,
    };

    use super::*;

    #[test]
    fn should_verify() {
        let stub = RequestStub {
            method: HttpMethodStub::from("POST"),
            url: HttpUrlStub {
                url: Some(String::from("/api/url?a=b&c=d")),
                ..Default::default()
            },
            queries: HttpQueryParamsStub {
                query_parameters: Some(Map::from_iter(vec![(
                    String::from("e"),
                    serde_json::to_value(MatcherValueStub {
                        equal_to: Some(Value::String(String::from("f"))),
                        ..Default::default()
                    })
                    .unwrap(),
                )])),
            },
            headers: HttpReqHeadersStub {
                headers: Some(Map::from_iter(vec![(
                    String::from("x-a"),
                    serde_json::to_value(MatcherValueStub {
                        equal_to: Some(Value::String(String::from("b"))),
                        ..Default::default()
                    })
                    .unwrap(),
                )])),
            },
            ..Default::default()
        };
        let req = Request::try_from(&stub).unwrap();
        assert_eq!(req.method(), Method::Post);
        assert_eq!(req.url().path(), "/api/url");
        let mut queries = req.url().query_pairs();
        assert_eq!(queries.next(), Some((Cow::Borrowed("a"), Cow::Borrowed("b"))));
        assert_eq!(queries.next(), Some((Cow::Borrowed("c"), Cow::Borrowed("d"))));
        assert_eq!(queries.next(), Some((Cow::Borrowed("e"), Cow::Borrowed("f"))));
        assert_eq!(req.header("x-a").unwrap().as_str(), "b");
    }
}
