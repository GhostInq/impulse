//! Running inside Wine or Proton: whether we are, a Unix path in the form Wine gives it, Steam's own folder on
//! the Linux side, and the game Proton was asked to start when Steam's launch options put Impulse in its place.
//! On Windows every answer is `None`, so nothing here changes how Impulse behaves there.

mod game;
mod path;
mod window;

use std::{
    ffi::{CStr, c_char},
    mem::transmute,
    sync::OnceLock,
};

use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};

pub use game::{Command, FLAG, Game, command};
pub use path::{dos_path, steam_root};
pub use window::own_title_bar;

/// Wine's version (`11.0`), `None` on Windows. Read once.
pub fn version() -> Option<&'static str> {
    static VERSION: OnceLock<Option<String>> = OnceLock::new();
    VERSION.get_or_init(read_version).as_deref()
}

fn read_version() -> Option<String> {
    // Only Wine's ntdll has it.
    let get = export(c"ntdll.dll", c"wine_get_version")?;
    // SAFETY: `wine_get_version` takes nothing and returns a static NUL-terminated string.
    let get = unsafe { transmute::<unsafe extern "system" fn() -> isize, unsafe extern "C" fn() -> *const c_char>(get) };
    // SAFETY: as above; a null result is checked before it is read.
    let text = unsafe { get() };
    // SAFETY: a non-null result is Wine's static version string.
    (!text.is_null()).then(|| unsafe { CStr::from_ptr(text) }.to_string_lossy().into_owned())
}

/// An export of a module this process has already loaded.
fn export(module: &CStr, name: &CStr) -> Option<unsafe extern "system" fn() -> isize> {
    let wide: Vec<u16> = module.to_str().ok()?.encode_utf16().chain([0]).collect();
    // SAFETY: the name is NUL-terminated; ntdll and kernel32 are loaded in every process.
    let module = unsafe { GetModuleHandleW(wide.as_ptr()) };
    if module.is_null() {
        return None;
    }
    // SAFETY: a plain lookup in a loaded module with a NUL-terminated name.
    unsafe { GetProcAddress(module, name.as_ptr().cast()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run the tests under Wine to see this hold (`wine target/.../wine-*.exe`); on Windows it is `None`.
    #[test]
    fn the_version_is_wines_own_or_none() {
        match version() {
            Some(version) => assert!(version.chars().next().is_some_and(|c| c.is_ascii_digit()), "{version}"),
            None => assert!(export(c"ntdll.dll", c"wine_get_version").is_none()),
        }
    }
}
