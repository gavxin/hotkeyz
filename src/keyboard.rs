use std::{
    collections::{HashMap, HashSet},
    thread::sleep,
    time::Duration,
};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        System::DataExchange::GlobalAddAtomA,
        UI::{
            Input::KeyboardAndMouse::{
                GetAsyncKeyState, RegisterHotKey, SendInput, UnregisterHotKey, VkKeyScanW,
                HOT_KEY_MODIFIERS, INPUT, INPUT_KEYBOARD, KEYEVENTF_KEYUP, MOD_ALT, MOD_CONTROL,
                MOD_SHIFT, MOD_WIN, VIRTUAL_KEY,
            },
            WindowsAndMessaging::{GetMessageA, MSG, WM_HOTKEY},
        },
    },
};

use crate::errors::{Error, Result};

pub enum KeyInput {
    KeyDown(u8),
    KeyUp(u8),
}

pub fn input_keys(key_inputs: &[KeyInput]) -> Result<()> {
    let zeroed: INPUT = unsafe { std::mem::zeroed() };
    let mut inputs = vec![zeroed; key_inputs.len()];
    for i in 0..key_inputs.len() {
        inputs[i].r#type = INPUT_KEYBOARD;
        match &key_inputs[i] {
            KeyInput::KeyDown(key) => {
                inputs[i].Anonymous.ki.wVk = VIRTUAL_KEY(*key as u16);
            }
            KeyInput::KeyUp(key) => {
                inputs[i].Anonymous.ki.wVk = VIRTUAL_KEY(*key as u16);
                inputs[i].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;
            }
        }
    }

    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as _) };
    if sent != inputs.len() as u32 {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "SendInput() failed."
        ));
    }
    Ok(())
}

pub fn input(s: &str) -> Result<()> {
    let inputs = parse_str(s)?;
    input_keys(&inputs)
}

pub fn wait_keys_up(s: &str) -> Result<()> {
    let inputs = parse_str(s)?;
    let mut keys: HashSet<u8> = HashSet::new();
    for input in inputs {
        match input {
            KeyInput::KeyDown(key) => keys.insert(key),
            KeyInput::KeyUp(key) => keys.insert(key),
        };
    }
    let mut count = 0i32;
    for vk in keys {
        loop {
            let ret = unsafe { GetAsyncKeyState(vk.into()) };
            if (ret as u16 & 0x8000) == 0 {
                break;
            }
            // key not up
            sleep(Duration::from_millis(1));
            count += 1;
            if count > 30000 {
                return Err("Wait longer than 30 seconds".into());
            }
        }
    }

    Ok(())
}

pub fn hotkey_register(hotkey: &str) -> Result<i32> {
    let inputs = parse_str(hotkey)?;
    let mut state = KeyState::new();

    let mut modifiers: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(0);
    let mut vk = 0u32;

    for input in inputs {
        if let KeyInput::KeyDown(key) = input {
            if is_shift_key(key) {
                if state.shift != KEY_STATE_NOT_PRESSED {
                    bail!("Invalid hotkey");
                }
                modifiers.0 |= MOD_SHIFT.0;
                state.shift = VK_SHIFT;
            } else if is_ctrl_key(key) {
                if state.ctrl != KEY_STATE_NOT_PRESSED {
                    bail!("Invalid hotkey");
                }
                modifiers.0 |= MOD_CONTROL.0;
                state.ctrl = VK_CONTROL;
            } else if is_alt_key(key) {
                if state.alt != KEY_STATE_NOT_PRESSED {
                    bail!("Invalid hotkey");
                }
                modifiers.0 |= MOD_ALT.0;
                state.alt = VK_MENU;
            } else if is_win_key(key) {
                if state.win != KEY_STATE_NOT_PRESSED {
                    bail!("Invalid hotkey");
                }
                modifiers.0 |= MOD_WIN.0;
                state.win = VK_LWIN;
            } else {
                if vk != 0 {
                    bail!("Invalid hotkey");
                }
                vk = key.into();
            }
        } else {
            break;
        }
    }

    if vk == 0 {
        bail!("Invalid hotkey");
    }

    let atom = unsafe { GlobalAddAtomA(PCSTR(hotkey.as_ptr())) };
    if atom == 0 {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "GlobalAddAtomA() failed"
        ));
    }

    let ret = unsafe { RegisterHotKey(HWND(0), atom.into(), modifiers, vk) };
    if !ret.as_bool() {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "RegisterHotKey() failed"
        ));
    }

    Ok(atom.into())
}

pub fn hotkey_unregister(id: i32) {
    unsafe { UnregisterHotKey(HWND(0), id) };
}

