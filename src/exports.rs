use std::os::raw::{c_char, c_int};

use windows::Win32::Foundation::HWND;

use crate::{keyboard, mouse, window};

/// Keyboard input
///
/// # Arguments
///
/// * `keys` - a sequence of strings represent the keys, support following types.
///
/// - single key stroke: "abc" represents type key a, key b, key c.
/// - uppercase key stroke: "ABC" represents press down shift key, type key a, key b, key c and release shift key.
/// - key combination: "<ctrl+c>" represents press down ctrl key, type key c, release ctrl key.
/// - press down: "<ctrl+>" represents press down ctrl key. warning: "<shift+>" will not work.
/// - release: "<ctrl->" represents release ctrl key.
/// - virtual key code: "<13>" represents type enter key.
/// - escaped chars:
///     - `<`: use `<lt>` instead
///     - `>`: use `<gt>` instead
///     - `-`: use `<minus>` instead
///     - `+`: use `<plus>` instead
///
/// key names table:
///
/// | Name                | Decimal | Hex  | Description             |
/// | ------------------- | ------- | ---- | ----------------------- |
/// | backspace           | 8       | 08   | Backspace               |
/// | bs                  | 8       | 08   | Backspace               |
/// | tab                 | 9       | 09   | TAB                     |
/// | clear               | 12      | 0C   | Clear key               |
/// | enter               | 13      | 0D   | ENTER                   |
/// | return              | 13      | 0D   | ENTER                   |
/// | shift               | 16      | 10   | SHIFT key               |
/// | ctrl                | 17      | 11   | CTRL key                |
/// | alt                 | 18      | 12   | ALT key                 |
/// | pause               | 19      | 13   | PAUSE                   |
/// | caps                | 20      | 14   | CAPS LOCK               |
/// | kana                | 21      | 15   | IME Kana mode           |
/// | hangul              | 21      | 15   | IME Hangul mode         |
/// | ime_on              | 22      | 16   | IME On                  |
/// | junja               | 23      | 17   | IME Junja mode          |
/// | final               | 24      | 18   | IME final mode          |
/// | hanja               | 25      | 19   | IME Hanja mode          |
/// | kanji               | 25      | 19   | IME Kanji mode          |
/// | ime_off             | 26      | 1A   | IME Off                 |
/// | esc                 | 27      | 1B   | ESC                     |
/// | convert             | 28      | 1C   | IME convert             |
/// | nonconvert          | 29      | 1D   | IME nonconvert          |
/// | accept              | 30      | 1E   | IME accept              |
/// | modechange          | 31      | 1F   | IME mode change request |
/// | space               | 32      | 20   | SPACE                   |
/// | pageup              | 33      | 21   | PAGE UP                 |
/// | pagedown            | 34      | 22   | PAGE DOWN               |
/// | end                 | 35      | 23   | END                     |
/// | home                | 36      | 24   | HOME                    |
/// | left                | 37      | 25   | LEFT ARROW              |
/// | up                  | 38      | 26   | UP ARROW                |
/// | right               | 39      | 27   | RIGHT ARROW             |
/// | down                | 40      | 28   | DOWN ARROW              |
/// | select              | 41      | 29   | SELECT                  |
/// | print               | 42      | 2A   | PRINT                   |
/// | execute             | 43      | 2B   | EXECUTE                 |
/// | printscreen         | 44      | 2C   | PRINT SCREEN            |
/// | ps                  | 44      | 2C   | PRINT SCREEN            |
/// | insert              | 45      | 2D   | INS                     |
/// | ins                 | 45      | 2D   | INS                     |
/// | delete              | 46      | 2E   | DEL                     |
/// | del                 | 46      | 2E   | DEL                     |
/// | help                | 47      | 2F   | HELP                    |
/// | lwin                | 91      | 5B   | Left Windows key        |
/// | rwin                | 92      | 5C   | Right Windows key       |
/// | apps                | 93      | 5D   | Applications key        |
/// | sleep               | 95      | 5F   | Computer Sleep key      |
/// | numpad0             | 96      | 60   | Numeric keypad 0        |
/// | numpad1             | 97      | 61   | Numeric keypad 1        |
/// | numpad2             | 98      | 62   | Numeric keypad 2        |
/// | numpad3             | 99      | 63   | Numeric keypad 3        |
/// | numpad4             | 100     | 64   | Numeric keypad 4        |
/// | numpad5             | 101     | 65   | Numeric keypad 5        |
/// | numpad6             | 102     | 66   | Numeric keypad 6        |
/// | numpad7             | 103     | 67   | Numeric keypad 7        |
/// | numpad8             | 104     | 68   | Numeric keypad 8        |
/// | numpad9             | 105     | 69   | Numeric keypad 9        |
/// | multiply            | 106     | 6A   | Keypad *                |
/// | add                 | 107     | 6B   | Keypad +                |
/// | separator           | 108     | 6C   | Keypad Separator        |
/// | subtract            | 109     | 6D   | Keypad -                |
/// | decimal             | 110     | 6E   | Keypad .                |
/// | divide              | 111     | 6F   | Keypad /                |
/// | f1                  | 112     | 70   | F1 key                  |
/// | f2                  | 113     | 71   | F2 key                  |
/// | f3                  | 114     | 72   | F3 key                  |
/// | f4                  | 115     | 73   | F4 key                  |
/// | f5                  | 116     | 74   | F5 key                  |
/// | f6                  | 117     | 75   | F6 key                  |
/// | f7                  | 118     | 76   | F7 key                  |
/// | f8                  | 119     | 77   | F8 key                  |
/// | f9                  | 120     | 78   | F9 key                  |
/// | f10                 | 121     | 79   | F10 key                 |
/// | f11                 | 122     | 7A   | F11 key                 |
/// | f12                 | 123     | 7B   | F12 key                 |
/// | f13                 | 124     | 7C   | F13 key                 |
/// | f14                 | 125     | 7D   | F14 key                 |
/// | f15                 | 126     | 7E   | F15 key                 |
/// | f16                 | 127     | 7F   | F16 key                 |
/// | f17                 | 128     | 80   | F17 key                 |
/// | f18                 | 129     | 81   | F18 key                 |
/// | f19                 | 130     | 82   | F19 key                 |
/// | f20                 | 131     | 83   | F20 key                 |
/// | f21                 | 132     | 84   | F21 key                 |
/// | f22                 | 133     | 85   | F22 key                 |
/// | f23                 | 134     | 86   | F23 key                 |
/// | f24                 | 135     | 87   | F24 key                 |
/// | numlock             | 144     | 90   | NUM LOCK key            |
/// | scroll              | 145     | 91   | SCROLL LOCK key         |
/// | lshift              | 160     | A0   | Left SHIFT              |
/// | rshift              | 161     | A1   | Right SHIFT             |
/// | lctrl               | 162     | A2   | Left CTRL               |
/// | rctrl               | 163     | A3   | Right CTRL              |
/// | lalt                | 164     | A4   | Left ALT                |
/// | ralt                | 165     | A5   | Right ALT               |
/// | browser_back        | 166     | A6   | Browser Back key        |
/// | browser_forward     | 167     | A7   | Browser Forward key     |
/// | browser_refresh     | 168     | A8   | Browser Refresh key     |
/// | browser_stop        | 169     | A9   | Browser Stop key        |
/// | browser_search      | 170     | AA   | Browser Search key      |
/// | browser_favorites   | 171     | AB   | Browser Favorites key   |
/// | browser_home        | 172     | AC   | Browser Home key        |
/// | volume_mute         | 173     | AD   | Volume Mute key         |
/// | volume_down         | 174     | AE   | Volume Down key         |
/// | volume_up           | 175     | AF   | Volume Up key           |
/// | media_next_track    | 176     | B0   | Next Track key          |
/// | media_prev_track    | 177     | B1   | Previous Track key      |
/// | media_stop          | 178     | B2   | Stop Media key          |
/// | media_play_pause    | 179     | B3   | Play/Pause Media key    |
/// | launch_mail         | 180     | B4   | Start Mail key          |
/// | launch_media_select | 181     | B5   | Select Media key        |
/// | launch_app1         | 182     | B6   | Start Application 1 key |
/// | launch_app2         | 183     | B7   | Start Application 2 key |
/// | attn                | 246     | F6   | Attn key                |
/// | crsel               | 247     | F7   | CrSel key               |
/// | exsel               | 248     | F8   | ExSel key               |
/// | ereof               | 249     | F9   | Erase EOF key           |
/// | play                | 250     | FA   | Play key                |
/// | zoom                | 251     | FB   | Zoom key                |
/// | pa1                 | 253     | FD   | PA1 key                 |
/// | oem_clear           | 254     | FE   | Clear key               |
///
/// # Returns
///
/// 0 on success, otherwise -1
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

