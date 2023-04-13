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
#[no_mangle]
pub static mut namespace: libc::c_int = 0;
// A recursive function to find nth catalan number
#[no_mangle]
pub unsafe extern "C" fn catalan(mut n: libc::c_uint) -> libc::c_ulong {
    // Base case
    if n <= 1 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int as libc::c_ulong;
    }
    // catalan(n) is sum of
    // catalan(i)*catalan(n-i-1)
    let mut res: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_uint) < n {
        res = res.wrapping_add(
            catalan(i as libc::c_uint).wrapping_mul(catalan(
                n.wrapping_sub(i as libc::c_uint)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            )),
        );
        i += 1
    }
    return res;
}
// Driver code
unsafe fn main_0() -> libc::c_int {
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
