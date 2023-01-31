use crate::wiremock::ResponseTemplate;
use const_format::concatcp;
use http_types::headers::SERVER;

use super::ResponseAppender;

const MATCHED_STUB_ID_HEADER: &str = "Matched-Stub-Id";

const STUBR_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVER_HEADER: &str = concatcp!("stubr(", STUBR_VERSION, ")");

pub struct WiremockIsoResponse<'a>(pub Option<&'a str>);

impl ResponseAppender for WiremockIsoResponse<'_> {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        resp = resp.append_header(SERVER, SERVER_HEADER);
        if let Some(uuid) = self.0 {
            resp = resp.append_header(MATCHED_STUB_ID_HEADER, uuid);
        }
        resp
    }
}
