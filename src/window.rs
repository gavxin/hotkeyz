use windows::{
    core::{PCSTR, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM},
        System::LibraryLoader::GetModuleHandleA,
        UI::{
            Input::{
                GetRawInputData, RegisterRawInputDevices, HRAWINPUT, RAWINPUT, RAWINPUTDEVICE,
                RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT,
            },
            WindowsAndMessaging::{
                CreateWindowExA, DefWindowProcA, DispatchMessageA, FindWindowW, GetMessageA,
                GetWindowRect, RegisterClassExA, TranslateMessage, HMENU, HWND_MESSAGE,
                WINDOW_EX_STYLE, WINDOW_STYLE, WM_INPUT, WNDCLASSEXA,
            },
        },
    },
};

use crate::errors::{Error, Result};

pub fn find_window(caption: &str, class: Option<&str>) -> HWND {
    if class.is_some() {
        unsafe { FindWindowW(class.unwrap(), caption) }
    } else {
        unsafe { FindWindowW(PCWSTR::default(), caption) }
    }
}

#[derive(Debug)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub fn get_window_rect(hwnd: &HWND) -> Option<Rect> {
    let mut rect: RECT = unsafe { std::mem::zeroed() };
    let ret = unsafe { GetWindowRect(hwnd, &mut rect) };
    if !ret.as_bool() {
        return None
    }
    Some(Rect {
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
    })
}

extern "system" fn wnd_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if msg == WM_INPUT {
        let mut kri: RAWINPUT = unsafe { std::mem::zeroed() };
        let mut kri_size: u32 = std::mem::size_of::<RAWINPUT>() as u32;
        let ret = unsafe {
            GetRawInputData(
                HRAWINPUT(l_param.0),
                RID_INPUT,
                std::mem::transmute(&mut kri),
                &mut kri_size,
                std::mem::size_of::<RAWINPUTHEADER>() as u32,
            )
        };
        if ret as i32 == -1 {
            println!(
                "GetRawInputData() failed, {:?}",
                std::io::Error::last_os_error()
            );
        } else {
            unsafe {
                let kb = &kri.data.keyboard;
                println!(
                    "GetRawInputData() succeeded. vk:{}, flags:{}, make code:{}, message:{}",
                    kb.VKey, kb.Flags, kb.MakeCode, kb.Message
                );
            }
        }
    }
    unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) }
}

pub fn log_key() -> Result<()> {
    let mut wcx: WNDCLASSEXA = unsafe { std::mem::zeroed() };
    wcx.cbSize = std::mem::size_of::<WNDCLASSEXA>() as u32;
    wcx.lpfnWndProc = Some(wnd_proc);
    wcx.hInstance = unsafe { GetModuleHandleA(PCSTR::default()).unwrap() };
    wcx.lpszClassName = PCSTR("KeyLogger\0".as_ptr());

    let ret = unsafe { RegisterClassExA(&wcx) };
    if ret == 0 {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "RegisterClassExA() failed"
        ));
    }

    let hwnd = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            wcx.lpszClassName,
            "KeyLogger",
            WINDOW_STYLE::default(),
            0,
            0,
            100,
            100,
            HWND_MESSAGE,
            HMENU::default(),
            wcx.hInstance,
            std::ptr::null_mut(),
        )
    };

    if hwnd.0 == 0 {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "CreateWindowExA() failed"
        ));
    }

    let mut rids: [RAWINPUTDEVICE; 1] = unsafe { std::mem::zeroed() };
    rids[0].usUsagePage = 1;
    rids[0].usUsage = 6;
    rids[0].dwFlags = RIDEV_INPUTSINK;
    rids[0].hwndTarget = hwnd;

    let ret =
        unsafe { RegisterRawInputDevices(&rids, std::mem::size_of::<RAWINPUTDEVICE>() as u32) };

    if !ret.as_bool() {
        bail!(Error::with_chain(
            std::io::Error::last_os_error(),
            "RegisterRawInputDevices() failed"
        ));
    }

    print!("RegisterRawInputDevices() returned {:?}", ret);

    let mut msg: windows::Win32::UI::WindowsAndMessaging::MSG = unsafe { std::mem::zeroed() };
    while unsafe { GetMessageA(&mut msg, HWND::default(), 0, 0) }.as_bool() {
        unsafe { TranslateMessage(&msg) };
        unsafe { DispatchMessageA(&msg) };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_find_window() {
        let hwnd = find_window("中文.txt - Notepad", None);
        println!("hwnd:{}", hwnd.0);
        assert!(hwnd != Default::default());
    }

    #[test]
    #[ignore]
    fn test_get_window_rect() {
        let hwnd = find_window("中文.txt - Notepad", None);
        println!("hwnd:{}", hwnd.0);
        assert!(hwnd != Default::default());
        let rect = get_window_rect(&hwnd);
        println!("rect:{:?}", rect);
    }

    #[test]
    #[ignore]
    fn test_log_key() {
        log_key().unwrap();
    }
}
