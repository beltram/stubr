#[cfg(feature = "record-isahc")]
use isahc::{config::Configurable, HttpClient};

#[cfg(feature = "record-isahc")]
pub fn isahc_client<T: Into<String>>(uri: T) -> HttpClient {
    HttpClient::builder().proxy(uri.into().as_str().parse().ok()).build().unwrap()
}