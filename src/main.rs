extern crate libc;

use libc::{ c_void, c_char, size_t };
use std::ffi::{ CStr, CString };

#[link(name = "hello", kind = "static")]
extern "C" {
  fn hello();
}

fn main() {
  unsafe{
    hello();
  }
}