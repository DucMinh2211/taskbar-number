#![windows_subsystem = "windows"]

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Com::*;
use windows::Win32::UI::Accessibility::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

use windows::Win32::UI::Accessibility::IUIAutomation;
use windows::Win32::System::Variant::{VARIANT, VT_BSTR};

use std::sync::{Arc, Mutex};

const WM_TRAYICON: u32 = WM_USER + 1;
const ID_TRAY_EXIT: u32 = 1001;
const ID_TRAY_PAUSE: u32 = 1002;

struct IconInfo {
    rect: RECT,
}

struct AppState {
    icons: Vec<IconInfo>,
    hwnd: HWND,
    is_paused: bool,
}

lazy_static::lazy_static! {
    static ref APP_STATE: Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState { 
        icons: Vec::new(),
        hwnd: HWND(0),
        is_paused: false,
    }));
}

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;
        std::thread::spawn(|| {
            update_icons_loop();
        });
        create_overlay_window()?;
    }
    Ok(())
}

fn create_string_variant(s: &str) -> VARIANT {
    let bstr = BSTR::from(s);
    let mut var = VARIANT::default();
    unsafe {
        (*var.Anonymous.Anonymous).vt = VT_BSTR;
        (*var.Anonymous.Anonymous).Anonymous.bstrVal = std::mem::ManuallyDrop::new(bstr);
    }
    var
}

