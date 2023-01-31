use crate::wiremock::{Match, Request};

use super::AUTHORIZATION_HEADER;

#[derive(Debug, Clone, Default, Hash, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct BasicAuthStub {
    username: String,
    password: String,
}

pub struct BasicAuthMatcher(String);

impl BasicAuthMatcher {
    const BASIC_PREFIX: &'static str = "Basic";
}

impl Match for BasicAuthMatcher {
    fn matches(&self, req: &Request) -> bool {
        req.headers
            .get(&AUTHORIZATION_HEADER)
            .map(|v| v.as_str() == self.0.as_str())
            .unwrap_or_default()
    }
}

impl From<&BasicAuthStub> for BasicAuthMatcher {
    fn from(stub: &BasicAuthStub) -> Self {
        use base64::Engine as _;
        let value = base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", stub.username, stub.password));
        Self(format!("{} {value}", Self::BASIC_PREFIX))
    }
}
