use super::syscall;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

pub fn sys_exit(code: i32) -> isize {
    let args = [code as usize, 0, 0];
    syscall(SYSCALL_EXIT, args)
}

pub fn sys_yield() {
    let args = [0, 0, 0];
    syscall(SYSCALL_YIELD, args);
}

pub fn sys_get_time() -> isize {
    let args = [0, 0, 0];
    syscall(SYSCALL_GET_TIME, args)
}