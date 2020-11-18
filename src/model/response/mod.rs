use serde::Deserialize;
use wiremock::ResponseTemplate;

use body::BodyDto;
use default::WiremockIsoResponse;
use headers::HttpRespHeadersDto;

use super::stub::StubDto;

mod body;
mod body_file;
mod headers;
mod default;

#[derive(Deserialize, Debug)]
pub struct ResponseDto {
    status: u16,
    #[serde(flatten)]
    body: BodyDto,
    #[serde(flatten)]
    headers: HttpRespHeadersDto,
}

impl From<&StubDto> for ResponseTemplate {
    fn from(stub: &StubDto) -> Self {
        let mut template = ResponseTemplate::new(stub.response.status);
        template = WiremockIsoResponse(stub).add(template);
        template = stub.response.headers.add(template);
        template = stub.response.body.add(template);
        template
    }
}

trait ResponseAppender {
    fn add(&self, resp: ResponseTemplate) -> ResponseTemplate;
}