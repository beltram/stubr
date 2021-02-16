/// Stubr server configuration.
#[derive(Default)]
pub struct Config {
    /// HTTP the mock server will be listening on
    pub port: Option<u16>,
    /// enables turning off logs
    pub verbose: Option<bool>,
}