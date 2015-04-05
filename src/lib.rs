#![crate_name = "xinerama"]
#![crate_type = "lib"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

extern crate libc;
extern crate xlib;

pub use xinerama::*;
mod xinerama;

