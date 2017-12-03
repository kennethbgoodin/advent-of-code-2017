use std::fs::File;
use std::io::prelude::*;
use std::path::{Path};

pub fn read_input<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).expect("no file of given name found");
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    contents
}

