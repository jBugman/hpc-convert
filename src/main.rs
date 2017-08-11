use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path = "test_data/fun-lang-test.tix";
    read_tix(&path);

    println!("\n");

    let path = "test_data/hpc/language-fun-0.29.1.0-Hg9DdLrIfsTzgrAVPGCMV/Fun.mix";
    read_mix(&path);
}

fn read_tix(path: &str) {
    let data = read_file(path);
    let data = data.trim_left_matches("Tix ");
    let modules = data.split_terminator("TixModule ");
    for module in modules {
        let module = module.trim_right_matches(',');
        println!("{}", module);
    }
}

fn read_mix(path: &str) {
    let data = read_file(path);
    let parts: Vec<&str> = data.splitn(8, ' ').collect();
    let filename = parts[1];
    println!("filename: {}", filename);

    let boxes = parts[7].trim_left_matches("[(").trim_right_matches("])");
    let boxes = boxes.split("),(");
    for b in boxes {
        let pos = b.split(',').nth(0).expect("position");
        println!("{}", pos);
    }
}

fn read_file(path: &str) -> String {
    // TODO: expect
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path,
                                                   why.description()),
        Ok(file) => file
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", path,
                                                   why.description()),
        Ok(_) => (),
    }
    return contents;
}
