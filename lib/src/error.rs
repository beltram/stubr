use http_types::url;
use std::num::TryFromIntError;

pub type StubrResult<T> = Result<T, StubrError>;

#[derive(thiserror::Error, Debug)]
pub enum StubrError {
    #[error("Invalid stub '{0:?}'")]
    InvalidStub(std::path::PathBuf),
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
    #[error(transparent)]
    ActixError(#[from] actix_web::Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    ProtobufError(#[from] protobuf::Error),
    #[error(transparent)]
    Protobuf2JsonError(#[from] protobuf_json_mapping::PrintError),
    #[error(transparent)]
    Protobuf2JsonParseError(#[from] protobuf_json_mapping::ParseError),
    #[error(transparent)]
    IntConversionError(#[from] TryFromIntError),
    #[error(transparent)]
    HttpError(#[from] anyhow::Error),
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
}

impl From<StubrError> for handlebars::RenderError {
    fn from(se: StubrError) -> Self {
        match se {
            StubrError::HandlebarsError(e) => e,
            _ => handlebars::RenderError::new("Unknown"),
        }
    }
}
