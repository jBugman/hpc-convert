use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    let tix = read_tix(path);
    println!("{:?}", tix);

    println!("\n");

    let path = &tix.last().expect("last tix").filename;
    // let path = &tix[1].filename;
    let path = Path::new("test_data/hpc").join(path.as_path());
    println!("{:?}", path);
    read_mix(&path);
}

#[derive(Debug)]
struct Tix {
    filename: PathBuf,
    tix:      Vec<String>, // TODO: ints
}

fn read_tix(path: &Path) -> Vec<Tix> {
    let data = read_file(path);
    let data = data.trim_left_matches("Tix ")
                   .trim_matches(|c| c == '[' || c == ']'); // TODO: Refactor out

    let mut txs: Vec<Tix> = Vec::new();

    let modules = data.split("TixModule ").skip(1);
    for module in modules {
        println!("{}", module);
        let parts: Vec<&str> = module.splitn(4, ' ').collect();

        let f = parts[0].trim_matches('"');  // FIXME: something funky with duplicating paths
        let f = PathBuf::from(f).with_extension("mix");

        let ticks = parts[3].trim_right_matches(',')
                            .trim_matches(|c| c == '[' || c == ']');
        let ticks: Vec<String> = ticks.split(',')
                                      .map(|s| s.to_string())
                                      .collect();
        let tx = Tix{
            filename: f,
            tix: ticks,
        };
        println!("{:?}", tx);
        txs.push(tx);
    }
    return txs;
}

#[derive(Debug)]
struct Pos {
    // line: u32, // TODO: ints
    // col:  u32,
    line: String,
    col:  String,
}

#[allow(dead_code)]
fn read_mix(path: &Path) {
    let data = read_file(path);
    let parts: Vec<&str> = data.splitn(8, ' ').collect();

    let filename = parts[1].trim_matches('"');
    let filename = Path::new(filename);
    println!("filename: {:?}", filename);

    let boxes = parts[7].trim_matches(|c| "[()]".contains(c));
    let boxes = boxes.split("),(");
    for b in boxes {
        let location = b.split(',').nth(0).expect("no position");
        let location: Vec<String> = location.split(|c| c == '-' || c == ':')
                               .map(|s| s.to_string())
                                   .collect();
        let start = Pos{
            line: location[0].clone(), // TODO: Is clone the only way?
            col: location[1].clone(),
        };
        let end = Pos{
            line: location[2].clone(),
            col: location[3].clone(),
        };
        println!("{:?} - {:?}", start, end);
    }
}


// TODO: how (should I?) to return &str?
fn read_file(path: &Path) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading the file");
    return contents;
}
