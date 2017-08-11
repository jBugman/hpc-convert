pub trait TrimExt {
    fn trim_brackets(&self) -> &str;

    fn trim_parens(&self) -> &str;
}

impl TrimExt for str {
    fn trim_brackets(&self) -> &str {
        self.trim_matches(|c| c == '[' || c == ']')
    }

    fn trim_parens(&self) -> &str {
        self.trim_matches(|c| c == '(' || c == ')')
    }
}
