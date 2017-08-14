#![feature(pattern)]

mod util;
mod mix;
mod tix;
mod errors;

use std::path::Path;
use std::fmt::Write;

use tix::Tix;
use errors::Error;

fn main() {
    let base_dir = Path::new("test_data/hpc");
    let tix_path = Path::new("test_data/fun-lang-test.tix");

    match convert_to_codecov(&tix_path, &base_dir) {
        Ok(result) => print!("{}", result),
        Err(err) => eprintln!("{:?}", err),
    }
}

fn convert_to_codecov(tix_path: &Path, base_dir: &Path) -> Result<String, Error> {
    let tixes = tix::from_file(tix_path)?;
    let mut res = String::new();
    writeln!(&mut res, "{}", "mode: atomic")?;
    for tix in tixes.iter() {
        write!(&mut res, "{}", combine_tix(tix, base_dir)?)?;
    }
    Ok(res)
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
