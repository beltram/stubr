use crate::wiremock_rs::ResponseTemplate;
use http_types::headers::SERVER;

use super::ResponseAppender;

const MATCHED_STUB_ID_HEADER: &str = "Matched-Stub-Id";

const STUBR_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVER_HEADER: &str = const_format::concatcp!("stubr(", STUBR_VERSION, ")");

lazy_static! {
    pub(crate) static ref VARY: http_types::headers::HeaderName = http_types::headers::VARY.try_into().unwrap();
    pub(crate) static ref ACCEPT_ENCODING: http_types::headers::HeaderValue = "Accept-Encoding".try_into().unwrap();
    pub(crate) static ref USER_AGENT: http_types::headers::HeaderValue = "User-Agent".try_into().unwrap();
}

pub struct WiremockIsoResponse<'a>(pub Option<&'a str>);

impl ResponseAppender for WiremockIsoResponse<'_> {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        resp = resp.append_header(SERVER, SERVER_HEADER);
        if let Some(uuid) = self.0 {
            resp = resp.append_header(MATCHED_STUB_ID_HEADER, uuid);
        }
        if resp.http_status_code.map(|s| s.is_success()).unwrap_or_default() {
            resp = resp.append_multi_header(VARY.clone(), [ACCEPT_ENCODING.clone(), USER_AGENT.clone()]);
        }
        resp
    }
}