pub fn hotkey_wait() -> Result<i32> {
    let mut msg: MSG = unsafe { std::mem::zeroed() };
    while unsafe { GetMessageA(&mut msg, HWND(0), 0, 0) }.as_bool() {
        if msg.message == WM_HOTKEY {
            return Ok(msg.wParam.0 as _);
        }
    }
    bail!("Unknown!");
}

const VK_SHIFT: u8 = 0x10;
const VK_CONTROL: u8 = 0x11;
const VK_MENU: u8 = 0x12;

const VK_LSHIFT: u8 = 0xA0;
const VK_RSHIFT: u8 = 0xA1;
const VK_LCONTROL: u8 = 0xA2;
const VK_RCONTROL: u8 = 0xA3;
const VK_LMENU: u8 = 0xA4;
const VK_RMENU: u8 = 0xA5;
const VK_LWIN: u8 = 0x5B;
const VK_RWIN: u8 = 0x5C;

lazy_static! {
    static ref KEY_NAME_TO_VK: HashMap<&'static str, u8> = HashMap::from([
    ("backspace", 0x08),
    ("bs", 0x08),
    ("tab", 0x09),
    ("clear", 0x0C),
    ("return", 0x0D),
    ("enter", 0x0D),
    ("shift", VK_SHIFT),
    ("ctrl", VK_CONTROL),
    ("alt", VK_MENU),
    ("pause", 0x13),
    ("caps", 0x14),
    ("kana", 0x15),
    ("hangul", 0x15),
    ("ime_on", 0x16),
    ("junja", 0x17),
    ("final", 0x18),
    ("hanja", 0x19),
    ("kanji", 0x19),
    ("ime_off", 0x1A),
    ("esc", 0x1B),
    ("convert", 0x1C),
    ("nonconvert", 0x1D),
    ("accept", 0x1E),
    ("modechange", 0x1F),
    ("space", 0x20),
    ("pageup", 0x21),
    ("pagedown", 0x22),
    ("end", 0x23),
    ("home", 0x24),
    ("left", 0x25),
    ("up", 0x26),
    ("right", 0x27),
    ("down", 0x28),
    ("select", 0x29),
    ("print", 0x2A),
    ("execute", 0x2B),
    ("ps", 0x2C),
    ("printscreen", 0x2C),
    ("insert", 0x2D),
    ("ins", 0x2D),
    ("delete", 0x2E),
    ("del", 0x2E),
    ("help", 0x2F),
    ("win", 0x5B),
    ("lwin", VK_LWIN),
    ("rwin", VK_RWIN),
    ("apps", 0x5D),
    ("sleep", 0x5F),
    ("numpad0", 0x60),
    ("numpad1", 0x61),
    ("numpad2", 0x62),
    ("numpad3", 0x63),
    ("numpad4", 0x64),
    ("numpad5", 0x65),
    ("numpad6", 0x66),
    ("numpad7", 0x67),
    ("numpad8", 0x68),
    ("numpad9", 0x69),
    ("multiply", 0x6A),  // keypad *
    ("add", 0x6B),       // keypad +
    ("separator", 0x6C), // ?
    ("subtract", 0x6D),  // keypad -
    ("decimal", 0x6E),   // keypad .
    ("divide", 0x6F),    // keypad /
    ("f1", 0x70),
    ("f2", 0x71),
    ("f3", 0x72),
    ("f4", 0x73),
    ("f5", 0x74),
    ("f6", 0x75),
    ("f7", 0x76),
    ("f8", 0x77),
    ("f9", 0x78),
    ("f10", 0x79),
    ("f11", 0x7A),
    ("f12", 0x7B),
    ("f13", 0x7C),
    ("f14", 0x7D),
    ("f15", 0x7E),
    ("f16", 0x7F),
    ("f17", 0x80),
    ("f18", 0x81),
    ("f19", 0x82),
    ("f20", 0x83),
    ("f21", 0x84),
    ("f22", 0x85),
    ("f23", 0x86),
    ("f24", 0x87),
    ("numlock", 0x90),
    ("scroll", 0x91),
    ("lshift", VK_LSHIFT),
    ("rshift", VK_RSHIFT),
    ("lctrl", VK_LCONTROL),
    ("rctrl", VK_RCONTROL),
    ("lalt", VK_LMENU),
    ("ralt", VK_RMENU),
    ("browser_back", 0xA6),
    ("browser_forward", 0xA7),
    ("browser_refresh", 0xA8),
    ("browser_stop", 0xA9),
    ("browser_search", 0xAA),
    ("browser_favorites", 0xAB),
    ("browser_home", 0xAC),
    ("volume_mute", 0xAD),
    ("volume_down", 0xAE),
    ("volume_up", 0xAF),
    ("media_next_track", 0xB0),
    ("media_prev_track", 0xB1),
    ("media_stop", 0xB2),
    ("media_play_pause", 0xB3),
    ("launch_mail", 0xB4),
    ("launch_media_select", 0xB5),
    ("launch_app1", 0xB6),
    ("launch_app2", 0xB7),
    ("attn", 0xF6),
    ("crsel", 0xF7),
    ("exsel", 0xF8),
    ("ereof", 0xF9),
    ("play", 0xFA),
    ("zoom", 0xFB),
    ("pa1", 0xFD),
    ("oem_clear", 0xFE),
    ]);

    static ref ESCAPED_KEY_NAME_TO_CHAR: HashMap<&'static str, char> = HashMap::from([
        ("lt", '<'), ("gt", '>'), ("minus", '-'), ("plus", '+')
    ]);
}

