pub mod any;
pub mod base64;
pub mod datetime;
pub mod json_path;
pub mod numbers;
pub mod size;
pub mod string;
pub mod string_replace;
pub mod trim;
pub mod url_encode;
pub mod verify;

trait HelperExt {
    fn get_str_hash(&self, key: &str) -> Option<&str>;
    fn get_first_str_value(&self) -> Option<&str>;
}

impl HelperExt for handlebars::Helper<'_, '_> {
    fn get_str_hash(&self, key: &str) -> Option<&str> {
        self.hash_get(key)?.relative_path().map(String::escape_single_quotes)
    }

    fn get_first_str_value(&self) -> Option<&str> {
        self.param(0)?.value().as_str()
    }
}

pub trait ValueExt {
    const QUOTE: char = '\'';

    fn escape_single_quotes(&self) -> &str;
}

impl ValueExt for String {
    fn escape_single_quotes(&self) -> &str {
        self.trim_start_matches(Self::QUOTE).trim_end_matches(Self::QUOTE)
    }
}

impl ValueExt for str {
    fn escape_single_quotes(&self) -> &str {
        self.trim_start_matches(Self::QUOTE).trim_end_matches(Self::QUOTE)
    }
}
