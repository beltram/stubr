use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

pub struct StringHelper;

impl HelperDef for StringHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = Self::value(h).unwrap_or_default();
        if Self::is_uppercase(h) {
            out.write(value.to_uppercase().as_str()).unwrap();
        } else if Self::is_lowercase(h) {
            out.write(value.to_lowercase().as_str()).unwrap();
        } else if Self::is_capitalize(h) {
            out.write(Self::capitalize(value).as_str()).unwrap();
        } else if Self::is_decapitalize(h) {
            out.write(Self::decapitalize(value).as_str()).unwrap();
        }
        Ok(())
    }
}

impl StringHelper {
    pub const CAPITALIZE: &'static str = "capitalize";
    pub const DECAPITALIZE: &'static str = "decapitalize";
    pub const UPPER: &'static str = "upper";
    pub const LOWER: &'static str = "lower";

    fn value<'a>(h: &'a Helper) -> Option<&'a str> {
        h.params().get(0)?.value().as_str()
    }
    fn is_capitalize(h: &Helper) -> bool { h.name() == Self::CAPITALIZE }
    fn is_decapitalize(h: &Helper) -> bool { h.name() == Self::DECAPITALIZE }
    fn is_uppercase(h: &Helper) -> bool { h.name() == Self::UPPER }
    fn is_lowercase(h: &Helper) -> bool { h.name() == Self::LOWER }

    fn capitalize(value: &str) -> String {
        value.char_indices()
            .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
            .collect()
    }

    fn decapitalize(value: &str) -> String {
        value.char_indices()
            .map(|(i, c)| if i == 0 { c.to_ascii_lowercase() } else { c })
            .collect()
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
