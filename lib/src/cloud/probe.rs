use wiremock::{Mock, ResponseTemplate, matchers::method};
use http_types::Method;
use wiremock::matchers::path;

pub struct HttpProbe;

impl HttpProbe {
    const PATH: &'static str = "/healtz";

    pub fn health_probe() -> Mock {
        Mock::given(method(Method::Get))
            .and(path(Self::PATH))
            .respond_with(ResponseTemplate::new(200))
    }
}