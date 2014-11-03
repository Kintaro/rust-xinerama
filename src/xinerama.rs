#![allow(non_uppercase_statics)]
#![allow(non_camel_case_types)]

use libc::*;
use xlib::Display;
use xlib::Window;
use xlib::Drawable;
use xlib::XID;

#[repr(C)]
pub struct XineramaScreenInfo {
   screen_number: c_int,
   x_org : c_short,
   y_org : c_short,
   width : c_short,
   height : c_short
}

#[repr(C)]
pub struct XPanoramiXInfo {
    window: Window,
    screen: c_int,
    State: c_int,   
    width: c_int,    
    height: c_int,    
    ScreenCount: c_int,
    eventMask: XID      
}

#[link(name="Xinerama")]
extern {
    pub fn XPanoramiXGetState(dpy: *mut Display, drawable: Drawable, panoramix_info: *mut XPanoramiXInfo) -> c_int;
    pub fn XPanoramiXGetScreenCount(dpy: *mut Display, drawable: Drawable, panoramix_info: *mut XPanoramiXInfo) -> c_int;
    pub fn XPanoramiXGetScreenSize(dpy: *mut Display, drawable: Drawable, screen_num: c_int, panoramix_info: *mut XPanoramiXInfo) -> c_int;
    pub fn XineramaQueryExtension(dpy: *mut Display, event: *mut c_int, error: *mut c_int) -> c_int;
    pub fn XineramaQueryVersion(dpy: *mut Display, major: *mut c_int, minor: *mut c_int) -> c_int;
	pub fn XineramaIsActive(dpy: *mut Display) -> c_int;
	pub fn XineramaQueryScreens(dpy: *mut Display, number: *mut c_int) -> *const XineramaScreenInfo;
}
