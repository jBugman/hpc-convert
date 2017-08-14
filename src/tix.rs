extern crate file;

use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use util::TrimExt;
use util::NthOk;
use errors::Error;

#[derive(Debug)]
pub struct Tix {
    pub filename: PathBuf,
    pub tix: Vec<u32>,
}

impl FromStr for Tix {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(4, ' ');
        let name = parts.nth_ok(0)?.trim_matches('"');
        let filename = PathBuf::from(name.to_owned() + ".mix");
        let tix = parts
            .nth_ok(2)?
            .trim_right_matches(',')
            .trim_brackets()
            .split(',')
            .flat_map(str::parse)
            .collect();
        Ok(Tix { filename, tix })
    }
}

pub fn from_file(path: &Path) -> Result<Vec<Tix>, Error> {
    let data = file::get_text(path)?;
    let data = data.trim_left_matches("Tix ").trim_brackets();
    let modules = data.split("TixModule ").skip(1);
    let tix = modules.flat_map(str::parse).collect();
    Ok(tix)
}
