use itertools::Itertools;
use serde_json::Value;

pub trait VerifyDetect {
    fn is_verify(&self) -> bool;
    fn read_response(&self) -> Option<Vec<u8>>;
    fn stub_name(&self) -> &str;
}

impl VerifyDetect for handlebars::Context {
    fn is_verify(&self) -> bool {
        self.data().as_object()
            .and_then(|o| o.get("is_verify"))
            .and_then(Value::as_bool)
            .unwrap_or_default()
    }

    fn read_response(&self) -> Option<Vec<u8>> {
        self.data().as_object()
            ?.get("response")
            ?.as_array()
            .filter(|a| !a.is_empty())
            .map(|b| b.iter().filter_map(|v| v.as_u64()).filter_map(|u| u8::try_from(u).ok()).collect_vec())
    }

    fn stub_name(&self) -> &str {
        self.data().as_object()
            .and_then(|o| o.get("stub_name"))
            .and_then(Value::as_str)
            .unwrap_or_default()
    }
}