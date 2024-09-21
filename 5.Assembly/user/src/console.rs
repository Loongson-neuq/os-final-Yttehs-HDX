use core::fmt::{Arguments, Result, Write};
use crate::syscall::fs::sys_write;

const STDOUT: usize = 1;

// Console begin
pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result {
        sys_write(STDOUT, s.as_bytes());
        Ok(())
    }
}

impl Console {
    pub fn print(args: Arguments) {
        Self.write_fmt(args).unwrap();
    }
}
// Console end

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::Console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::Console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };

    () => {
        print!("\n");
    }
}