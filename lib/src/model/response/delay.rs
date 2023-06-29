use std::time::Duration;

use crate::wiremock_rs::ResponseTemplate;

use crate::Config;

use super::ResponseAppender;

/// See [https://wiremock.org/docs/simulating-faults/#per-stub-fixed-delays]
pub struct Delay<'a>(pub Option<u64>, pub &'a Option<RandomDelay>, pub &'a Config);

impl ResponseAppender for Delay<'_> {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(global_delay) = self.2.global_delay {
            resp = resp.set_delay(Duration::from_millis(global_delay))
        } else if let Some(latency) = self.2.latency {
            if let Some(delay) = self.0 {
                resp = resp.set_delay(Duration::from_millis(latency + delay))
            } else {
                resp = resp.set_delay(Duration::from_millis(latency))
            }
        } else if let Some(delay) = self.0 {
            resp = resp.set_delay(Duration::from_millis(delay))
        } else if let Some(RandomDelay::Lognormal { median, sigma }) = self.1 {
            resp = resp.set_lognormal_delay(*median, *sigma)
        }
        resp
    }
}

/// See [https://wiremock.org/docs/simulating-faults/#per-stub-random-delays]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum RandomDelay {
    #[serde(rename = "lognormal")]
    Lognormal { median: u64, sigma: f64 },
}
