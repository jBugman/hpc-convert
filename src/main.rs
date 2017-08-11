use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    read_tix(path);

    println!("\n");

    // spellchecker: disable
    // let path = Path::new("test_data/hpc/language-fun-0.29.1.0-Hg9DdLrIfsTzgrAVPGCMV/Fun.mix");
    // spellchecker: enable
    // read_mix(path);
}

// TODO: return types
fn read_tix(path: &Path) {
    let data = read_file(path);
    let data = data.trim_left_matches("Tix ")
                   .trim_matches(|c| c == '[' || c == ']'); // TODO: refactor out
    // println!("{}", data);

    let modules = data.split("TixModule ").skip(1);
    for module in modules {
        println!("");
        // println!("TixModule\nsrc: {}", module);

        let parts: Vec<&str> = module.splitn(4, ' ').collect();

        let filename = parts[0].trim_matches('"');
        let filename = Path::new(filename).with_extension("mix");
        println!("filename: {:?}", filename);

        let tix = parts[3].trim_right_matches(',')
                          .trim_matches(|c| c == '[' || c == ']');
        let tix: Vec<&str> = tix.split(',').collect(); // TODO: ints?
        println!("tix: {:?}", tix);
    }
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
