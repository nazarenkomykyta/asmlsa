use std::{fs::{self, File}, io::{Write}};

pub fn _READ_FILE(f: String) -> String {
    let y = fs::read_to_string(f);
    y.unwrap()
}

pub fn _CREATE_FILE(f: String) {
    let s: File = File::create_new(f).unwrap();
}
