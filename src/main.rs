use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    let tix = read_tix(path);
    // println!("{:?}", tix);

    // println!("\n");

    // let path = &tix.last().expect("last tix").filename;
    // let path = &tix[1].filename;
    // let path = Path::new("test_data/hpc").join(path.as_path());
    // println!("{:?}", path);
    // let mix = read_mix(&path);
    // println!("{:?}", mix);
    combine(&tix.last().unwrap());
}

#[derive(Debug)]
struct Tix {
    filename: PathBuf,
    tix: Vec<u32>,
}

fn read_tix(path: &Path) -> Vec<Tix> {
    let data = read_file(path);
    let data = data.trim_left_matches("Tix ")
                   .trim_matches(|c| c == '[' || c == ']'); // TODO: Refactor out

    let mut txs: Vec<Tix> = Vec::new();

    let modules = data.split("TixModule ").skip(1);
    for module in modules {
        let parts: Vec<&str> = module.splitn(4, ' ').collect();

        let name = parts[0].trim_matches('"');

        let ticks = parts[3].trim_right_matches(',')
                            .trim_matches(|c| c == '[' || c == ']');
        let ticks = ticks.split(',')
                         .map(|s| s.parse().unwrap());
        let tx = Tix{
            filename: PathBuf::from(name.to_owned() + ".mix"),
            tix: ticks.collect(),
        };
        txs.push(tx);
    }
    return txs;
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

#[derive(Debug)]
struct Tick {
    start: Pos,
    end: Pos,
}

#[derive(Debug)]
struct Mix {
    filename: PathBuf,
    tix: Vec<Tick>,
}

#[allow(dead_code)]
fn read_mix(path: &Path) -> Mix {
    let data = read_file(path);
    let parts: Vec<&str> = data.splitn(8, ' ').collect();

    let filename = parts[1].trim_matches('"');

    let mut mix = Mix {
        filename: PathBuf::from(filename),
        tix: Vec::new(),
    };

    let boxes = parts[7].trim_matches(|c| "[()]".contains(c));
    let boxes = boxes.split("),(");

    for b in boxes {
        let location = b.split(',').nth(0).unwrap();
        let location: Vec<&str> = location.split('-').collect();

        let tick = Tick {
            start: Pos::from(location[0]),
            end: Pos::from(location[1]),
        };
        // println!("{:?}", tick);
        mix.tix.push(tick);
    }
    return mix;
}

fn combine(t: &Tix) {
    let path = Path::new("test_data/hpc").join(t.filename.as_path());
    println!("filepath: {:?}", path);
    println!("tix: {:?}", t.tix);
    let mix = read_mix(&path);
    println!("mix: {:?}", mix);
    assert!(t.tix.len() == mix.tix.len());

    for it in t.tix.iter().zip(mix.tix.iter()) {
        let (t, m) = it;
        println!("{}:{:?} {}", mix.filename.to_str().unwrap(), m, t);
    }
}

// TODO: how (should I?) to return &str?
fn read_file(path: &Path) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading the file");
    return contents;
}
