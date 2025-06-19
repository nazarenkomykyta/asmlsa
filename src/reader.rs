use std::{fs::{File}, io::{Read}};

pub fn _READ_ASM_F(f: String) -> [u8; 2048] {
    let mut d = File::open(f).unwrap();
    let mut buff_r = [0; 2048];
    d.read(&mut buff_r).unwrap();
    buff_r
}


