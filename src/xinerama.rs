use libc::*;
use xlib::Display;
use xlib::Window;
use xlib::Drawable;
use xlib::XID;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XineramaScreenInfo {
   pub screen_number: c_int,
   pub x_org : c_short,
   pub y_org : c_short,
   pub width : c_short,
   pub height : c_short
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XPanoramiXInfo {
    pub window: Window,
    pub screen: c_int,
    pub state: c_int,
    pub width: c_int,
    pub height: c_int,
    pub screen_count: c_int,
    pub event_mask: XID
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
