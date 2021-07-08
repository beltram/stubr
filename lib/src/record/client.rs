#[cfg(feature = "record-isahc")]
use isahc::{config::Configurable, HttpClient as IsahcClient};
#[cfg(feature = "record-reqwest")]
use reqwest::{
    Client as ReqwestClient,
    ClientBuilder as ReqwestClientBuilder,
    Proxy as ReqwestProxy,
};

#[cfg(feature = "record-isahc")]
pub fn isahc_client<T: Into<String>>(uri: T) -> IsahcClient {
    IsahcClient::builder()
        .proxy(uri.into().as_str().parse().ok())
        .build()
        .expect("Failed building isahc recording client")
}

#[cfg(feature = "record-reqwest")]
pub fn reqwest_client<T: Into<String>>(uri: T) -> ReqwestClient {
    ReqwestClientBuilder::new()
        .proxy(ReqwestProxy::http(uri.into()).expect("Failed building reqwest proxy"))
        .build()
        .expect("Failed building reqwest recording client")
}