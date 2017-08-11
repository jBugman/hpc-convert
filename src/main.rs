use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::prelude::*;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    let tix = read_tix(path);
    println!("{:?}", tix);

    println!("\n");

    // spellchecker: disable
    let path = tix.last().expect("last tix").filename.as_path();
    let path = Path::new("test_data/hpc").join(path);
    println!("{:?}", path)
    // let path = Path::new("test_data/hpc/language-fun-0.29.1.0-Hg9DdLrIfsTzgrAVPGCMV/Fun.mix");
    // spellchecker: enable
    // read_mix(path);
}

#[derive(Debug)]
struct Tix {
    filename: PathBuf,
    tix: Vec<String>, // TODO: ints
}

fn read_tix(path: &Path) -> Vec<Tix> {
    let data = read_file(path);
    let data = data.trim_left_matches("Tix ")
                   .trim_matches(|c| c == '[' || c == ']'); // TODO: refactor out

    let mut txs: Vec<Tix> = Vec::new();

    let modules = data.split("TixModule ").skip(1);
    for module in modules {
        let parts: Vec<&str> = module.splitn(4, ' ').collect();

        let f = parts[0].trim_matches('"');
        let f = Path::new(f).with_extension("mix");

        let ticks = parts[3].trim_right_matches(',')
                            .trim_matches(|c| c == '[' || c == ']');
        let ticks: Vec<String> = ticks.split(',')
                                      .map(|s| s.to_string())
                                      .collect();
        txs.push(Tix{
            filename: f,
            tix: ticks,
        });
    }
    return txs;
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
        let location: Vec<&str> = location.split(|c| c == '-' || c == ':').collect();
        println!("{:?}", location);
    }
}


// TODO: how (should I?) to return &str?
fn read_file(path: &Path) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading the file");
    return contents;
}
