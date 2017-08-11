use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use trim::TrimExt;


#[derive(Debug)]
pub struct Mix {
    pub filename: PathBuf,
    pub tix: Vec<Tick>,
}

pub fn read_mix(path: &Path) -> Mix {
    let data = read_file(path);
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
    return mix;
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


// TODO: copypasted for now
fn read_file(path: &Path) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading the file");
    return contents;
}