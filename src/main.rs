use std::error::Error;
use std::fs::File;
// use std::path::Path;
use std::io::prelude::*;

fn main() {
    let path = "test_data/fun-lang-test.tix";
    println!("{}", path);
    let data = read_tix(&path);
    let data = data.trim_left_matches("Tix ");
    // let brackets: &[_] = &['[', ']'];
    // let data = data.trim_matches(brackets);
    let modules = data.split_terminator("TixModule ");
    for module in modules {
        let module = module.trim_right_matches(',');
        println!("{}", module);
    }
}

fn read_tix(path: &str) -> String {
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
