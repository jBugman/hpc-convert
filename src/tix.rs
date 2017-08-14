extern crate file;

use std::path::Path;
use std::path::PathBuf;

use trim::TrimExt;


#[derive(Debug)]
pub struct Tix {
    pub filename: PathBuf,
    pub tix: Vec<u32>,
}

pub fn from_file(path: &Path) -> Vec<Tix> {
    let data = file::get_text(path).unwrap();
    let data = data.trim_left_matches("Tix ").trim_brackets();

    let mut txs: Vec<Tix> = Vec::new();

    let modules = data.split("TixModule ").skip(1);
    for module in modules {
        let parts: Vec<&str> = module.splitn(4, ' ').collect();

        let name = parts[0].trim_matches('"');

        let ticks = parts[3]
            .trim_right_matches(',')
            .trim_brackets()
            .split(',')
            .map(|s| s.parse().unwrap());
        let tx = Tix {
            filename: PathBuf::from(name.to_owned() + ".mix"),
            tix: ticks.collect(),
        };
        txs.push(tx);
    }
    txs
}
