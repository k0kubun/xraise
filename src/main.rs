extern crate x11;

use std::ffi::CString;
use std::mem::zeroed;
use std::ptr;
use x11::xlib::*;

fn intern_atom(display: *mut Display, name: &str) -> u64 {
    unsafe {
        let cstr = CString::new(name).unwrap();
        XInternAtom(display, cstr.as_ptr(), False)
    }
}

fn get_property(display: *mut Display,
                window: Window,
                prop_name: &str,
                size: *mut u64)
                -> *mut i64 {
    unsafe {
        let mut actual_type: u64 = 0;
        let mut actual_format: i32 = 0;
        let mut n_items: u64 = 0;
        let mut bytes_after: u64 = 0;
        let mut prop: *mut u8 = zeroed();

        let net_client_list = intern_atom(display, prop_name);
        XGetWindowProperty(display,
                           window,
                           net_client_list,
                           0,
                           1024,
                           False,
                           AnyPropertyType as u64,
                           &mut actual_type,
                           &mut actual_format,
                           &mut n_items,
                           &mut bytes_after,
                           &mut prop);

        *size = n_items;
        prop as *mut i64
    }
}

fn get_client_list(display: *mut Display, size: *mut u64) -> *mut Window {
    unsafe {
        let root = XDefaultRootWindow(display);
        let client_list = get_property(display, root, "_NET_CLIENT_LIST", size);
        client_list as *mut Window
    }
}

fn get_pid(display: *mut Display, window: Window) -> i64 {
    unsafe {
        let root = XDefaultRootWindow(display);
        let mut size = 0;
        let prop = get_property(display, window, "_NET_WM_PID", &mut size);
        if size > 0 {
            *prop
        } else {
            panic!("_NET_WM_PID failed")
        }
    }
}

fn main() {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed")
        }

        let mut size = 0;
        let windows = get_client_list(display, &mut size);

        for i in 0..size {
            let window = *windows.offset(i as isize);
            println!("hello {}", get_pid(display, window));
        }

        XCloseDisplay(display);
    }
}
