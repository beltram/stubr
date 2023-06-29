use std::{
    fs::OpenOptions,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use request::RequestStub;
use response::{
    template::{HandlebarTemplatable, StubTemplate},
    ResponseStub,
};

use crate::error::{StubrError, StubrResult};
use crate::wiremock_rs::{Mock, MockBuilder, Respond, ResponseTemplate};
use crate::Config;

#[cfg(feature = "grpc")]
pub mod grpc;
pub mod request;
pub mod response;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonStub {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub uuid: Option<String>,
    #[serde(skip_serializing)]
    pub priority: Option<u8>,
    #[serde(skip_serializing)]
    pub expect: Option<u32>,
    #[serde(rename = "request")]
    pub http_request: Option<RequestStub>,
    #[serde(rename = "response")]
    pub http_response: Option<ResponseStub>,
    #[cfg(feature = "grpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_request: Option<grpc::request::GrpcRequestStub>,
    #[cfg(feature = "grpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_response: Option<grpc::response::GrpcResponseStub>,
    /// Protobuf message descriptor file
    #[cfg(feature = "grpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto_file: Option<PathBuf>,
}

impl JsonStub {
    pub const DEFAULT_PRIORITY: u8 = 5;

    fn is_http(&self) -> bool {
        self.http_request.is_some() || self.http_response.is_some()
    }

    #[cfg(feature = "grpc")]
    fn is_grpc(&self) -> bool {
        self.grpc_request.is_some() || self.grpc_response.is_some()
    }

    #[cfg(feature = "grpc")]
    pub fn proto_file(&self) -> Option<&PathBuf> {
        self.proto_file.as_ref().filter(|f| f.exists())
    }

    pub(crate) fn try_creating_from(self, config: &Config, file: &std::path::Path) -> StubrResult<Mock> {
        let expect = self.expect;
        if self.is_http() {
            let req = self.http_request.clone().unwrap_or_default();
            let mut mock = MockBuilder::try_from(&req)?.respond_with(self.into_respond(config)?);
            if let (true, Some(expect)) = (config.verify, expect) {
                mock = mock.expect(expect as u64);
            }
            Ok(mock)
        } else {
            #[cfg(feature = "grpc")]
            {
                if self.is_grpc() {
                    let req = self.grpc_request.clone().unwrap_or_default();
                    let mut mock =
                        grpc::request::GrpcRequestStub::try_new(&req, self.proto_file())?.respond_with(self.into_respond(config)?);
                    if let (true, Some(expect)) = (config.verify, expect) {
                        mock = mock.expect(expect as u64);
                    }
                    return Ok(mock);
                }
            }
            Err(StubrError::InvalidStub(file.to_path_buf()))
        }
    }

    pub fn into_respond<'a>(self, config: &Config) -> StubrResult<impl Respond + 'a> {
        #[cfg(not(feature = "grpc"))]
        {
            let respond = if let Some(resp) = self.http_response.clone() {
                self.http_respond(resp, config)
            } else if self.http_request.is_some() {
                self.default_http_respond(config)
            } else {
                Self::fallback_respond()
            };
            Ok(respond)
        }
        #[cfg(feature = "grpc")]
        {
            let respond = if let Some(resp) = self.http_response.clone() {
                self.http_respond(resp, config)
            } else if self.http_request.is_some() {
                self.default_http_respond(config)
            } else if let Some((proto_file, resp)) = Some(self.proto_file.clone()).zip(self.grpc_response.clone()) {
                Self::grpc_respond(proto_file, resp)?
            } else if self.grpc_request.is_some() {
                Self::default_grpc_respond()
            } else {
                Self::fallback_respond()
            };
            Ok(respond)
        }
    }

    fn http_respond(&self, resp: ResponseStub, config: &Config) -> StubTemplate {
        use crate::model::response::ResponseAppender as _;

        let mut template = ResponseTemplate::new(resp.status());
        template = response::default::WiremockIsoResponse(self.uuid.as_deref()).add(template);
        template = response::delay::Delay(resp.fixed_delay_milliseconds, &resp.delay_distribution, config).add(template);
        if resp.requires_response_templating() {
            resp.headers.register_template();
            resp.body.register_template();
            StubTemplate {
                template,
                response: Some(resp),
                requires_templating: true,
                ..Default::default()
            }
        } else {
            template = resp.headers.add(template);
            template = resp.body.add(template);
            StubTemplate {
                template,
                response: Some(resp),
                requires_templating: false,
                ..Default::default()
            }
        }
    }

    fn default_http_respond(&self, config: &Config) -> StubTemplate {
        use crate::model::response::ResponseAppender as _;

        let resp = &ResponseStub::default();
        let mut template = ResponseTemplate::new(resp.status());
        template = response::default::WiremockIsoResponse(self.uuid.as_deref()).add(template);
        template = response::delay::Delay(resp.fixed_delay_milliseconds, &resp.delay_distribution, config).add(template);
        StubTemplate {
            template,
            requires_templating: false,
            ..Default::default()
        }
    }

    #[cfg(feature = "grpc")]
    fn grpc_respond(proto_file: Option<PathBuf>, resp: grpc::response::GrpcResponseStub) -> StubrResult<StubTemplate> {
        let message_descriptor = resp
            .body
            .as_ref()
            .map(|_| resp.message_descriptor(proto_file.as_ref()))
            .transpose()?;
        if resp.requires_response_templating() {
            let template = ResponseTemplate::new_grpc(resp.status());
            resp.register_template();
            Ok(StubTemplate {
                template,
                md: message_descriptor,
                grpc_response: Some(resp),
                requires_templating: true,
                ..Default::default()
            })
        } else {
            let mut template = ResponseTemplate::new_grpc(resp.status());
            template = resp.register(template, proto_file.as_ref())?;
            Ok(StubTemplate {
                template,
                md: message_descriptor,
                grpc_response: Some(resp),
                requires_templating: false,
                ..Default::default()
            })
        }
    }

    #[cfg(feature = "grpc")]
    fn default_grpc_respond() -> StubTemplate {
        let template = ResponseTemplate::new_grpc(0);
        StubTemplate {
            template,
            requires_templating: false,
            ..Default::default()
        }
    }

    fn fallback_respond() -> StubTemplate {
        StubTemplate {
            template: ResponseTemplate::new(200),
            ..Default::default()
        }
    }
}

impl TryFrom<&PathBuf> for JsonStub {
    type Error = StubrError;

    fn try_from(maybe_stub: &PathBuf) -> StubrResult<Self> {
        let file = OpenOptions::new().read(true).open(maybe_stub)?;
        Ok(serde_json::from_reader(file)?)
    }
}

impl Default for JsonStub {
    fn default() -> Self {
        Self {
            id: Option::default(),
            uuid: Option::default(),
            priority: Some(Self::DEFAULT_PRIORITY),
            expect: Option::default(),
            http_request: Option::default(),
            http_response: Option::default(),
            #[cfg(feature = "grpc")]
            grpc_request: Option::default(),
            #[cfg(feature = "grpc")]
            grpc_response: Option::default(),
            #[cfg(feature = "grpc")]
            proto_file: Option::default(),
        }
    }
}

impl Hash for JsonStub {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(it) = self.id.as_ref() {
            it.hash(state);
        }
        if let Some(it) = self.uuid.as_ref() {
            it.hash(state);
        }
        if let Some(it) = self.priority.as_ref() {
            it.hash(state);
        }
        self.http_request.hash(state);
        self.http_response.hash(state);
    }
}
