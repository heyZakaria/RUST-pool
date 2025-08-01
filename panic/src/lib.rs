use std::fs::File;

pub fn open_file(s: &str) -> File {
    let f = File::open(s).unwrap();

    f
}