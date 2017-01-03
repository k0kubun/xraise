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

fn get_pid(display: *mut Display, window: Window) -> i64 {
    unsafe {
        let mut size = 0;
        let prop = get_property(display, window, "_NET_WM_PID", &mut size);
        if size > 0 {
            *prop
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

fn main() {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("XOpenDisplay failed")
        }

        for window in get_windows(display) {
            let pid = get_pid(display, window);
            if pid == 2230 {
                activate_window(display, window);
            }
            println!("pid: {}", pid);
        }

        XCloseDisplay(display);
    }
}
