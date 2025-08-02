use std::path::Path;
use std::io::Read;
use std::fs::File;
use std::io::{ ErrorKind };
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match OpenOptions::new().read(true).write(true).append(true).create(true).open(path) {
        Ok(mut file) => {
            file.write_all(content.as_bytes().expect("Errr"));
        }
        Err(e) =>
            match e.kind() {
                ErrorKind::NotFound => {
                    let mut f = File::create(path).expect("Errr");
                    f.write_all(content.as_bytes().expect("Errr"));
                }
                _ => panic!(),
            }
    }
}
 

