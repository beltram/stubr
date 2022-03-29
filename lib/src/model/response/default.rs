use const_format::concatcp;
use http_types::headers::SERVER;
use wiremock::ResponseTemplate;

use super::{JsonStub, ResponseAppender};

const MATCHED_STUB_ID_HEADER: &str = "Matched-Stub-Id";

const STUBR_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVER_HEADER: &str = concatcp!("stubr(", STUBR_VERSION, ")");

pub struct WiremockIsoResponse<'a>(pub &'a JsonStub);

impl ResponseAppender for WiremockIsoResponse<'_> {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        resp = resp.append_header(SERVER, SERVER_HEADER);
        if let Some(uuid) = self.0.uuid.as_deref() {
            resp = resp.append_header(MATCHED_STUB_ID_HEADER, uuid);
        }
        resp
    }
}