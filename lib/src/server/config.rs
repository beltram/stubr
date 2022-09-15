/// Stubr server configuration.
#[derive(Default, Copy, Clone)]
pub struct Config {
    /// HTTP port the mock server will be listening on
    pub port: Option<u16>,
    /// Enables turning off logs
    pub verbose: Option<bool>,
    /// Enables verification via https://docs.rs/wiremock/latest/wiremock/struct.Mock.html#method.expect
    pub verify: Option<bool>,
    /// Global delay in milliseconds.
    /// Supersedes any locally defined delay
    pub global_delay: Option<u64>,
    /// Global delay in milliseconds.
    /// Contrary to [global_delay], this one is added to any locally defined delay.
    /// Use it to simulate network delays.
    pub latency: Option<u64>,
}