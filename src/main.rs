use std::{arch::asm, ffi::CStr, mem::transmute};

fn hello() {
    unsafe {
        asm!(
            "push 0x6F6C6C65",
            "add eax, 0x726F7720",
            "add eax, 0x0000646C",
            out("eax") _,
        );
    }
}

fn main() {
    unsafe {
        println!(
            "{}",
            CStr::from_ptr(transmute::<_, *const i8>(hello as fn()).offset(1))
                .to_str()
                .unwrap()
        );
    }
}