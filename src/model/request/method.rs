use serde::Deserialize;
use wiremock::matchers::{method, MethodExactMatcher};

#[derive(Deserialize, Debug, Default)]
pub struct HttpMethod(String);

impl From<HttpMethod> for MethodExactMatcher {
    fn from(http_method: HttpMethod) -> Self {
        method(http_method.0.as_str())
    }
}
