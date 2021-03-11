/// Stubr server configuration.
#[derive(Default)]
pub struct Config {
    /// HTTP the mock server will be listening on
    pub port: Option<u16>,
    /// enables turning off logs
    pub verbose: Option<bool>,
    /// global delay in milliseconds. Supersedes any locally defined delay
    pub global_delay: Option<u64>,
}