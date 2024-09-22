use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked: {}, at {} {}:{}",
            info.message(),
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        println!(
            "Panicked: {}, unknown location",
            info.message(),
        );
    }

    loop {}
}