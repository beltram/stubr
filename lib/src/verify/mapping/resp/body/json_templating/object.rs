use anyhow::anyhow;
use itertools::Itertools;
use serde_json::{Map, Value};

use crate::model::response::{
    template::{
        data::{HandlebarsData, RequestData},
        utils::TemplateExt,
        verify::Predictable,
        HandlebarTemplatable,
    },
    ResponseStub,
};

use super::{
    super::super::{StdResponse, Verifier},
    JsonBodyTemplatingVerifier,
};

pub struct JsonObjectVerifier<'a> {
    pub actual: &'a Map<String, Value>,
    pub expected: &'a Map<String, Value>,
}

impl JsonObjectVerifier<'_> {
    fn to_bytes(&self, value: &Value) -> Option<Vec<u8>> {
        match value {
            Value::String(s) => Some(s.as_bytes().to_vec()),
            Value::Number(n) => Some(n.to_string().as_bytes().to_vec()),
            Value::Null => Some("null".to_string().as_bytes().to_vec()),
            Value::Bool(b) => Some(b.to_string().as_bytes().to_vec()),
            Value::Array(_) => None,
            Value::Object(_) => None,
        }
    }

    fn cast_to_value(&self, raw: &str) -> Value {
        if let Ok(i) = raw.parse::<i32>() {
            Value::from(i)
        } else if let Ok(b) = raw.parse::<bool>() {
            Value::from(b)
        } else if let Ok(f) = raw.parse::<f64>() {
            Value::from(f)
        } else if raw == "null" {
            Value::Null
        } else {
            Value::from(raw)
        }
    }
}

impl<'a> TryFrom<&'a JsonBodyTemplatingVerifier> for JsonObjectVerifier<'a> {
    type Error = anyhow::Error;

    fn try_from(parent: &'a JsonBodyTemplatingVerifier) -> anyhow::Result<Self> {
        parent
            .actual
            .as_object()
            .zip(parent.expected.as_object())
            .map(|(actual, expected)| Self { actual, expected })
            .ok_or_else(|| anyhow!(""))
    }
}

impl Verifier<'_> for JsonObjectVerifier<'_> {
    fn verify(self, stub: &'_ ResponseStub, name: &'_ str, req: &'_ RequestData, resp: &'_ mut StdResponse) {
        let key_diff = self.expected.keys().filter(|k| !self.actual.keys().contains(k)).collect_vec();
        assert!(
            key_diff.is_empty(),
            "Verification failed for stub '{}'. Expected json fields '{:?}' were absent from response body",
            name,
            self.expected
                .iter()
                .filter(|(k, _)| key_diff.contains(k))
                .map(|(k, v)| (k, v.as_str().unwrap_or_default()))
                .collect_vec()
        );
        let actual = self
            .actual
            .iter()
            .filter(|(k, _)| self.expected.keys().contains(k))
            .sorted_by_key(|(k, _)| k.as_str());
        actual
            .zip(self.expected.iter().sorted_by_key(|(k, _)| k.as_str()))
            .for_each(|((_, va), (ke, ve))| {
                if let Some(expected) = ve.as_str().filter(|v| v.has_template_expressions()) {
                    let response = self.to_bytes(va);
                    let data = HandlebarsData {
                        request: req,
                        response: response.as_deref(),
                        is_verify: true,
                        stub_name: Some(name),
                    };
                    stub.body.register(expected, expected);
                    let render = stub.body.render(expected, &data);
                    if expected.is_predictable() {
                        assert_eq!(
                            va,
                            &self.cast_to_value(&render),
                            "Verification failed for stub '{}'. Expected json response body for field '{}' to be '{}' but was '{}'",
                            name,
                            ke,
                            render,
                            va
                        )
                    }
                } else {
                    JsonBodyTemplatingVerifier {
                        actual: va.clone(),
                        expected: ve.clone(),
                    }
                    .verify(stub, name, req, resp)
                }
            })
    }
}
