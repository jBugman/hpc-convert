#![feature(pattern)]

mod util;
mod mix;
mod tix;
mod errors;

use std::path::Path;

use tix::Tix;
use errors::Error;

fn main() {
    let path = Path::new("test_data/fun-lang-test.tix");
    let tix = tix::from_file(path);

    let base_dir = Path::new("test_data/hpc");
    let t = tix.unwrap();
    let t = t.last().unwrap();
    let result = combine_tix(t, base_dir).unwrap();
    // println!("mode: atomic");
    println!("{}", result);
}

fn combine_tix(t: &Tix, base_dir: &Path) -> Result<String, Error> {
    use std::fmt::Write;

    let path = base_dir.join(t.filename.as_path());
    let mix = mix::from_file(&path)?;

    if t.tix.len() != mix.tix.len() {
        return Err(Error::DisparityError);
    }

    let mut res = String::new();
    for it in t.tix.iter().zip(mix.tix.iter()) {
        let (t, m) = it;
        writeln!(&mut res, "{}:{} 1 {}", mix.filename.to_str().unwrap(), m, t)?;
    }
    Ok(res)
}
