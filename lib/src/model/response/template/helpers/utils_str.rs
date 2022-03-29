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