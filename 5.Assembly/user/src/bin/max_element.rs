#![no_std]
#![no_main]

use core::arch::global_asm;
use user_lib::println;

extern crate user_lib;

global_asm!(include_str!("../../../riscv.s"));

extern "C" {
    fn __max_element(arr: *const u32, len: i64) -> u32;
}

#[no_mangle]
fn main() -> i32 {
    let arr = [1, 2, 3, 4, 5];
    println!("Max element: {}", unsafe { __max_element(arr.as_ptr(), arr.len() as i64) });
    0
}
