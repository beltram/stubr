use http_types::url;
use std::num::TryFromIntError;

pub type StubrResult<T> = Result<T, StubrError>;

#[derive(thiserror::Error, Debug)]
pub enum StubrError {
    #[error("Invalid stub {0:?}")]
    InvalidStub(std::path::PathBuf),
    #[error("Stub {0:?} not found")]
    StubNotFound(std::path::PathBuf),
    #[error("App {0:?} not found")]
    AppNotFound(String),
    #[error("Could not find output directory")]
    OutputDirFound,
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
    #[error(transparent)]
    RegexSyntaxError(#[from] Box<regex_syntax::Error>),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    RandRegexError(#[from] Box<rand_regex::Error>),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    HandlebarsError(#[from] handlebars::RenderError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[cfg(any(feature = "verify-actix", feature = "record-actix"))]
    #[error(transparent)]
    ActixError(#[from] actix_web::Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[cfg(feature = "grpc")]
    #[error(transparent)]
    ProtobufError(#[from] protobuf::Error),
    #[cfg(feature = "grpc")]
    #[error(transparent)]
    Protobuf2JsonError(#[from] protobuf_json_mapping::PrintError),
    #[cfg(feature = "grpc")]
    #[error(transparent)]
    Protobuf2JsonParseError(#[from] protobuf_json_mapping::ParseError),
    #[error(transparent)]
    IntConversionError(#[from] TryFromIntError),
    #[error(transparent)]
    HttpError(#[from] anyhow::Error),
    #[error(transparent)]
    HyperServerError(#[from] HyperError),
    #[error("json path error '{0}'")]
    JsonPathError(String),
    #[error("A request body matcher must contain at least one matcher")]
    NoRequestBodyMatcher,
    #[error("Invalid request body matcher '{0}'")]
    InvalidRequestBodyMatcher(&'static str),
    #[error("Invalid response template '{0}' because {1}")]
    InvalidTemplate(&'static str, &'static str),
    #[error("Internal error which should not bubble up")]
    QuietError,
    #[error("Error while recording because {0}")]
    RecordingError(&'static str),
    #[error("Missing Protobuf file in stub")]
    MissingProtoFile,
    #[error("Could not find protobuf file at path {0:?}")]
    ProtobufFileNotFound(std::path::PathBuf),
    #[error("Provided protobuf message '{0}' not found in file {1:?}")]
    ProtoMessageNotFound(String, std::path::PathBuf),
    #[error("A protobuf 'message' has to be defined in stub")]
    MissingProtoMessage,
    #[error("Unexpected invalid gRPC request")]
    InvalidGrpcRequest,
    #[error("Could not convert file {0:?} name to utf-8 string")]
    FileNameError(std::path::PathBuf),
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),
}

impl From<StubrError> for handlebars::RenderError {
    fn from(se: StubrError) -> Self {
        match se {
            StubrError::HandlebarsError(e) => e,
            _ => handlebars::RenderError::new("Unknown"),
        }
    }
}

/// Dedicated error for building internal hyper server.
/// This has to be Send + Sync
#[derive(thiserror::Error, Debug)]
pub enum HyperError {
    #[error("Internal error")]
    ImplementationError,
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error(transparent)]
    HttpError(#[from] http::Error),
    #[error("Error with http_types")]
    HttpTypesError,
}
