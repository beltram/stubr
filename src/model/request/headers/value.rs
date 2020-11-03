use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeaderValue {
    // matches a header value exactly e.g. '"equalTo": "application/json"'
    pub equal_to: Option<String>,
    // should header exact matching be case insensitive
    pub case_insensitive: Option<bool>,
    // should header contain this
    pub contains: Option<String>,
}
