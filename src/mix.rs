extern crate file;

use std::fmt;
use std::path::Path;
use std::path::PathBuf;

use trim::TrimExt;


#[derive(Debug)]
pub struct Mix {
    pub filename: PathBuf,
    pub tix: Vec<Tick>,
}

pub fn read_mix(path: &Path) -> Mix {
    let data = file::get_text(path).unwrap();
    let parts: Vec<&str> = data.splitn(8, ' ').collect();

    let filename = parts[1].trim_matches('"');

    let mut mix = Mix {
        filename: PathBuf::from(filename),
        tix: Vec::new(),
    };

    let boxes = parts[7].trim_brackets().trim_parens();
    let boxes = boxes.split("),(");

    for b in boxes {
        let location = b.split(',').nth(0).unwrap();
        let location: Vec<&str> = location.split('-').collect();

        let tick = Tick {
            start: Pos::from(location[0]),
            end: Pos::from(location[1]),
        };
        mix.tix.push(tick);
    }
    mix
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


#[derive(Debug)]
struct Pos {
    line: u32,
    col: u32,
}

impl Pos {
    // TODO: return Result
    fn from(s: &str) -> Pos {
        let parts: Vec<&str> = s.split(':').collect();
        Pos {
            line: parts[0].parse().unwrap(),
            col: parts[1].parse().unwrap(),
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.line, self.col)
    }
}