fn update_icons_loop() {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL).unwrap();

        loop {
            let (is_paused, hwnd_overlay) = {
                let state = APP_STATE.lock().unwrap();
                (state.is_paused, state.hwnd)
            };

            if is_paused {
                if hwnd_overlay.0 != 0 {
                    let _ = ShowWindow(hwnd_overlay, SW_HIDE);
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
                continue;
            } else {
                if hwnd_overlay.0 != 0 {
                    let _ = ShowWindow(hwnd_overlay, SW_SHOWNOACTIVATE);
                }
            }

            let mut new_icons = Vec::new();
            let cond_tasklist = automation.CreatePropertyCondition(UIA_ClassNamePropertyId, create_string_variant("MSTaskListWClass")).unwrap();
            let root = automation.GetRootElement().unwrap();
            
            if let Ok(task_list) = root.FindFirst(TreeScope_Descendants, &cond_tasklist) {
                let true_condition = automation.CreateTrueCondition().unwrap();
                if let Ok(children) = task_list.FindAll(TreeScope_Children, &true_condition) {
                    for i in 0..children.Length().unwrap_or(0) {
                        if let Ok(el) = children.GetElement(i) {
                            if let Ok(rect) = el.CurrentBoundingRectangle() {
                                if rect.right > rect.left && rect.bottom > rect.top {
                                    new_icons.push(IconInfo { rect });
                                }
                            }
                        }
                    }
                }
            } else {
                let cond_frame = automation.CreatePropertyCondition(UIA_ClassNamePropertyId, create_string_variant("Taskbar.TaskbarFrameAutomationPeer")).unwrap();
                if let Ok(frame) = root.FindFirst(TreeScope_Descendants, &cond_frame) {
                    let true_cond = automation.CreateTrueCondition().unwrap();
                    if let Ok(all) = frame.FindAll(TreeScope_Children, &true_cond) {
                        for i in 0..all.Length().unwrap_or(0) {
                            if let Ok(el) = all.GetElement(i) {
                                let name = el.CurrentName().unwrap_or(BSTR::new()).to_string().to_lowercase();
                                let is_system = name.contains("start") || name.contains("bắt đầu") || 
                                               name.contains("search") || name.contains("tìm kiếm") ||
                                               name.contains("widgets") || name.contains("tiện ích") ||
                                               name.contains("task view") || name.contains("nhiệm vụ") ||
                                               name.is_empty();

                                if !is_system {
                                    if let Ok(rect) = el.CurrentBoundingRectangle() {
                                        if rect.right > rect.left && rect.bottom > rect.top {
                                            new_icons.push(IconInfo { rect });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let hwnd_overlay = {
                let mut state = APP_STATE.lock().unwrap();
                state.icons = new_icons;
                state.hwnd
            };

            if hwnd_overlay.0 != 0 {
                // Ép cửa sổ overlay luôn nằm trên cùng (HWND_TOPMOST) 
                // Sử dụng SWP_NOACTIVATE để không chiếm tiêu điểm khi người dùng đang gõ
                let _ = SetWindowPos(hwnd_overlay, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_SHOWWINDOW);
                InvalidateRect(hwnd_overlay, None, true);
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
            
            let mut rect_win = RECT::default();
            let _ = unsafe { GetClientRect(hwnd, &mut rect_win) };

            let brush = unsafe { CreateSolidBrush(COLORREF(0)) };
            unsafe { FillRect(hdc, &rect_win, brush) };
            unsafe { DeleteObject(brush) };

            unsafe { SetBkMode(hdc, TRANSPARENT) };
            
            let hfont = unsafe { CreateFontW(22, 0, 0, 0, FW_BOLD.0 as i32, 0, 0, 0, DEFAULT_CHARSET.0 as u32, 
                                   OUT_DEFAULT_PRECIS.0 as u32, CLIP_DEFAULT_PRECIS.0 as u32, 
                                   CLEARTYPE_QUALITY.0 as u32, DEFAULT_PITCH.0 as u32 | FF_DONTCARE.0 as u32, 
                                   w!("Arial")) };
            unsafe { SelectObject(hdc, hfont) };

            let state = APP_STATE.lock().unwrap();
            for (i, icon) in state.icons.iter().enumerate() {
                let num = (i + 1) % 10;
                let text = format!("{}", num);
                let wide_text: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
                
                let x = icon.rect.left + (icon.rect.right - icon.rect.left) / 2 - 5;
                let y = icon.rect.top + (icon.rect.bottom - icon.rect.top) / 2 - 10;

                unsafe { SetTextColor(hdc, COLORREF(0x000000)) };
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            let _ = unsafe { TextOutW(hdc, x + dx, y + dy, &wide_text) };
                        }
                    }
                }

                unsafe { SetTextColor(hdc, COLORREF(0x00FFFF)) }; 
                let _ = unsafe { TextOutW(hdc, x, y, &wide_text) };
            }

            unsafe { DeleteObject(hfont) };
            unsafe { EndPaint(hwnd, &ps) };
            LRESULT(0)
        }
        WM_TRAYICON => {
            if lparam.0 as u32 == WM_RBUTTONUP {
                let mut pt = POINT::default();
                let _ = unsafe { GetCursorPos(&mut pt) };
                unsafe { SetForegroundWindow(hwnd) };
                
                if let Ok(hmenu) = unsafe { CreatePopupMenu() } {
                    let is_paused = {
                        let state = APP_STATE.lock().unwrap();
                        state.is_paused
                    };
                    let pause_text = if is_paused { w!("Resume") } else { w!("Pause") };
                    let _ = unsafe { AppendMenuW(hmenu, MF_STRING, ID_TRAY_PAUSE as usize, pause_text) };
                    let _ = unsafe { AppendMenuW(hmenu, MF_STRING, ID_TRAY_EXIT as usize, w!("Exit")) };
                    let _ = unsafe { TrackPopupMenu(hmenu, TPM_LEFTALIGN | TPM_RIGHTBUTTON, pt.x, pt.y, 0, hwnd, None) };
                    let _ = unsafe { DestroyMenu(hmenu) };
                }
            }
            LRESULT(0)
        }
        WM_COMMAND => {
            let id = (wparam.0 & 0xffff) as u32;
            if id == ID_TRAY_EXIT {
                unsafe { PostQuitMessage(0) };
            } else if id == ID_TRAY_PAUSE {
                let mut state = APP_STATE.lock().unwrap();
                state.is_paused = !state.is_paused;
                if state.is_paused {
                    state.icons.clear();
                    let _ = unsafe { ShowWindow(hwnd, SW_HIDE) };
                } else {
                    let _ = unsafe { ShowWindow(hwnd, SW_SHOWNOACTIVATE) };
                }
            }
            LRESULT(0)
        }
        WM_DESTROY => {
            let nid = NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: hwnd,
                uID: 1,
                ..Default::default()
            };
            let _ = unsafe { Shell_NotifyIconW(NIM_DELETE, &nid) };
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

fn create_overlay_window() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        let class_name = w!("TaskbarNumberOverlay");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };

        RegisterClassW(&wc);

        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);

        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
            class_name,
            w!("Taskbar Number Overlay"),
            WS_POPUP,
            0, 0, screen_w, screen_h - 1,
            None, None, instance, None,
        );

        {
            let mut state = APP_STATE.lock().unwrap();
            state.hwnd = hwnd;
        }

        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: 1,
            uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
            uCallbackMessage: WM_TRAYICON,
            hIcon: LoadIconW(None, IDI_APPLICATION).unwrap(),
            ..Default::default()
        };
        
        let tip = w!("Taskbar Number");
        let tip_slice = tip.as_wide();
        let len = tip_slice.len().min(127);
        for i in 0..len {
            nid.szTip[i] = tip_slice[i];
        }

        Shell_NotifyIconW(NIM_ADD, &nid);

        SetLayeredWindowAttributes(hwnd, COLORREF(0), 0, LWA_COLORKEY)?;
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).as_bool() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}
