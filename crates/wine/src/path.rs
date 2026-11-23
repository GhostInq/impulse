//! Unix paths as Wine shows them to Windows programs.

use std::{
    env,
    ffi::{CString, OsString, c_char},
    mem::transmute,
    os::windows::ffi::OsStringExt,
    path::PathBuf,
    slice,
};

use windows_sys::Win32::System::Memory::{GetProcessHeap, HeapFree};

use crate::{export, version};

// Steam names its own folder for the compatibility tool it starts: Proton, and what Proton starts.
const STEAM_ROOT: &str = "STEAM_COMPAT_CLIENT_INSTALL_PATH";

/// `/home/me/Games` as Wine maps it (`Z:\home\me\Games`, or a drive letter Proton made for it). `None` on
/// Windows, for a relative path and for one Wine cannot map.
pub fn dos_path(unix: &str) -> Option<PathBuf> {
    version()?;
    if !unix.starts_with('/') {
        return None;
    }
    let convert = export(c"kernel32.dll", c"wine_get_dos_file_name")?;
    // SAFETY: `wine_get_dos_file_name` takes a NUL-terminated path in the Unix code page (UTF-8 here) and returns a
    // NUL-terminated wide string from the process heap, or null.
    let convert = unsafe { transmute::<unsafe extern "system" fn() -> isize, unsafe extern "C" fn(*const c_char) -> *mut u16>(convert) };
    let unix = CString::new(unix).ok()?;
    // SAFETY: as above; `unix` outlives the call.
    let dos = unsafe { convert(unix.as_ptr()) };
    if dos.is_null() {
        return None;
    }
    // SAFETY: `dos` is NUL-terminated, so the scan stays inside it.
    let len = (0..).take_while(|&i| unsafe { *dos.add(i) } != 0).count();
    // SAFETY: the first `len` units are the string, read before the buffer is freed.
    let path = OsString::from_wide(unsafe { slice::from_raw_parts(dos, len) });
    // SAFETY: Wine allocated `dos` on the process heap for us, and nothing reads it after this.
    unsafe { HeapFree(GetProcessHeap(), 0, dos.cast()) };
    (len > 0).then(|| path.into())
}

/// Steam's own folder on the Linux side, in Windows form: under Proton, Steam's library records live there,
/// not in the prefix's stand-in `C:\Program Files (x86)\Steam`. `None` on Windows or outside Proton.
pub fn steam_root() -> Option<PathBuf> {
    let root = env::var(STEAM_ROOT).ok()?;
    dos_path(root.trim_end_matches('/')).filter(|dir| dir.is_dir())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_an_absolute_unix_path_is_mapped_and_only_under_wine() {
        assert_eq!(dos_path("relative/path"), None);
        assert_eq!(dos_path(r"C:\Windows"), None);
        let root = dos_path("/");
        match version() {
            // Wine maps the Unix root to a drive unless a prefix removed it.
            Some(_) => assert!(root.as_ref().is_none_or(|root| root.to_string_lossy().ends_with(":\\")), "{root:?}"),
            None => assert_eq!(root, None),
        }
    }
}
