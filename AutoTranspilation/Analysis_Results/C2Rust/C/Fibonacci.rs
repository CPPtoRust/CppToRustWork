#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(main, register_tool)]
extern "C" {
    #[no_mangle]
    fn getchar() -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
//Fibonacci Series using Recursion
#[no_mangle]
pub unsafe extern "C" fn fib(mut n: libc::c_int) -> libc::c_int {
    if n <= 1 as libc::c_int {
        return n;
    }
    return fib(n - 1 as libc::c_int) + fib(n - 2 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 9 as libc::c_int;
    printf(b"%d\x00" as *const u8 as *const libc::c_char, fib(n));
    getchar();
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
