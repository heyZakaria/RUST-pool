use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind };
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    match OpenOptions::new().read(true).write(true).append(true).create(true).open(path) {
        Ok(mut file) => {
            let x =file.write_all(content.as_bytes());
            match x {
                        Ok(()) => () ,
                        Err(_) => panic!(),
                    }
        }
        Err(e) =>
            match e.kind() {
                ErrorKind::NotFound => {
                    let mut f = File::create(path).expect("Errr");
                    let x = f.write_all(content.as_bytes());
                    match x {
                        Ok(()) => () ,
                        Err(_) => panic!(),
                    }
                    
                }
                _ => panic!(),
            }
    }
}
 

