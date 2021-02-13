use serde::Deserialize;
use wiremock::ResponseTemplate;

use body::BodyDto;
use default::WiremockIsoResponse;
use delay::Delay;
use headers::HttpRespHeadersDto;

use super::stub::StubDto;

mod body;
mod body_file;
mod headers;
mod default;
mod delay;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDto {
    /// HTTP response status
    status: u16,
    /// delay in milliseconds to apply to the response
    fixed_delay_milliseconds: Option<u64>,
    /// HTTP response body
    #[serde(flatten)]
    body: BodyDto,
    /// HTTP response headers
    #[serde(flatten)]
    headers: HttpRespHeadersDto,
}

impl From<&StubDto> for ResponseTemplate {
    fn from(stub: &StubDto) -> Self {
        let mut template = ResponseTemplate::new(stub.response.status);
        template = WiremockIsoResponse(stub).add(template);
        template = stub.response.headers.add(template);
        template = stub.response.body.add(template);
        template = Delay(stub).add(template);
        template
    }
}

trait ResponseAppender {
    fn add(&self, resp: ResponseTemplate) -> ResponseTemplate;
}