use itertools::Itertools;
use jsonwebtoken::Header;
use serde_json::Value;
use wiremock::Request;

use super::{AUTHORIZATION_HEADER, BEARER_PREFIX};

pub trait RequestAuthExtension {

    fn authorization_header(&self) -> Option<&str>;

    fn jwt(&self) -> Option<&str> {
        self.authorization_header()
            .filter(|h| h.contains(BEARER_PREFIX))
            .map(|h| &h[BEARER_PREFIX.len() + 1..])
    }

    fn jwt_header(&self) -> Option<Header> {
        self.jwt()
            .and_then(|jwt| jsonwebtoken::decode_header(jwt).ok())
    }

    fn jwt_payload(&self) -> Option<Value> {
        self.jwt()
            .and_then(|jwt| {
                jwt.split('.').collect_vec()
                    .get(1)
                    .and_then(|it| base64::decode(it).ok())
                    .and_then(|it| serde_json::from_slice(it.as_slice()).ok())
            })
    }
}

impl RequestAuthExtension for Request {
    fn authorization_header(&self) -> Option<&str> {
        self.headers.get(&AUTHORIZATION_HEADER)
            .map(|v| v.as_str())
    }
}