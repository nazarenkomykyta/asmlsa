use std::{fs::{self, File, Metadata}, io::{Read, Write}};

pub fn _READ_FILE(f: String) -> String {
    let y = fs::read_to_string(f);
    y.unwrap()
}

pub fn _FILE_SIZE(f: String) -> u64 {
    fs::symlink_metadata(f).unwrap().len()
}
