/// A distribution of a random delay
#[derive(Debug, Clone)]
pub struct LognormalDelay {
    pub median: u64,
    pub sigma: f64,
}

impl LognormalDelay {
    /// see [https://github.com/wiremock/wiremock/blob/60e9e858068548786af4a1a434b52fd1376c4d43/src/main/java/com/github/tomakehurst/wiremock/http/LogNormal.java#L52]
    pub fn new_sample(&self) -> core::time::Duration {
        let mean = self.median as f64;
        // TODO: error handling
        let normal = rand_distr::LogNormal::new(mean.ln(), self.sigma).unwrap();

        use rand_distr::Distribution as _;
        let milli = normal.sample(&mut rand::thread_rng()) as u64;
        core::time::Duration::from_millis(milli)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn lognormal_should_return_expected_mean() {
        const ROUNDS: usize = 10_000;
        const DELTA: f64 = 5.0;
        const EXPECTED: f64 = 97.1115;
        const LOWER: f64 = EXPECTED - DELTA;
        const UPPER: f64 = EXPECTED + DELTA;

        let lognormal = LognormalDelay { median: 90, sigma: 0.39 };
        let mut sum = 0.0;
        for _ in 0..ROUNDS {
            let sample = lognormal.new_sample().as_millis() as f64;
            sum += sample;
        }
        let mean = sum / (ROUNDS as f64);
        assert!((mean > LOWER) && (mean < UPPER));
    }
}
