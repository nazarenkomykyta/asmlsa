use crate::asm::*;
use crate::reader::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub struct _BLOCK_TO_ANALYZE_ASM_DOUBLE {
    v: Arc<Mutex<Vec<_ASM_STRICT_DOUBLE>>>
}

impl _BLOCK_TO_ANALYZE_ASM_DOUBLE {
    pub fn new() -> Self {
        let v = Arc::new(Mutex::new(Vec::new()));
        Self { v: v }
    }

    pub fn add_block(v: &mut MutexGuard<'_, Vec<_ASM_STRICT_DOUBLE>>, b: _ASM_STRICT_DOUBLE) { 
        v.push(b);
    }
}

pub struct _BLOCK_TO_ANALYZE_ASM_TRIPLE {
    v: Arc<Mutex<Vec<_ASM_STRICT_TRIPLE>>>
}

impl _BLOCK_TO_ANALYZE_ASM_TRIPLE {
    pub fn new() -> Self {
        let v: Arc<Mutex<Vec<_ASM_STRICT_TRIPLE>>> = Arc::new(Mutex::new(Vec::new()));
        Self { v: v }
    }

    pub fn add_block(v: &mut MutexGuard<'_, Vec<_ASM_STRICT_TRIPLE>>, b: _ASM_STRICT_TRIPLE) {
        v.push(b);
    }
}

pub struct _BLOCK_TO_ANALYZE_ASM_PROC_ENDP {
    v: Arc<Mutex<Vec<_ASM_STRICT_PROC_ENDP>>>
}

impl _BLOCK_TO_ANALYZE_ASM_PROC_ENDP {
    pub fn new() -> Self {
        let v= Arc::new(Mutex::new(Vec::new()));
        Self { v: v }
    }

    pub fn add_block(v: &mut MutexGuard<'_, Vec<_ASM_STRICT_PROC_ENDP>>, b: _ASM_STRICT_PROC_ENDP) {
        v.push(b);
    }
}

pub fn _ASM_CUT_LINE_SLICED_SPLIT(l: &str) -> Vec<&str>{
    let y: Vec<&str> = l
        .split(',')
        .flat_map(|idk| idk.split_whitespace())
        .collect();
    // println!("{:?}", y); 
    y 

}

pub fn _ASM_SLICED_IS_NUMERIC_END(l: &Vec<&str>) -> bool {
    let o = l[l.len() - 1];
    let mut or = false;
    if o.parse::<i128>().is_ok() {
        or = true;
    }
    or
} 