const KEY_STATE_NOT_PRESSED: u8 = 0;
struct KeyState {
    // 0 means not pressed, otherwise should be shift virtual key code
    shift: u8,
    ctrl: u8,
    alt: u8,
    win: u8,
}

impl KeyState {
    fn new() -> KeyState {
        KeyState {
            shift: KEY_STATE_NOT_PRESSED,
            ctrl: KEY_STATE_NOT_PRESSED,
            alt: KEY_STATE_NOT_PRESSED,
            win: KEY_STATE_NOT_PRESSED,
        }
    }
}

fn parse_str(s: &str) -> Result<Vec<KeyInput>> {
    let mut result: Vec<KeyInput> = Vec::new();
    let mut in_bracket = false;
    let mut state: KeyState = KeyState::new();
    let mut part = String::new();

    let mut vk_shift_vec: Vec<(u8, bool)> = Vec::new();
    let mut seps: Vec<bool> = Vec::new();

    for c in s.chars() {
        if !in_bracket {
            if c == '<' {
                in_bracket = true;
                vk_shift_vec.clear();
                seps.clear();
                continue;
            }

            let (vk, shift) = parse_char(c)?;
            push_key_down_and_up(vk, shift, &mut result, &mut state)?;
            continue;
        }

        // in bracket
        if c != '>' && c != '-' && c != '+' {
            // normal char
            part.push(c);
            continue;
        }

        if !part.is_empty() {
            let (vk, shift) = parse_part(&part)?;
            part.clear();
            vk_shift_vec.push((vk, shift));
        }

        if c == '+' || c == '-' {
            if seps.len() >= vk_shift_vec.len() {
                return Err(
                    format!("Invalid key string, expected key name before '+' or '-'").into(),
                );
            }
            seps.push(c == '+');
            continue;
        }

        assert_eq!('>', c);
        if vk_shift_vec.is_empty() {
            return Err(format!("Invalid key string, expect key name after '<'").into());
        }

        if seps.is_empty() {
            // one name only, like <pagedown>
            assert_eq!(1, vk_shift_vec.len());
            push_key_down_and_up(
                vk_shift_vec[0].0,
                vk_shift_vec[0].1,
                &mut result,
                &mut state,
            )?;
            in_bracket = false;
            continue;
        }

        if vk_shift_vec.len() == seps.len() {
            // end with '+' or '-'
            for i in 0..seps.len() {
                if seps[i] {
                    push_key_down(vk_shift_vec[i].0, &mut result, &mut state);
                } else {
                    push_key_up(vk_shift_vec[i].0, &mut result, &mut state);
                }
            }
            in_bracket = false;
            continue;
        }

        // key combination, like <ctrl+c>
        assert_eq!(vk_shift_vec.len(), seps.len() + 1);
        for i in 0..seps.len() {
            if !seps[i] {
                return Err("Invalid key string, combination cannot contain '-'".into());
            }
        }

        for i in 0..vk_shift_vec.len() {
            push_key_down(vk_shift_vec[i].0, &mut result, &mut state);
        }

        for i in 0..vk_shift_vec.len() {
            push_key_up(
                vk_shift_vec[vk_shift_vec.len() - 1 - i].0,
                &mut result,
                &mut state,
            );
        }
        in_bracket = false;
    }
    Ok(result)
}

