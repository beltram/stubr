use http_types::url;

pub type StubrResult<T> = Result<T, StubrError>;

#[derive(thiserror::Error, Debug)]
pub enum StubrError {
    #[error("Invalid stub '{0:?}'")]
    InvalidStub(std::path::PathBuf),
    #[error(transparent)]
    UrlError(#[from] url::ParseError),
    #[error(transparent)]
    RegexSyntaxError(#[from] regex_syntax::Error),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    RandRegexError(#[from] rand_regex::Error),
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
    MissingProtobufFile,
}

impl From<StubrError> for handlebars::RenderError {
    fn from(se: StubrError) -> Self {
        match se {
            StubrError::HandlebarsError(e) => e,
            _ => handlebars::RenderError::new("Unknown"),
        }
    }
}
