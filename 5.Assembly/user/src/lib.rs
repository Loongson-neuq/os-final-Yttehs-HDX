#![feature(
    linkage,
)]

#![no_std]

// module begin
pub mod syscall;
pub mod console;
mod lang_item;
mod boot;
// module end

#[no_mangle]
#[linkage = "weak"]
pub fn main() -> i32 {
    panic!("Default main should not be called")
}
