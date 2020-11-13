use serde::Deserialize;
use wiremock::ResponseTemplate;

use body::BodyDto;

mod body;

#[derive(Deserialize, Debug)]
pub struct ResponseDto {
    pub status: u16,
    #[serde(flatten)]
    body: BodyDto,
}

impl From<ResponseDto> for ResponseTemplate {
    fn from(resp: ResponseDto) -> Self {
        let mut template = ResponseTemplate::new(resp.status);
        template = resp.body.add_to_response(template);
        template
    }
}