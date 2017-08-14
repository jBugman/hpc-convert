#![feature(pattern)]

mod util;
mod mix;
mod tix;
mod errors;

use std::path::Path;

use tix::Tix;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    let tix = tix::from_file(path);

    let base_dir = Path::new("test_data/hpc");
    let t = tix.unwrap();
    let t = t.last().unwrap();
    combine_tix(t, base_dir);
}

fn combine_tix(t: &Tix, base_dir: &Path) {
    let path = base_dir.join(t.filename.as_path());
    let mix = mix::from_file(&path).unwrap();
    assert_eq!(t.tix.len(), mix.tix.len());

    println!("mode: atomic");
    for it in t.tix.iter().zip(mix.tix.iter()) {
        let (t, m) = it;
        println!("{}:{} 1 {}", mix.filename.to_str().unwrap(), m, t);
    }
}
