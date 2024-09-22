use crate::{main, syscall::process::sys_exit};

#[no_mangle]
#[link_section = ".text.entry"]
extern "C" fn _start() -> ! {
    clear_bss();
    sys_exit(main());

    unreachable!("Unreachable in _start")
}

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    for i in sbss as usize..ebss as usize {
        unsafe {
            (i as *mut u8).write_volatile(0);
        }
    }
}