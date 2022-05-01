use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_HWHEEL,
        MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP,
        MOUSEEVENTF_MOVE, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL,
        MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP,
    },
    WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, XBUTTON1, XBUTTON2},
};

use crate::errors::{Error, Result};

pub enum MouseButton {
    ButtonLeft = 0,
    ButtonRight = 1,
    ButtonMiddle = 2,
    ButtonX1 = 3,
    ButtonX2 = 4,
}

impl MouseButton {
    pub fn from_i32(i: i32) -> Option<MouseButton> {
        match i {
            0 => Some(MouseButton::ButtonLeft),
            1 => Some(MouseButton::ButtonRight),
            2 => Some(MouseButton::ButtonMiddle),
            3 => Some(MouseButton::ButtonX1),
            4 => Some(MouseButton::ButtonX2),
            _ => None,
        }
    }
}

pub enum PressType {
    PressDown = 1,
    PressUp = 2,
    Click = 3,
}

impl PressType {
    pub fn from_i32(i: i32) -> Option<PressType> {
        match i {
            1 => Some(PressType::PressDown),
            2 => Some(PressType::PressUp),
            3 => Some(PressType::Click),
            _ => None,
        }
    }
}

pub enum MouseInput {
    MoveTo(i32, i32),
    MoveDelta(i32, i32),
    Press(MouseButton, PressType),
    Wheel(i32),
    HWheel(i32),
}

static mut SCREEN_SIZE: (i32, i32) = (0, 0);
static INIT: std::sync::Once = std::sync::Once::new();

fn screen_size() -> (i32, i32) {
    unsafe {
        INIT.call_once(|| {
            SCREEN_SIZE.0 = GetSystemMetrics(SM_CXSCREEN);
            SCREEN_SIZE.1 = GetSystemMetrics(SM_CYSCREEN);
        });
        SCREEN_SIZE
    }
}

pub fn input_mouses(mouse_inputs: &[MouseInput]) -> Result<()> {
    let zeroed: INPUT = unsafe { std::mem::zeroed() };
    let mut inputs = vec![zeroed; mouse_inputs.len()];

    for i in 0..mouse_inputs.len() {
        let mut mouse = &mut inputs[i];
        mouse.r#type = INPUT_MOUSE;
        let mut mi = unsafe { &mut mouse.Anonymous.mi };
        match &mouse_inputs[i] {
            MouseInput::MoveTo(x, y) => {
                let (w, h) = screen_size();
                mi.dx = (*x as i64 * 65535 / w as i64) as _;
                mi.dy = (*y as i64 * 65535 / h as i64) as _;
                mi.dwFlags = MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE;
            }
            MouseInput::MoveDelta(x, y) => {
                mi.dx = *x;
                mi.dy = *y;
                mi.dwFlags = MOUSEEVENTF_MOVE;
            }
            MouseInput::Press(button, press) => match button {
                MouseButton::ButtonLeft => match press {
                    PressType::PressDown => mi.dwFlags = MOUSEEVENTF_LEFTDOWN,
                    PressType::PressUp => mi.dwFlags = MOUSEEVENTF_LEFTUP,
                    PressType::Click => mi.dwFlags = MOUSEEVENTF_LEFTDOWN | MOUSEEVENTF_LEFTUP,
                },
                MouseButton::ButtonRight => match press {
                    PressType::PressDown => mi.dwFlags = MOUSEEVENTF_RIGHTDOWN,
                    PressType::PressUp => mi.dwFlags = MOUSEEVENTF_RIGHTUP,
                    PressType::Click => mi.dwFlags = MOUSEEVENTF_RIGHTDOWN | MOUSEEVENTF_RIGHTUP,
                },
                MouseButton::ButtonMiddle => match press {
                    PressType::PressDown => mi.dwFlags = MOUSEEVENTF_MIDDLEDOWN,
                    PressType::PressUp => mi.dwFlags = MOUSEEVENTF_MIDDLEUP,
                    PressType::Click => mi.dwFlags = MOUSEEVENTF_MIDDLEDOWN | MOUSEEVENTF_MIDDLEUP,
                },
                MouseButton::ButtonX1 => {
                    match press {
                        PressType::PressDown => mi.dwFlags = MOUSEEVENTF_XDOWN,
                        PressType::PressUp => mi.dwFlags = MOUSEEVENTF_XUP,
                        PressType::Click => mi.dwFlags = MOUSEEVENTF_XDOWN | MOUSEEVENTF_XUP,
                    }
                    mi.mouseData = XBUTTON1.0 as _;
                }
                MouseButton::ButtonX2 => {
                    match press {
                        PressType::PressDown => mi.dwFlags = MOUSEEVENTF_XDOWN,
                        PressType::PressUp => mi.dwFlags = MOUSEEVENTF_XUP,
                        PressType::Click => mi.dwFlags = MOUSEEVENTF_XDOWN | MOUSEEVENTF_XUP,
                    }
                    mi.mouseData = XBUTTON2.0 as _;
                }
            },
            MouseInput::Wheel(amount) => {
                mi.mouseData = *amount;
                mi.dwFlags = MOUSEEVENTF_WHEEL;
            }
            MouseInput::HWheel(amount) => {
                mi.mouseData = *amount;
                mi.dwFlags = MOUSEEVENTF_HWHEEL;
            }
        }
    }

    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as _) };
    if sent != inputs.len() as u32 {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "SendInput() failed"
        ));
    }

    Ok(())
}

pub fn move_to(x: i32, y: i32) -> Result<()> {
    input_mouses(&vec![MouseInput::MoveTo(x, y)])
}

pub fn move_delta(x: i32, y: i32) -> Result<()> {
    input_mouses(&vec![MouseInput::MoveDelta(x, y)])
}

pub fn left_click() -> Result<()> {
    input_mouses(&vec![MouseInput::Press(
        MouseButton::ButtonLeft,
        PressType::Click,
    )])
}

pub fn left_down() -> Result<()> {
    input_mouses(&vec![MouseInput::Press(
        MouseButton::ButtonLeft,
        PressType::PressDown,
    )])
}

pub fn left_up() -> Result<()> {
    input_mouses(&vec![MouseInput::Press(
        MouseButton::ButtonLeft,
        PressType::PressUp,
    )])
}

pub fn right_click() -> Result<()> {
    input_mouses(&vec![MouseInput::Press(
        MouseButton::ButtonRight,
        PressType::Click,
    )])
}

pub fn middle_click() -> Result<()> {
    input_mouses(&vec![MouseInput::Press(
        MouseButton::ButtonMiddle,
        PressType::Click,
    )])
}

pub fn wheel(amount: i32) -> Result<()> {
    input_mouses(&vec![MouseInput::Wheel(amount)])
}

pub fn hwheel(amount: i32) -> Result<()> {
    input_mouses(&vec![MouseInput::HWheel(amount)])
}

pub fn button_press(btn: MouseButton, press: PressType) -> Result<()> {
    input_mouses(&vec![MouseInput::Press(btn, press)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to() {
        move_to(0, 0).unwrap();
    }

    #[test]
    fn test_move_delta() {
        move_delta(100, 100).unwrap();
    }

    #[test]
    fn test_left_click() {
        left_click().unwrap();
    }

    #[test]
    fn test_select_square() {
        input_mouses(&vec![
            MouseInput::Press(MouseButton::ButtonLeft, PressType::PressDown),
            MouseInput::MoveDelta(100, 100),
            MouseInput::Press(MouseButton::ButtonLeft, PressType::PressUp),
        ])
        .unwrap()
    }
}
