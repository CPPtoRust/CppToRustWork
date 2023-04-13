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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
// A simple C program for traversal of a linked list
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub data: libc::c_int,
    pub next: *mut Node,
}
// This function prints contents of linked list starting from
// the given node
#[no_mangle]
pub unsafe extern "C" fn printList(mut n: *mut Node) {
    while !n.is_null() {
        printf(b" %d \x00" as *const u8 as *const libc::c_char, (*n).data);
        n = (*n).next
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut head: *mut Node = 0 as *mut Node;
    let mut second: *mut Node = 0 as *mut Node;
    let mut third: *mut Node = 0 as *mut Node;
    // allocate 3 nodes in the heap
    head = malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node; // assign data in first node
    second = malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node; // Link first node with second
    third = malloc(::std::mem::size_of::<Node>() as libc::c_ulong) as *mut Node; // assign data to second node
    (*head).data = 1 as libc::c_int; // assign data to third node
    (*head).next = second;
    (*second).data = 2 as libc::c_int;
    (*second).next = third;
    (*third).data = 3 as libc::c_int;
    (*third).next = 0 as *mut Node;
    printList(head);
    return 0 as libc::c_int;
}
#[main]
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
