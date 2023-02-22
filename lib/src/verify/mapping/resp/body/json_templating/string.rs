use crate::{
    model::response::{
        template::{
            data::{HandlebarsData, RequestData},
            utils::TemplateExt,
            HandlebarTemplatable,
        },
        ResponseStub,
    },
    StubrError, StubrResult,
};

use super::{
    super::super::{StdResponse, Verifier},
    JsonBodyTemplatingVerifier,
};

pub struct JsonStrVerifier<'a> {
    pub actual: &'a str,
    pub expected: &'a str,
}

impl<'a> TryFrom<&'a JsonBodyTemplatingVerifier> for JsonStrVerifier<'a> {
    type Error = StubrError;

    fn try_from(parent: &'a JsonBodyTemplatingVerifier) -> StubrResult<Self> {
        parent
            .actual
            .as_str()
            .zip(parent.expected.as_str())
            .map(|(actual, expected)| Self { actual, expected })
            .ok_or_else(|| StubrError::QuietError)
    }
}

impl Verifier<'_> for JsonStrVerifier<'_> {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ RequestData, _: &'_ mut StdResponse) {
        if self.expected.has_template_expressions() {
            let data = HandlebarsData {
                request: req,
                response: Some(self.actual.as_bytes()),
                is_verify: true,
                stub_name: Some(name),
            };
            stub.body.register(self.expected, self.expected);
            let _ = stub.body.render(self.expected, &data);
        } else {
            assert_eq!(
                self.actual, self.expected,
                "Verification failed for stub '{}'. Expected json field to be '{}' but was '{}'",
                name, self.expected, self.actual
            );
        }
    }
}
