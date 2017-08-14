use std::iter::Iterator;
use std::str::Split;
use std::str::SplitN;
use std::str::pattern::Pattern;
use errors::Error;

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

pub trait NthOk: Iterator {
    fn nth_ok(&mut self, n: usize) -> Result<Self::Item, Error>;
}

impl<'a, P: Pattern<'a>> NthOk for Split<'a, P> {
    fn nth_ok(&mut self, n: usize) -> Result<Self::Item, Error> {
        self.nth(n).ok_or(Error::FormatError)
    }
}

impl<'a, P: Pattern<'a>> NthOk for SplitN<'a, P> {
    fn nth_ok(&mut self, n: usize) -> Result<Self::Item, Error> {
        self.nth(n).ok_or(Error::FormatError)
    }
}