#[allow(dead_code)]
fn mod_keys_up(inputs: &mut Vec<KeyInput>, state: &mut KeyState) -> () {
    if state.win != KEY_STATE_NOT_PRESSED {
        inputs.push(KeyInput::KeyUp(state.win));
        state.win = KEY_STATE_NOT_PRESSED;
    }

    if state.shift != KEY_STATE_NOT_PRESSED {
        inputs.push(KeyInput::KeyUp(state.shift));
        state.shift = KEY_STATE_NOT_PRESSED;
    }

    if state.ctrl != KEY_STATE_NOT_PRESSED {
        inputs.push(KeyInput::KeyUp(state.ctrl));
        state.ctrl = KEY_STATE_NOT_PRESSED;
    }
}

// return vk and shift pressed
fn parse_char(c: char) -> Result<(u8, bool)> {
    let ret = unsafe { VkKeyScanW(c as _) };
    if ret == -1 {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "VkKeyScan() failed"
        ));
    }
    let vk: u8 = (ret & 0xFF) as _;
    let shift: bool = (ret & 0x0100) != 0;
    Ok((vk, shift))
}

fn push_key_down_and_up(
    vk: u8,
    shift: bool,
    inputs: &mut Vec<KeyInput>,
    state: &mut KeyState,
) -> Result<()> {
    if shift && state.shift == KEY_STATE_NOT_PRESSED {
        inputs.push(KeyInput::KeyDown(VK_SHIFT));
        state.shift = VK_SHIFT;
    }

    if !shift && state.shift != KEY_STATE_NOT_PRESSED {
        inputs.push(KeyInput::KeyUp(state.shift));
        state.shift = KEY_STATE_NOT_PRESSED;
    }

    inputs.push(KeyInput::KeyDown(vk));
    inputs.push(KeyInput::KeyUp(vk));
    Ok(())
}

// return vk and shift pressed
fn parse_part(s: &str) -> Result<(u8, bool)> {
    if s.len() == 0 {
        return Err(format!("expected key name").into());
    }

    if KEY_NAME_TO_VK.contains_key(s) {
        return Ok((KEY_NAME_TO_VK[s], false));
    }

    if !ESCAPED_KEY_NAME_TO_CHAR.contains_key(s) && s.len() != 1 {
        return Err(format!("unknown key name {}", s).into());
    }

    let ch: char;

    if ESCAPED_KEY_NAME_TO_CHAR.contains_key(s) {
        ch = ESCAPED_KEY_NAME_TO_CHAR[s];
    } else {
        ch = s.chars().next().unwrap();
    }

    parse_char(ch)
}

fn is_shift_key(vk: u8) -> bool {
    vk == VK_SHIFT || vk == VK_LSHIFT || vk == VK_RSHIFT
}

fn is_ctrl_key(vk: u8) -> bool {
    vk == VK_CONTROL || vk == VK_LCONTROL || vk == VK_RCONTROL
}

fn is_alt_key(vk: u8) -> bool {
    vk == VK_MENU || vk == VK_LMENU || vk == VK_RMENU
}

fn is_win_key(vk: u8) -> bool {
    vk == VK_LWIN || vk == VK_RWIN
}

fn push_key_down(vk: u8, inputs: &mut Vec<KeyInput>, state: &mut KeyState) -> () {
    inputs.push(KeyInput::KeyDown(vk));
    if is_shift_key(vk) {
        state.shift = vk;
    }
    if is_ctrl_key(vk) {
        state.ctrl = vk;
    }
    if is_alt_key(vk) {
        state.alt = vk;
    }
    if is_win_key(vk) {
        state.win = vk;
    }
}

fn push_key_up(vk: u8, inputs: &mut Vec<KeyInput>, state: &mut KeyState) -> () {
    inputs.push(KeyInput::KeyUp(vk));
    if is_shift_key(vk) {
        state.shift = KEY_STATE_NOT_PRESSED;
    }
    if is_ctrl_key(vk) {
        state.ctrl = KEY_STATE_NOT_PRESSED;
    }
    if is_alt_key(vk) {
        state.alt = KEY_STATE_NOT_PRESSED;
    }
    if is_win_key(vk) {
        state.win = KEY_STATE_NOT_PRESSED;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_abc() -> Result<()> {
        println!("Testing start");
        input("aBc")
    }

    #[test]
    fn type_escaped() -> Result<()> {
        println!("Testing start");
        input("<lt>")
    }

    #[test]
    fn show_desktop() -> Result<()> {
        println!("Testing start");
        input("<win+d>")
    }

    #[test]
    fn switch_windows() -> Result<()> {
        println!("Testing start");
        input("<win+>23<win->")
    }

    #[test]
    fn hotkey() {
        let id = hotkey_register("<ctrl+y>").unwrap();
        println!("id {}", id);

        let wait = hotkey_wait().unwrap();
        println!("wait {}", wait);

        hotkey_unregister(id);
    }
}
