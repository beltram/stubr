use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryValue {
    // matches a query value exactly e.g. '"equalTo": "42"'
    pub equal_to: Option<Value>,
    // should query exact matching be case insensitive
    pub case_insensitive: Option<bool>,
    // should query contain this
    pub contains: Option<String>,
}
