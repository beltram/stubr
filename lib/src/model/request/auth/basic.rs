use std::str::FromStr;

use http_types::headers::HeaderName;
use serde::{Deserialize, Serialize};
use wiremock::{Match, Request};

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
#[serde(default, rename_all = "camelCase")]
pub struct BasicAuthStub {
    username: String,
    password: String,
}

lazy_static! {
    pub(crate) static ref AUTHORIZATION_HEADER: HeaderName = HeaderName::from_str("authorization").unwrap();
}

pub struct BasicAuthMatcher(String);

impl BasicAuthMatcher {
    const BASIC_PREFIX: &'static str = "Basic";
}


impl Match for BasicAuthMatcher {
    fn matches(&self, req: &Request) -> bool {
            req.headers.get(&AUTHORIZATION_HEADER)
            .map(|v| v.as_str() == self.0.as_str())
            .unwrap_or_default()
    }
}

impl From<&BasicAuthStub> for BasicAuthMatcher {
    fn from(dto: &BasicAuthStub) -> Self {
        let value = base64::encode(format!("{}:{}", dto.username, dto.password));
        Self(format!("{} {}", Self::BASIC_PREFIX, value))
    }
}