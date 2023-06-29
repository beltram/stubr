#![allow(dead_code)]

use crate::wiremock_rs::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};
use http_types::Method;

pub struct HttpProbe;

impl HttpProbe {
    const PATH: &'static str = "/healtz";

    pub fn health_probe() -> Mock {
        Mock::given(method(Method::Get))
            .and(path(Self::PATH))
            .respond_with(ResponseTemplate::new(200))
    }
}
