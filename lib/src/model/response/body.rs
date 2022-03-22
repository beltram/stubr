use std::{
    f64,
    fs::OpenOptions,
    i64,
    io::Read,
    path::PathBuf,
    str::{from_utf8, FromStr},
};
use std::ffi::OsStr;

use handlebars::JsonValue;
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use wiremock::ResponseTemplate;

use super::{
    body_file::BodyFile,
    ResponseAppender,
    template::{data::HandlebarsData, HandlebarTemplatable},
};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BodyStub {
    /// plain text body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// json body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<Value>,
    /// binary Base 64 encoded body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_64_body: Option<String>,
    /// relative path to raw body content
    #[serde(default, skip_serializing, deserialize_with = "deserialize_body_file")]
    pub body_file_name: Option<BodyFile>,
}

impl BodyStub {
    pub const BOOL_IDENTIFIER: &'static str = "[bool]";
    pub const NUMBER_IDENTIFIER: &'static str = "[number]";
    pub const FLOAT_IDENTIFIER: &'static str = "[float]";
    pub const NULL_IDENTIFIER: &'static str = "[null]";
    pub const OBJECT_IDENTIFIER: &'static str = "[object]";
    pub const ARRAY_IDENTIFIER: &'static str = "[array]";

    pub fn register_json_body_template<'a, T>(&self, json_values: T) where T: Iterator<Item=&'a JsonValue> {
        json_values.into_iter().for_each(|value| {
            match value {
                Value::String(s) => self.register(s, s),
                Value::Object(o) => self.register_json_body_template(o.values()),
                Value::Array(a) => self.register_json_body_template(a.iter()),
                _ => {}
            }
        });
    }

    pub fn render_json_body(&self, json_body: Option<&Value>, data: &HandlebarsData) -> Option<Value> {
        json_body
            .and_then(|it| it.as_object().map(|o| self.render_json_obj(o, data)))
            .or_else(|| json_body.and_then(Value::as_array).map(|a| self.render_json_array(a, data)))
    }

    fn render_json_obj(&self, json_body: &Map<String, Value>, data: &HandlebarsData) -> Value {
        let obj = json_body.into_iter()
            .map(|(key, value)| match value {
                Value::String(s) => (key.to_owned(), Self::cast_to_value(self.render(s, data))),
                Value::Object(o) => (key.to_owned(), self.render_json_obj(o, data)),
                Value::Array(a) => (key.to_owned(), self.render_json_array(a, data)),
                _ => (key.to_owned(), value.to_owned())
            });
        Value::from(Map::from_iter(obj))
    }

    fn render_json_array(&self, json_body: &[Value], data: &HandlebarsData) -> Value {
        Value::Array(json_body.iter().map(|value| match value {
            Value::String(s) => Self::cast_to_value(self.render(s, data)),
            Value::Object(o) => self.render_json_obj(o, data),
            Value::Array(a) => self.render_json_array(a, data),
            _ => value.to_owned()
        }).collect_vec())
    }

    fn cast_to_value(raw: String) -> Value {
        let len = raw.len();
        match raw {
            o if o.ends_with(Self::OBJECT_IDENTIFIER) => {
                Value::from_str(&o[..len - Self::OBJECT_IDENTIFIER.len()].to_string())
                    .unwrap_or_default()
            }
            a if a.ends_with(Self::ARRAY_IDENTIFIER) => {
                Value::from_str(&a[..len - Self::ARRAY_IDENTIFIER.len()].to_string())
                    .unwrap_or_default()
            }
            b if b.ends_with(Self::BOOL_IDENTIFIER) => {
                bool::from_str(&b[..len - Self::BOOL_IDENTIFIER.len()].to_string())
                    .map(Value::from)
                    .unwrap_or_default()
            }
            n if n.ends_with(Self::NUMBER_IDENTIFIER) => {
                i64::from_str(&n[..len - Self::NUMBER_IDENTIFIER.len()].to_string())
                    .map(Value::from)
                    .unwrap_or_default()
            }
            f if f.ends_with(Self::FLOAT_IDENTIFIER) => {
                f64::from_str(&f[..len - Self::FLOAT_IDENTIFIER.len()].to_string())
                    .map(Value::from)
                    .unwrap_or_default()
            }
            n if n.ends_with(Self::NULL_IDENTIFIER) => Value::Null,
            _ => Value::from(raw)
        }
    }

    fn binary_body(&self) -> Option<Vec<u8>> {
        self.base_64_body.as_ref()
            .and_then(|b| base64::decode(b).ok())
    }
}

fn deserialize_body_file<'de, D>(path: D) -> Result<Option<BodyFile>, D::Error> where D: Deserializer<'de> {
    let body_file = String::deserialize(path).ok()
        .map(PathBuf::from)
        .map(|path| {
            let path_exists = path.exists();
            let extension = path.extension().and_then(OsStr::to_str).map(str::to_string);
            let content = OpenOptions::new().read(true).open(&path).ok()
                .and_then(|mut file| {
                    let mut buf = vec![];
                    file.read_to_end(&mut buf).map(|_| buf).ok()
                })
                .and_then(|bytes| from_utf8(bytes.as_slice()).map(str::to_string).ok())
                .unwrap_or_default();
            let path = path.to_str().map(str::to_string).unwrap_or_default();
            BodyFile { path_exists, path, extension, content }
        });
    Ok(body_file)
}

impl HandlebarTemplatable for BodyStub {
    fn register_template(&self) {
        if let Some(body) = self.body.as_ref() {
            self.register(body, body);
        } else if let Some(json_body) = self.json_body.as_ref() {
            if let Some(obj) = json_body.as_object() {
                self.register_json_body_template(obj.values());
            } else if let Some(array) = json_body.as_array() {
                self.register_json_body_template(array.iter());
            }
        } else if let Some(body_file) = self.body_file_name.as_ref() {
            self.register(body_file.path.as_str(), &body_file.content);
        }
    }

    fn render_response_template(&self, mut template: ResponseTemplate, data: &HandlebarsData) -> ResponseTemplate {
        if let Some(body) = self.body.as_ref() {
            template = template.set_body_string(self.render(body, data));
        } else if let Some(binary) = self.binary_body() {
            template = template.set_body_bytes(binary);
        } else if let Some(json_body) = self.render_json_body(self.json_body.as_ref(), data) {
            template = template.set_body_json(json_body);
        } else if let Some(body_file) = self.body_file_name.as_ref() {
            let rendered = self.render(body_file.path.as_str(), data);
            template = body_file.render_templated(template, rendered);
        }
        template
    }
}

impl ResponseAppender for BodyStub {
    fn add(&self, mut resp: ResponseTemplate) -> ResponseTemplate {
        if let Some(json) = self.json_body.as_ref() { resp = resp.set_body_json(json) }
        else if let Some(text) = self.body.as_ref() { resp = resp.set_body_string(text) }
        else if let Some(body_file) = self.body_file_name.as_ref() { resp = body_file.add(resp) }
        else if let Some(binary) = self.binary_body() { resp = resp.set_body_bytes(binary) }
        resp
    }
}