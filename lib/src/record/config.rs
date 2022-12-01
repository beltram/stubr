use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct RecordConfig {
    /// Port number the recording proxy server is listening on.
    /// Defaults to a random one.
    pub port: Option<u16>,
    /// Directory where recorded stubs will be written.
    /// Defaults to 'target/stubs'
    pub output: Option<PathBuf>,
    /// Do not record those request headers
    pub except_request_headers: Option<Vec<&'static str>>,
    /// Do not record those response headers
    pub except_response_headers: Option<Vec<&'static str>>,
}

impl RecordConfig {
    const HOST_HEADER: &'static str = "host";
    const USER_AGENT_HEADER: &'static str = "user-agent";
}

impl Default for RecordConfig {
    fn default() -> Self {
        Self {
            port: None,
            output: None,
            except_request_headers: Some(vec![Self::HOST_HEADER, Self::USER_AGENT_HEADER]),
            except_response_headers: None,
        }
    }
}
