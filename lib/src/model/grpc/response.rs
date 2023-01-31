use crate::model::{
    grpc::proto::parse_message_descriptor,
    response::{body::BodyStub, template::data::HandlebarsData, template::HandlebarTemplatable},
};
use crate::wiremock::ResponseTemplate;
use protobuf::reflect::MessageDescriptor;
use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrpcResponseStub {
    /// gRPC response status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<super::code::Code>,
    /// response body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
    /// Name of the message definition within protobuf
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Mostly used for enabling response templating
    #[serde(default, skip_serializing)]
    pub transformers: Vec<String>,
}

impl HandlebarTemplatable for GrpcResponseStub {
    fn register_template(&self) {
        BodyStub {
            json_body: self.body.clone(),
            ..Default::default()
        }
        .register_template();
    }

    #[cfg(feature = "grpc")]
    fn render_response_template(
        &self, mut template: ResponseTemplate, data: &HandlebarsData, md: Option<&MessageDescriptor>,
    ) -> ResponseTemplate {
        let delegate = BodyStub {
            json_body: self.body.clone(),
            ..Default::default()
        };
        if let Some(body) = delegate.render_json_body(self.body.as_ref(), data) {
            let bytes = self.to_protobuf(&body, md.unwrap());
            template = template.set_body_bytes(bytes);
        }
        template
    }

    #[cfg(not(feature = "grpc"))]
    fn render_response_template(&self, mut _template: ResponseTemplate, _data: &HandlebarsData) -> ResponseTemplate {
        unimplemented!()
    }
}

impl GrpcResponseStub {
    pub fn status(&self) -> i32 {
        self.status.unwrap_or_default().into()
    }

    pub fn message_descriptor(&self, proto_file: Option<&PathBuf>) -> MessageDescriptor {
        let msg = self
            .message
            .as_ref()
            .expect("A message has to be defined to find the protobuf definition");
        parse_message_descriptor(msg, proto_file.unwrap())
    }

    pub fn register(&self, mut resp: ResponseTemplate, proto_file: Option<&PathBuf>) -> ResponseTemplate {
        if let Some(body) = self.body.as_ref() {
            let md = self.message_descriptor(proto_file);
            resp = resp.set_body_bytes(self.to_protobuf(body, &md));
        }
        resp
    }

    pub fn to_protobuf(&self, body: &serde_json::Value, md: &MessageDescriptor) -> Vec<u8> {
        let json = serde_json::to_string(body).unwrap();
        let message = protobuf_json_mapping::parse_dyn_from_str(md, json.as_str()).unwrap();
        let mut body = message.write_to_bytes_dyn().unwrap();

        let len: u32 = body.len().try_into().unwrap();
        let l = len.to_be_bytes();
        // gRPC response starts with a 0 then the length of the payload on 4 bytes
        let mut buf = vec![0u8, l[0], l[1], l[2], l[3]];
        buf.append(&mut body);
        buf
    }

    pub(crate) fn requires_response_templating(&self) -> bool {
        const RESPONSE_TEMPLATE: &str = "response-template";
        self.transformers.iter().any(|it| it == RESPONSE_TEMPLATE)
    }
}

impl Hash for GrpcResponseStub {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // we do not need response hash for recorded stub file name
    }
}
