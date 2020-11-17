use const_format::concatcp;
use wiremock::ResponseTemplate;

use super::ResponseAppender;

const STUBR_VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVER_HEADER: &str = concatcp!("stubr(", STUBR_VERSION, ")");

pub struct WiremockIsoResponse;

impl ResponseAppender for WiremockIsoResponse {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        resp = resp.append_header("Server", SERVER_HEADER);
        resp
    }
}