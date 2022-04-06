use super::{
    StdResponse,
    super::super::super::model::response::{ResponseStub, template::data::RequestData},
    Verifier
};

mod json;
mod json_templating;
mod text;
mod text_templating;

pub struct BodyVerifier;

impl Verifier<'_> for BodyVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ RequestData, resp: &'_ mut StdResponse) {
        json::JsonBodyVerifier.verify(stub, name, req, resp);
        text::TextBodyVerifier.verify(stub, name, req, resp);
    }
}
