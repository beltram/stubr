use std::time::Duration;

use crate::wiremock::ResponseTemplate;

use crate::Config;

use super::ResponseAppender;

pub struct Delay<'a>(pub Option<u64>, pub &'a Config);

impl ResponseAppender for Delay<'_> {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(global_delay) = self.1.global_delay {
            resp = resp.set_delay(Duration::from_millis(global_delay))
        } else if let Some(latency) = self.1.latency {
            if let Some(delay) = self.0 {
                resp = resp.set_delay(Duration::from_millis(latency + delay))
            } else {
                resp = resp.set_delay(Duration::from_millis(latency))
            }
        } else if let Some(delay) = self.0 {
            resp = resp.set_delay(Duration::from_millis(delay))
        }
        resp
    }
}
