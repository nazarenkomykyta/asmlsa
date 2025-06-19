use std::vec;

pub const _ASM_COMMANDS_NOARCH_TYPE: [&str; 10] = 
["mov", "inc", "dec", "add", 
"sub", "mul", "cmp", 
"syscall", "ret", "xor"];
pub const _ASM_COMMANDS_X8_TYPE: [&str; 8] = 
["al", "ah", "bl", "bh", "cl", "ch", "dl", "dh"];
pub const _ASM_COMMANDS_X16_TYPE: [&str; 8] = 
["ax", "bx", "cx", "dx", "sp", "bp", "si", "di"];
pub const _ASM_OPERANDS_X32_TYPE: [&str; 8] = 
["eax", "ebx", "ecx", 
"edx", "esp", "ebp", "esi", "edi"];
pub const _ASM_OPERANDS_X64_TYPE: [&str; 9] = 
["rax", "rbx", "rcx", 
"rdx", "rsi", "rbp", "rsp", "rsi", "rdi"];
pub const _ASM_OPERANDS_R_TYPE: [&str; 8] = 
["r8", "r9", "r10", 
"r11", "r12", "r13", 
"r14", "r15"];
pub const _ASM_COMMANDS_STACK_TYPE: [&str; 2] = 
["push", "pop"];

pub struct _ASM_STRICT_ALONE {
    coop: Option<String>
}

impl _ASM_STRICT_ALONE {
    pub fn new(c: String) -> Self {
        Self { coop: Some(String::from(c)) }
    }
}
#[derive(Debug)]
pub struct _ASM_STRICT_DOUBLE {
    coop: Option<Vec<String>>
}

impl _ASM_STRICT_DOUBLE {
    pub fn new() -> Self {
        Self { coop: Some(vec![String::default(); 2]) }
    }

    pub fn add_command(v: &mut _ASM_STRICT_DOUBLE, c: String) {
        if let Some(j) = v.coop.as_mut() {
            j[0] = c;
        };
    } 

    pub fn add_operand(v: &mut _ASM_STRICT_DOUBLE, o: String) {
        if let Some(m) = v.coop.as_mut() {
            m[1] = o;
        }
    }

    pub fn get_len(v: &mut _ASM_STRICT_DOUBLE) -> usize {
        v.coop.as_ref().unwrap().len()
    }
}
#[derive(Debug)]
pub struct _ASM_STRICT_TRIPLE {
    coop: Option<Vec<String>>
}
impl _ASM_STRICT_TRIPLE {
    pub fn new() -> Self {
        Self { coop: Some(vec![String::default(); 3]) }
    }

    pub fn add_command(v: &mut _ASM_STRICT_TRIPLE, c: String ) {
        if let Some(ref mut k) = v.coop {
            k[0] = c;
        }
    }

    pub fn add_operand(v: &mut _ASM_STRICT_TRIPLE, o: String, index: usize) {
        if let Some(ref mut q) = v.coop {
            if index > 2 {
                panic!("Index cannot be greater then 2!");
            }
            q[index] = o;
        }
    }

    pub fn get_len(v: &mut _ASM_STRICT_TRIPLE) -> usize {
        v.coop.as_ref().unwrap().len()
    }
}

pub struct _ASM_STRICT_PROC_ENDP {
    coop: Option<Vec<String>>
}

impl _ASM_STRICT_PROC_ENDP {
    pub fn new() -> Self { 
        Self { coop: Some(vec![String::default(); 2]) }
    }
}

pub fn ok_test() {
}
