use itertools::Itertools;
use rand::Rng;

pub struct ContainsGenerator;

impl ContainsGenerator {
    const OFFSET: usize = 5;
    const CHARSET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    pub fn generate_string_containing(value: String) -> String {
        format!("{}{}{}{}{}", value, Self::rand_str(Self::OFFSET), value, Self::rand_str(Self::OFFSET), value)
    }

    fn rand_str(len: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..Self::CHARSET.len());
                Self::CHARSET[idx] as char
            })
            .collect::<String>()
    }

    pub fn generate_number_containing(value: i64) -> String {
        let random = rand::thread_rng().gen::<i64>().to_string();
        let value = value.to_string();
        let value_chars = value.chars().filter(char::is_ascii_digit).collect_vec();
        let random_chars = random.chars().filter(char::is_ascii_digit).collect_vec();
        let (_, end) = random_chars.split_at(value_chars.len());
        value + end.iter().collect::<String>().as_str()
    }
}

#[cfg(test)]
mod contains_generator_tests {
    use super::*;

    mod string {
        use super::*;

        #[test]
        fn should_generate_string_containing() {
            let value = String::from("abcd");
            let rand_str = ContainsGenerator::generate_string_containing(value.clone());
            assert!(rand_str.contains(value.as_str()));
        }

        #[test]
        fn should_have_correct_size() {
            let value = String::from("abcd");
            let rand_str = ContainsGenerator::generate_string_containing(value.clone());
            assert_eq!(rand_str.len(), value.len() * 3 + ContainsGenerator::OFFSET * 2);
        }
    }

    mod number {
        use super::*;

        #[test]
        fn should_generate_number_starting_with() {
            let rand_num = ContainsGenerator::generate_number_containing(42);
            assert!(rand_num.starts_with("42"));
        }

        #[test]
        fn should_preserve_negative_sign() {
            let rand_num = ContainsGenerator::generate_number_containing(-42);
            assert!(rand_num.starts_with("-42"));
        }

        #[test]
        fn should_not_truncate_input() {
            let rand_num = ContainsGenerator::generate_number_containing(i64::MAX);
            assert_eq!(rand_num, i64::MAX.to_string());
            let rand_num = ContainsGenerator::generate_number_containing(i64::MIN);
            assert_eq!(rand_num, i64::MIN.to_string());
        }

        #[test]
        fn should_have_correct_size() {
            let rand_num = ContainsGenerator::generate_number_containing(42);
            assert!(rand_num.len() <= i64::MAX.to_string().chars().count());
        }
    }
}