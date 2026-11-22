//! Native Windows window icon attacher.
//!
//! Windows applications built on top of windowing libraries like winit register a custom
//! window class with no default class icon (hIcon = NULL). Even if an `.ico` is embedded
//! in the PE executable via resource compiler (winres), Windows does not automatically apply
//! it to the live HWND title bar.
//!
//! This module resolves the top-level visible HWND of WinBlast0 and delivers `WM_SETICON`
//! messages for both `ICON_BIG` (32x32 for taskbar & Alt+Tab) and `ICON_SMALL` (16x16 for the
//! title bar), ensuring the official WinBlast0 lightning flame icon is permanently visible.

static EMBEDDED_ICON_BYTES: &[u8] = include_bytes!("../../assets/winblast.ico");

#[cfg(windows)]
pub fn attach_window_icon() {
    use windows::core::{BOOL, PCWSTR};
    use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::System::Threading::GetCurrentProcessId;
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowLongW, GetWindowThreadProcessId, IsWindowVisible, LoadImageW,
        SendMessageW, GWL_EXSTYLE, HICON, ICON_BIG, ICON_SMALL, IMAGE_ICON, LR_DEFAULTCOLOR,
        LR_LOADFROMFILE, WM_SETICON, WS_EX_TOOLWINDOW,
    };

    unsafe {
        unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let mut pid = 0u32;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid == GetCurrentProcessId() && IsWindowVisible(hwnd).as_bool() {
                let style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
                let is_tool = (style & WS_EX_TOOLWINDOW.0) != 0;
                if !is_tool {
                    *(lparam.0 as *mut HWND) = hwnd;
                    return BOOL(0); // Stop enumeration, window found
                }
            }
            BOOL(1) // Continue enumeration
        }

        let mut target_hwnd = HWND::default();
        let _ = EnumWindows(Some(enum_proc), LPARAM(&mut target_hwnd as *mut _ as isize));

        if !target_hwnd.is_invalid() {
            // Ensure window is cleanly maximized to full desktop work area (retaining standard frame & taskbar)
            let _ = windows::Win32::UI::WindowsAndMessaging::ShowWindow(
                target_hwnd,
                windows::Win32::UI::WindowsAndMessaging::SW_MAXIMIZE,
            );

            let hinstance = GetModuleHandleW(None).unwrap_or_default();

            // 1. Primary: load from embedded PE resource ID 1 (placed by winres)
            let mut hicon_big = LoadImageW(
                Some(hinstance.into()),
                PCWSTR(1 as _),
                IMAGE_ICON,
                32,
                32,
                LR_DEFAULTCOLOR,
            )
            .ok()
            .map(|h| HICON(h.0));

            let mut hicon_small = LoadImageW(
                Some(hinstance.into()),
                PCWSTR(1 as _),
                IMAGE_ICON,
                16,
                16,
                LR_DEFAULTCOLOR,
            )
            .ok()
            .map(|h| HICON(h.0));

            // 2. Secondary fallback: write embedded binary icon bytes to %TEMP% and load directly
            if hicon_big.is_none() || hicon_small.is_none() {
                let temp_icon_path = std::env::temp_dir().join("winblast0_app_icon.ico");
                let _ = std::fs::write(&temp_icon_path, EMBEDDED_ICON_BYTES);
                let wpath: Vec<u16> = temp_icon_path
                    .to_string_lossy()
                    .encode_utf16()
                    .chain(std::iter::once(0))
                    .collect();

                if hicon_big.is_none() {
                    hicon_big = LoadImageW(
                        None,
                        PCWSTR(wpath.as_ptr()),
                        IMAGE_ICON,
                        32,
                        32,
                        LR_LOADFROMFILE | LR_DEFAULTCOLOR,
                    )
                    .ok()
                    .map(|h| HICON(h.0));
                }
                if hicon_small.is_none() {
                    hicon_small = LoadImageW(
                        None,
                        PCWSTR(wpath.as_ptr()),
                        IMAGE_ICON,
                        16,
                        16,
                        LR_LOADFROMFILE | LR_DEFAULTCOLOR,
                    )
                    .ok()
                    .map(|h| HICON(h.0));
                }
            }

            // Deliver WM_SETICON for both big and small icons
            if let Some(h) = hicon_big {
                SendMessageW(
                    target_hwnd,
                    WM_SETICON,
                    Some(WPARAM(ICON_BIG as _)),
                    Some(LPARAM(h.0 as _)),
                );
            }
            if let Some(h) = hicon_small {
                SendMessageW(
                    target_hwnd,
                    WM_SETICON,
                    Some(WPARAM(ICON_SMALL as _)),
                    Some(LPARAM(h.0 as _)),
                );
            }
        }
    }
}

#[cfg(not(windows))]
pub fn attach_window_icon() {}
