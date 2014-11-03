#![allow(non_uppercase_statics)]
#![allow(non_camel_case_types)]

use libc::*;
use xlib::Display;

pub struct XineramaScreenInfo {
   screen_number: c_int,
   x_org : c_short,
   y_org : c_short,
   width : c_short,
   height : c_short
}

#[link(name="Xinerama")]
extern {
	pub fn XineramaIsActive(dpy: *mut Display) -> c_int;
	pub fn XineramaQueryScreens(dpy: *mut Display, number: *mut c_int) -> *const XineramaScreenInfo;
}
