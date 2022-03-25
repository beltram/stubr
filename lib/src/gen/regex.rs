use std::str::FromStr;

use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use regex::Regex;
use regex_syntax::ParserBuilder;

/// Generates random data given a regex
pub struct RegexRndGenerator(pub Regex);

impl RegexRndGenerator {
    pub fn try_generate(self) -> anyhow::Result<String> {
        const MAX_REPEAT: u32 = 10;
        let mut rng = XorShiftRng::seed_from_u64(42);
        let mut parser = ParserBuilder::new().unicode(false).build();
        let hir = parser.parse(self.0.as_str())?;
        let gen = rand_regex::Regex::with_hir(hir, MAX_REPEAT)?;
        (&mut rng).sample_iter(&gen)
            .take(1)
            .next()
            .ok_or_else(|| anyhow::Error::msg(format!("Failed generating random string from regex '{}'", self.0.as_str())))
    }
}

impl TryFrom<&str> for RegexRndGenerator {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Regex::from_str(value)?))
    }
}

#[cfg(test)]
mod verify_regex_tests {
    use super::*;

    #[test]
    fn should_respect_type_and_length() {
        let regex = "([a-z]{10})";
        let sample: String = RegexRndGenerator::try_from(regex).unwrap().try_generate().unwrap();
        assert_eq!(sample.len(), 10);
        assert!(sample.chars().all(|c| c.is_ascii()))
    }
}