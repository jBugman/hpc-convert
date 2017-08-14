extern crate file;

use std::fmt;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use util::TrimExt;
use util::NthOk;
use errors::Error;


#[derive(Debug)]
pub struct Mix {
    pub filename: PathBuf,
    pub tix: Vec<Tick>,
}

pub fn from_file(path: &Path) -> Result<Mix, Error> {
    let data = file::get_text(path)?;
    data.parse()
}

impl FromStr for Mix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(8, ' ');
        let filename = PathBuf::from(parts.nth_ok(1)?.trim_matches('"'));
        let tix = parts
            .nth_ok(5)?
            .trim_brackets()
            .trim_parens()
            .split("),(")
            .flat_map(str::parse) // FIXME: lost errors
            .collect();
        Ok(Mix { filename, tix })
    }
}

#[derive(Debug)]
pub struct Tick {
    start: Pos,
    end: Pos,
}

impl fmt::Display for Tick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.start, self.end)
    }
}

impl FromStr for Tick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let loc = s.split(',').next().ok_or(Error::FormatError)?;
        let mut parts = loc.split('-');
        let start = parts.nth_ok(0)?.parse()?;
        let end = parts.nth_ok(0)?.parse()?;
        Ok(Tick { start, end })
    }
}

#[derive(Debug)]
struct Pos {
    line: u32,
    col: u32,
}

impl FromStr for Pos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');
        let line = parts.nth_ok(0)?.parse()?;
        let col = parts.nth_ok(0)?.parse()?;
        Ok(Pos { line, col })
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.line, self.col)
    }
}
