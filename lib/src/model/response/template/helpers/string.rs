use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::Value;

pub struct StringHelper;

impl StringHelper {
    pub const CAPITALIZE: &'static str = "capitalize";
    pub const DECAPITALIZE: &'static str = "decapitalize";
    pub const UPPER: &'static str = "upper";
    pub const LOWER: &'static str = "lower";

    fn value<'a>(h: &'a Helper) -> Option<&'a str> {
        h.params().get(0)?.value().as_str()
    }

    fn capitalize(value: &str) -> String {
        Self::map_first(value, char::to_ascii_uppercase)
    }

    fn decapitalize(value: &str) -> String {
        Self::map_first(value, char::to_ascii_lowercase)
    }

    fn map_first(value: &str, transform: fn(&char) -> char) -> String {
        value.char_indices().map(|(i, c)| if i == 0 { transform(&c) } else { c }).collect()
    }
}

impl HelperDef for StringHelper {
    fn call_inner<'reg: 'rc, 'rc>(&self, h: &Helper<'reg, 'rc>, _: &'reg Handlebars<'reg>, _: &'rc Context, _: &mut RenderContext<'reg, 'rc>) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        Self::value(h)
            .map(|value| {
                match h.name() {
                    Self::UPPER => value.to_uppercase(),
                    Self::LOWER => value.to_lowercase(),
                    Self::CAPITALIZE => Self::capitalize(value),
                    Self::DECAPITALIZE => Self::decapitalize(value),
                    _ => panic!("Unexpected error")
                }
            })
            .ok_or_else(|| RenderError::new("Invalid string case transform response template"))
            .map(Value::from)
            .map(ScopedJson::from)
    }
}

#[cfg(test)]
mod string_case_tests {
    use super::*;

    mod capitalize {
        use super::*;

        #[test]
        fn should_capitalize_lowercase() {
            assert_eq!(StringHelper::capitalize("john"), String::from("John"))
        }

        #[test]
        fn should_preserve_already_capitalized() {
            assert_eq!(StringHelper::capitalize("John"), String::from("John"))
        }

        #[test]
        fn should_preserve_rest() {
            assert_eq!(StringHelper::capitalize("jOHN"), String::from("JOHN"))
        }

        #[test]
        fn should_preserve_non_ascii() {
            assert_eq!(StringHelper::capitalize("john 42 !/%"), String::from("John 42 !/%"))
        }

        #[test]
        fn should_just_capitalize_first_word() {
            assert_eq!(StringHelper::capitalize("john doe"), String::from("John doe"))
        }
    }

    mod decapitalize {
        use super::*;

        #[test]
        fn should_decapitalize_uppercase() {
            assert_eq!(StringHelper::decapitalize("JOHN"), String::from("jOHN"))
        }

        #[test]
        fn should_preserve_already_decapitalized() {
            assert_eq!(StringHelper::decapitalize("jOHN"), String::from("jOHN"))
        }

        #[test]
        fn should_preserve_rest() {
            assert_eq!(StringHelper::decapitalize("JOHN"), String::from("jOHN"))
        }

        #[test]
        fn should_preserve_non_ascii() {
            assert_eq!(StringHelper::decapitalize("John 42 !/%"), String::from("john 42 !/%"))
        }

        #[test]
        fn should_just_decapitalize_first_word() {
            assert_eq!(StringHelper::decapitalize("John Doe"), String::from("john Doe"))
        }
    }
}
