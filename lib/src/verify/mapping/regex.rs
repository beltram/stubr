use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use regex::Regex;
use regex_syntax::ParserBuilder;

pub struct RegexStub(pub Regex);

impl From<RegexStub> for String {
    fn from(RegexStub(regex): RegexStub) -> Self {
        const MAX_REPEAT: u32 = 10;
        let mut rng = XorShiftRng::seed_from_u64(42);
        let mut parser = ParserBuilder::new().unicode(false).build();
        let hir = parser.parse(regex.as_str()).unwrap();
        let gen = rand_regex::Regex::with_hir(hir, MAX_REPEAT).unwrap();
        (&mut rng).sample_iter(&gen)
            .take(1)
            .next().unwrap()
    }
}

#[cfg(test)]
mod verify_regex_tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn should_respect_type_and_length() {
        let regex = "([a-z]{10})";
        let sample: String = RegexStub(Regex::from_str(regex).unwrap()).into();
        assert_eq!(sample.len(), 10);
        assert!(sample.chars().all(|c| c.is_ascii()))
    }
}