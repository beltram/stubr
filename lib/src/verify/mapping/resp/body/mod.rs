use super::{StdResponse, super::{req::StdRequest, super::super::model::response::ResponseStub}, Verifier};

mod json;
mod json_templating;
mod text;
mod text_templating;

pub struct BodyVerifier;

impl Verifier<'_> for BodyVerifier {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ mut StdRequest, resp: &'_ mut StdResponse) {
        json::JsonBodyVerifier.verify(stub, name, req, resp);
        text::TextBodyVerifier.verify(stub, name, req, resp);
    }
}