pub fn _BR_ASM_COMPARE_BLOCK(inout: String) {
    let buffer_bytes = _READ_FILE(inout);
    let coll: Vec<&str> = buffer_bytes.split('\n').collect();
    let mut h: Vec<String> = Vec::new();
    for flex in 0..coll.len() {
        h.push(coll[flex].trim().to_string());
    }
    for plex in 0..h.len() {
        let mut verifed_finalized: Vec<&str> = Vec::new();
        let ioc = &h[plex];
        println!("IOC RN IS -> {}", ioc);
        let fl = _ASM_CUT_LINE_SLICED_SPLIT(&ioc.as_str());
        for t in 0.._ASM_COMMANDS_NOARCH_TYPE.len() {
            match fl {
                _ if fl.contains(&_ASM_COMMANDS_NOARCH_TYPE[t]) => {
                    println!("Found command -> {}", _ASM_COMMANDS_NOARCH_TYPE[t]);
                    verifed_finalized.push(&_ASM_COMMANDS_NOARCH_TYPE[t]);
                }
                _ => {

                }
            }
        }
        for z in 0.._ASM_COMMANDS_STACK_TYPE.len()  {
            match fl {
                _ if fl.contains(&_ASM_COMMANDS_STACK_TYPE[z]) => {
                    verifed_finalized.push(&_ASM_COMMANDS_STACK_TYPE[z]);
                    if _ASM_SLICED_IS_NUMERIC_END(&fl) {
                        verifed_finalized.push(fl[fl.len() - 1]);
                    }
                }
                _ => {

                }
            }
        }
        for g in 0.._ASM_OPERANDS_X8_TYPE.len() {
            match fl {
                _ if fl.contains(&_ASM_OPERANDS_X8_TYPE[g]) => {
                    println!("Found X8 bit operand -> {}", _ASM_OPERANDS_X8_TYPE[g]);
                    verifed_finalized.push(&_ASM_OPERANDS_X8_TYPE[g]);
                    if _ASM_SLICED_IS_NUMERIC_END(&fl) {
                        verifed_finalized.push(fl[fl.len() - 1]);
                    }
                }
                _ => {

                }
            }
        }
        for j in 0.. _ASM_OPERANDS_X16_TYPE.len() {
            match fl {
                _ if fl.contains(&_ASM_OPERANDS_X16_TYPE[j]) => {
                    println!("Found X16 bit operand -> {}", _ASM_OPERANDS_X16_TYPE[j]);
                    verifed_finalized.push(&_ASM_OPERANDS_X16_TYPE[j]);
                    if _ASM_SLICED_IS_NUMERIC_END(&fl) {
                        verifed_finalized.push(fl[fl.len() - 1]);
                    }
                }
                _ => {

                }
            }
        }
        for q in 0.._ASM_OPERANDS_X32_TYPE.len() {
            match fl {
                _ if fl.contains(&_ASM_OPERANDS_X32_TYPE[q]) => {
                    println!("Found X32 bit operand -> {}", _ASM_OPERANDS_X32_TYPE[q]);
                    verifed_finalized.push(&_ASM_OPERANDS_X32_TYPE[q]);
                    if _ASM_SLICED_IS_NUMERIC_END(&fl) {
                        verifed_finalized.push(fl[fl.len() - 1])
                    }
                }
                _ => {

                }
            }
        }
        for e in 0.._ASM_OPERANDS_X64_TYPE.len() {
            match fl {
                _ if fl.contains(&_ASM_OPERANDS_X64_TYPE[e]) => {
                    println!("Found X64 bit operand -> {}", _ASM_OPERANDS_X64_TYPE[e]);
                    verifed_finalized.push(&_ASM_OPERANDS_X64_TYPE[e]);
                    if _ASM_SLICED_IS_NUMERIC_END(&fl) {
                        verifed_finalized.push(fl[fl.len() - 1])
                    }
                }
                _ => {

                }
            }
        }
        for b in 0.._ASM_OPERANDS_R_TYPE.len() {
            match fl {
                _ if fl.contains(&_ASM_OPERANDS_R_TYPE[b]) => {
                    println!("Found (R) type operand -> {}", _ASM_OPERANDS_R_TYPE[b]);
                    verifed_finalized.push(&_ASM_OPERANDS_R_TYPE[b]);
                    if _ASM_SLICED_IS_NUMERIC_END(&fl) {
                        verifed_finalized.push(fl[fl.len() - 1])
                    }
                }
                _ => {

                }
            }
        }
        println!("{:?}", verifed_finalized);
        println!("{}", verifed_finalized.len());
        let d = &mut _ASM_STRICT_TRIPLE::new();
        let h = &mut _ASM_STRICT_DOUBLE::new();
        match verifed_finalized.len() {
            3 => {
                println!("{}", _ASM_STRICT_TRIPLE::get_len(d));
                println!("Len of verifedd_finalized is -> {}",  verifed_finalized.len());
                _ASM_STRICT_TRIPLE::add_command(
                    d,
                    verifed_finalized[0].to_string());
                _ASM_STRICT_TRIPLE::add_operand(
                    d, 
                    verifed_finalized[1].to_string(), 
                    1);
                _ASM_STRICT_TRIPLE::add_operand(
                    d, 
                    verifed_finalized[2].to_string(), 
                    2);
                println!("{:?}", d);
            }
            2 => {
                _ASM_STRICT_DOUBLE::add_command(
                    h, 
                    verifed_finalized[0].to_string());
                _ASM_STRICT_DOUBLE::add_operand(
                    h, 
                    verifed_finalized[1].to_string());
                println!("{:?}", h);
            }
            1 => {
                let a = _ASM_STRICT_ALONE::new(verifed_finalized[0].to_string());
            }
            _ => {
                // Ok
            }
        }
    }
}

