use serde::Deserialize;
use wiremock::matchers::{method, MethodExactMatcher};

#[derive(Deserialize, Debug, Default)]
pub struct HttpMethodDto(String);

impl From<HttpMethodDto> for MethodExactMatcher {
    fn from(http_method: HttpMethodDto) -> Self {
        method(http_method.0.as_str())
    }
}
