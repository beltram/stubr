use std::time::Duration;

use wiremock::ResponseTemplate;

use super::{ResponseAppender, StubDto};

pub struct Delay<'a>(pub &'a StubDto);

impl ResponseAppender for Delay<'_> {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(delay) = self.0.response.fixed_delay_milliseconds {
            resp = resp.set_delay(Duration::from_millis(delay))
        }
        resp
    }
}