use serde::Deserialize;
use wiremock::ResponseTemplate;

use body::BodyDto;
use default::WiremockIsoResponse;
use headers::HttpRespHeadersDto;

mod body;
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

impl From<ResponseDto> for ResponseTemplate {
    fn from(resp: ResponseDto) -> Self {
        let mut template = ResponseTemplate::new(resp.status);
        template = resp.body.add(template);
        template = resp.headers.add(template);
        template = WiremockIsoResponse.add(template);
        template
    }
}

trait ResponseAppender {
    fn add(&self, resp: ResponseTemplate) -> ResponseTemplate;
}