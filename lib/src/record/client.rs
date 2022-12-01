#[cfg(feature = "record-isahc")]
use isahc::config::Configurable;

#[cfg(feature = "record-isahc")]
#[allow(dead_code)]
pub fn isahc_client<T: Into<String>>(uri: T) -> isahc::HttpClient {
    isahc::HttpClient::builder()
        .proxy(uri.into().as_str().parse().ok())
        .build()
        .expect("Failed building isahc recording client")
}

#[cfg(feature = "record-reqwest")]
pub fn reqwest_client<T: Into<String>>(uri: T) -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .proxy(reqwest::Proxy::http(uri.into()).expect("Failed building reqwest proxy"))
        .build()
        .expect("Failed building reqwest recording client")
}
