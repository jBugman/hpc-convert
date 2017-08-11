use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    read_tix(path);

    println!("\n");

    let path = Path::new("test_data/hpc/language-fun-0.29.1.0-Hg9DdLrIfsTzgrAVPGCMV/Fun.mix");
    read_mix(path);
}

fn read_tix(path: &Path) {
    let data = read_file(path);
    let data = data.trim_left_matches("Tix ");
    let modules = data.split_terminator("TixModule ");
    for module in modules {
        let module = module.trim_right_matches(',');
        println!("{}", module);
    }
}

fn read_mix(path: &Path) {
    let data = read_file(path);
    let parts: Vec<&str> = data.splitn(8, ' ').collect();

    let filename = Path::new(parts[1].trim_matches('"'));
    println!("filename: {:?}", filename);

    let boxes = parts[7].trim_left_matches("[(").trim_right_matches("])");
    let boxes = boxes.split("),(");
    for b in boxes {
        let location = b.split(',').nth(0).expect("no position");
        let location: Vec<&str> = location.split('-').collect();
        println!("{:?}", location);
    }
}

fn read_file(path: &Path) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error reading the file");
    return contents;
}
