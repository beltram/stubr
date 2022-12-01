use serde::{Deserialize, Serialize};
use wiremock::{Match, Request};

use super::AUTHORIZATION_HEADER;

#[derive(Serialize, Deserialize, Debug, Default, Hash)]
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
        let value = base64::encode(format!("{}:{}", stub.username, stub.password));
        Self(format!("{} {}", Self::BASIC_PREFIX, value))
    }
}
