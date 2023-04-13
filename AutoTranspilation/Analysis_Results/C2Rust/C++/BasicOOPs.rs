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
// C++ program to demonstrate constructors
#[no_mangle]
pub static mut namespace: libc::c_int = 0;
#[no_mangle]
pub static mut Geeks: libc::c_int = 0;
//Default Constructor
//Parameterized Constructor
unsafe fn main_0() -> libc::c_int {
    // obj1 will call Default Constructor
    // obj2 will call Parameterized Constructor
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
