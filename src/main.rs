extern crate procrs;
extern crate x11;

use procrs::pid::Pid;
use std::env;
use std::ffi::{CString, CStr};
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

fn get_window_name(display: *mut Display, window: Window) -> Option<String> {
    let mut size = 0;
    unsafe {
        let prop = get_property(display, window, "WM_NAME", &mut size);
        if prop.is_null() {
            None
        } else {
            CStr::from_ptr(prop as *mut i8).to_str().map(|it| it.to_string()).ok()
        }
    }
}

fn get_windows(display: *mut Display) -> Vec<Window> {
    let mut ret = vec![];
    let mut size = 0;

    unsafe {
        let ptr = get_client_list(display, &mut size);

        for i in 0..size {
            let window = *ptr.offset(i as isize);
            ret.push(window);
        }
    }
    ret
}

fn get_pid(display: *mut Display, window: Window) -> i32 {
    unsafe {
        let mut size = 0;
        let prop = get_property(display, window, "_NET_WM_PID", &mut size);
        if size > 0 {
            *prop as i32
        } else {
            panic!("_NET_WM_PID failed")
        }
    }
}

fn send_event(display: *mut Display, window: Window, name: &str) {
    unsafe {
        let data = {
            let mut data = ClientMessageData::new();
            data.set_long(0, 0);
            data.set_long(1, 0);
            data.set_long(2, 0);
            data.set_long(3, 0);
            data.set_long(4, 0);
            data
        };
        let event = XClientMessageEvent {
            type_: ClientMessage,
            serial: 0,
            send_event: True,
            message_type: intern_atom(display, name),
            window: window,
            format: 32,
            data: data,
            display: display,
        };

        XSendEvent(display,
                   XDefaultRootWindow(display),
                   False,
                   SubstructureNotifyMask | SubstructureRedirectMask,
                   &mut XEvent::from(event));
    }
}

fn activate_window(display: *mut Display, window: Window) {
    send_event(display, window, "_NET_ACTIVE_WINDOW");
}

fn get_cmdline(display: *mut Display, window: Window) -> Vec<String> {
    let pid = get_pid(display, window);
    let process = Pid::new(pid).expect("Failed to fetch cmdline!");
    process.cmdline
}

fn match_window_name(display: *mut Display, window: Window, name: &String) -> bool {
    if let Some(actual) = get_window_name(display, window) {
        if actual.len() > name.len() {
            let end = actual[(actual.len() - name.len())..(actual.len() - 0)].to_string();
            end == *name
        } else {
            actual == *name
        }
    } else {
        false
    }
}

fn main() {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed")
        }

        let windows = get_windows(display);
        if let Some(command) = env::args().nth(1) {
            if let Some(name) = env::args().nth(2) {
                for window in windows {
                    if get_cmdline(display, window)[0] == command &&
                       match_window_name(display, window, &name) {
                        activate_window(display, window);
                        break;
                    }
                }
            } else {
                for window in windows {
                    if get_cmdline(display, window)[0] == command {
                        activate_window(display, window);
                        break;
                    }
                }
            }
        } else {
            for window in windows {
                let cmdline = get_cmdline(display, window);
                if let Some(name) = get_window_name(display, window) {
                    println!("{} {}: '{}'", get_pid(display, window), cmdline[0], name);
                } else {
                    println!("{} {}:", get_pid(display, window), cmdline[0]);
                }
            }
        }

        XCloseDisplay(display);
    }
}
