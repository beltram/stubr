use handlebars::{RenderContext, Template, template::TemplateElement};

use super::{
    AnyAlphaNumeric,
    AnyFloat,
    AnyInteger,
    AnyNonBlank,
    AnyNonEmpty,
    AnyNumber,
    AnyRegex,
    AnyUuid,
    AnyBoolean,
    AnyDate,
    AnyDatetime,
    AnyIso8601Datetime,
    AnyTime,
    AnyEmail,
    AnyHostname,
    AnyIp,
    AnyOf,
};

/// Some templates can be made of many elements e.g. '{{anyNonBlankString}}{{anyNonEmptyString}}'.
/// In that setup it is impossible to verify them given any String.
/// Hence the role of this trait is to determine if we can generate assertions from a given template.
pub trait Verifiable {
    fn is_verifiable(&self) -> bool;
}

impl Verifiable for Template {
    fn is_verifiable(&self) -> bool {
        let (rnd, pred): (Vec<_>, Vec<_>) = self.elements.iter().partition(|e| e.is_rnd());
        match rnd.len() {
            0 => true,
            1 => pred.is_empty(),
            _ => false,
        }
    }
}

impl Verifiable for str {
    fn is_verifiable(&self) -> bool {
        Template::compile(self)
            .map(|t| t.is_verifiable())
            .unwrap_or_default()
    }
}

impl Verifiable for RenderContext<'_, '_> {
    fn is_verifiable(&self) -> bool {
        self.get_root_template_name()
            .map(|t| t.is_verifiable())
            .unwrap_or(true)
    }
}

pub trait Predictable {
    const RND_NAMES: [&'static str; 24] = [
        AnyRegex::NAME,
        AnyNonBlank::NAME,
        AnyNonEmpty::NAME,
        AnyAlphaNumeric::NAME,
        AnyNumber::NAME,
        AnyFloat::NAME,
        AnyInteger::U64,
        AnyInteger::I64,
        AnyInteger::U32,
        AnyInteger::I32,
        AnyInteger::U16,
        AnyInteger::I16,
        AnyInteger::U8,
        AnyInteger::I8,
        AnyUuid::NAME,
        AnyBoolean::NAME,
        AnyDate::NAME,
        AnyDatetime::NAME,
        AnyIso8601Datetime::NAME,
        AnyTime::NAME,
        AnyEmail::NAME,
        AnyHostname::NAME,
        AnyIp::NAME,
        AnyOf::NAME,
    ];

    fn is_predictable(&self) -> bool;
    fn is_rnd(&self) -> bool {
        !self.is_predictable()
    }
}

impl Predictable for TemplateElement {
    fn is_predictable(&self) -> bool {
        match self {
            TemplateElement::Expression(e) => {
                e.name.as_name()
                    .map(|n| !Self::RND_NAMES.contains(&n))
                    .unwrap_or_default()
            }
            _ => true
        }
    }
}

impl Predictable for str {
    fn is_predictable(&self) -> bool {
        Template::compile(self)
            .map(|t| t.elements.iter().all(|e| e.is_predictable()))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod verifiable_test {
    use super::*;

    mod verifiable {
        use super::*;

        #[test]
        fn without_template_should_be_verifiable() {
            assert!("hello world".is_verifiable());
        }

        #[test]
        fn one_predictable_template_should_be_verifiable() {
            assert!("{{request.path}}".is_verifiable());
        }

        #[test]
        fn one_predictable_template_should_be_verifiable_when_prefix_suffix() {
            assert!("begin{{request.path}}".is_verifiable());
            assert!("{{request.path}}end".is_verifiable());
        }

        #[test]
        fn many_predictable_template_should_be_verifiable() {
            assert!("{{request.url}}{{request.path}}".is_verifiable());
        }

        #[test]
        fn many_interleaved_predictable_template_should_be_verifiable() {
            assert!("-{{request.url}}-{{request.path}}-".is_verifiable());
        }

        #[test]
        fn one_rnd_template_should_be_verifiable() {
            assert!("{{anyRegex '[0-9]+'}}".is_verifiable());
            assert!("{{anyNonBlankString}}".is_verifiable());
            assert!("{{anyNonEmptyString}}".is_verifiable());
        }

        #[test]
        fn one_rnd_template_should_not_be_verifiable_when_prefix_suffix() {
            assert!(!"begin{{anyRegex '[0-9]+'}}".is_verifiable());
            assert!(!"{{anyRegex '[0-9]+'}}end".is_verifiable());
        }

        #[test]
        fn many_rnd_template_should_not_be_verifiable() {
            assert!(!"{{anyRegex '[0-9]+'}}{{anyNonBlankString}}".is_verifiable());
            assert!(!"{{anyRegex '[0-9]+'}}-{{anyNonBlankString}}".is_verifiable());
        }
    }

    mod predictable {
        use super::*;

        #[test]
        fn should_be_predictable() {
            assert!("{{request.url}}".is_predictable());
        }

        #[test]
        fn should_not_be_predictable() {
            assert!(!"{{anyRegex '[0-9]+'}}".is_predictable());
            assert!(!"{{anyNonBlankString}}".is_predictable());
            assert!(!"{{anyNonEmptyString}}".is_predictable());
        }
    }
}