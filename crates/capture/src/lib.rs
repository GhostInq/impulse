//! One frame of a window through Windows.Graphics.Capture. Data only, no UI. Windows only.
#![cfg(windows)]

mod grab;
mod readback;

use std::time::Duration;

use anyhow::{Context, Result, bail};
use windows::Win32::Foundation::{HWND, RPC_E_CHANGED_MODE};
use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize};
use windows::Win32::UI::WindowsAndMessaging::IsIconic;

/// One frame of a window: BGRA, rows tight (`width * 4` bytes each), top-down.
pub struct Shot {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

/// One frame of `window` (an HWND as isize), the cursor and the capture border left out where Windows lets it.
/// Blocking: waits `within` at most for the first frame.
pub fn window(window: isize, within: Duration) -> Result<Shot> {
    let hwnd = HWND(window as *mut core::ffi::c_void);
    // SAFETY: IsIconic only reads window state and accepts any handle value.
    if unsafe { IsIconic(hwnd) }.as_bool() {
        bail!("the window is minimized");
    }
    let _apartment = Apartment::enter()?;
    grab::first_frame(hwnd, within)
}

// Balances this call's CoInitializeEx on the pool thread.
struct Apartment {
    owned: bool,
}

impl Apartment {
    fn enter() -> Result<Self> {
        // SAFETY: plain COM init on this thread; the matching uninit is in drop.
        let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        // The thread already has another apartment: use it as it is.
        if hr == RPC_E_CHANGED_MODE {
            return Ok(Self { owned: false });
        }
        hr.ok().context("initialize COM")?;
        Ok(Self { owned: true })
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        if self.owned {
            // SAFETY: pairs with the successful CoInitializeEx in enter, on the same thread.
            unsafe { CoUninitialize() };
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use windows::Win32::UI::WindowsAndMessaging::FindWindowW;
    use windows::core::{PCWSTR, w};

    /// Needs a desktop session with a taskbar, hence ignored.
    #[test]
    #[ignore = "needs a desktop session"]
    fn captures_the_taskbar() {
        // SAFETY: the class name is a static NUL-terminated string; a null title matches any.
        let taskbar = unsafe { FindWindowW(w!("Shell_TrayWnd"), PCWSTR::null()) }.unwrap();
        let shot = super::window(taskbar.0 as isize, Duration::from_secs(2)).unwrap();
        assert!(shot.width > 0 && shot.height > 0);
        assert_eq!(shot.bgra.len(), shot.width as usize * shot.height as usize * 4);
    }
}