/// Wait for release these keys
///
/// * `keys` - keys to wait for release, see `kb_input` documentation
///
/// return 0 on success, -1 on failure
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

/// Register hotkey, return hotkey id
///
/// * `keys` - keys to wait for release, see `kb_input` documentation, must be key combination form (e.g. "<ctrl+a>")
///
/// return 0 on success, -1 on failure
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

/// Unregister hotkey with hotkey id.
///
/// * `id` - hotkey id, which was returned by `hotkey_register`
#[no_mangle]
pub extern "C" fn hotkey_unregister(id: c_int) {
    keyboard::hotkey_unregister(id)
}

/// Wait for hotkey input, when any hotkey pressed, return hotkey id.
///
/// return hotkey id on success, -1 on failure
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

/// Mouse move to an absolute position.
///
/// * `x` - x position
/// * `y` - y position
///
/// return 0 on success, -1 on failure
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

/// Mouse move to relative position.
///
/// * `x` - x delta
/// * `y` - y delta
///
/// return 0 on success, -1 on failure
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

/// Mouse left button click
///
/// return 0 on success, -1 on failure
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

/// Mouse left button press down
///
/// return 0 on success, -1 on failure
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

/// Mouse left button release
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

/// Mouse right button click
///
/// return 0 on success, -1 on failure
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

/// Mouse middle button click
///
/// return 0 on success, -1 on failure
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

/// Mouse wheel scroll by amount
///
/// * `amount` - amount to scroll
///
/// return 0 on success, -1 on failure
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

/// Mouse horizontal wheel scroll by amount
///
/// * `amount` - amount to scroll
///
/// return 0 on success, -1 on failure
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

/// Mouse button action
///
/// * `button`: mouse button, `0` for left, `1` for right, `2` for middle, `3` for x1, `4` for x2
/// * `press`: `1` for press down, `2` for release, `3` for click
///
/// return 0 on success, -1 on failure
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

/// Find window by caption and class name, return handle
///
/// return handle on success, 0 on failure
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

/// Get window rect with handle
///
/// * `hwnd` - window handle
/// * `left` - pointer for left position
/// * `right` - pointer for right position
/// * `top` - pointer for top position
/// * `bottom` - pointer for bottom position
///
/// return 0 on success, -1 on failure, when success, left, right, top, bottom will be set
#[no_mangle]
pub extern "C" fn window_get_rect(
    hwnd: isize,
    left: *mut c_int,
    right: *mut c_int,
    top: *mut c_int,
    bottom: *mut c_int,
) -> c_int {
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
        None => -1,
    }
}
