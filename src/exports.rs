use std::os::raw::{c_char, c_int};

use windows::Win32::Foundation::HWND;

use crate::{keyboard, mouse, window};

#[no_mangle]
pub extern "C" fn kb_input(keys: *const c_char) -> c_int {
    let s = unsafe { std::ffi::CStr::from_ptr(keys).to_str().unwrap() };
    match keyboard::input(s) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn kb_wait_keys_up(keys: *const c_char) -> c_int {
    let s = unsafe { std::ffi::CStr::from_ptr(keys).to_str().unwrap() };
    match keyboard::wait_keys_up(s) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn hotkey_register(keys: *const c_char) -> c_int {
    let s = unsafe { std::ffi::CStr::from_ptr(keys).to_str().unwrap() };
    match keyboard::hotkey_register(s) {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn hotkey_unregister(id: c_int) {
    keyboard::hotkey_unregister(id)
}

#[no_mangle]
pub extern "C" fn hotkey_wait() -> c_int {
    match keyboard::hotkey_wait() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_move_to(x: i32, y: i32) -> c_int {
    match mouse::move_to(x, y) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_move_delta(x: c_int, y: c_int) -> c_int {
    match mouse::move_delta(x, y) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_left_click() -> c_int {
    match mouse::left_click() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_left_down() -> c_int {
    match mouse::left_down() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_left_up() -> c_int {
    match mouse::left_up() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_right_click() -> c_int {
    match mouse::right_click() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_middle_click() -> c_int {
    match mouse::middle_click() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_wheel(amount: c_int) -> c_int {
    match mouse::wheel(amount) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_hwheel(amount: c_int) -> c_int {
    match mouse::hwheel(amount) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn mouse_button_press(button: c_int, press: c_int) -> c_int {
    let btn = mouse::MouseButton::from_i32(button);
    let press = mouse::PressType::from_i32(press);

    if btn.is_none() || press.is_none() {
        return -1;
    }

    match mouse::button_press(btn.unwrap(), press.unwrap()) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{}", e);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn window_find(caption: *const c_char, class: *const c_char) -> isize {
    let cap_str = unsafe { std::ffi::CStr::from_ptr(caption).to_str().unwrap() };
    let clz_str: Option<&str> = if class == std::ptr::null() {
        None
    } else {
        Some(unsafe { std::ffi::CStr::from_ptr(class).to_str().unwrap() })
    };

    let hwnd = window::find_window(cap_str, clz_str);
    hwnd.0
}

#[no_mangle]
pub extern "C" fn window_get_rect(hwnd: isize, left: *mut c_int, right: *mut c_int, top: *mut c_int, bottom: *mut c_int) -> c_int {
    match window::get_window_rect(&HWND(hwnd)) {
        Some(rect) => {
            unsafe {
                *left = rect.left;
                *right = rect.right;
                *top = rect.top;
                *bottom = rect.bottom;
            }
            0
        }
        None => {
            -1
        }
    }
}
