//! Loads a DLL into a running process and starts one of its exports there.
//! x64 and x86 (WOW64) targets, each with an image of its own architecture.

use std::{
    ffi::{OsString, c_void},
    io,
    mem::transmute,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
    ptr,
};

use anyhow::{Context, Result, anyhow, ensure};
use model::{Failure, FailureKind, Process};
use windows_sys::Win32::{
    Foundation::{ERROR_ACCESS_DENIED, HMODULE, WAIT_OBJECT_0},
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        LibraryLoader::{GetModuleHandleW, GetProcAddress},
        Memory::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx},
        ProcessStatus::{EnumProcessModulesEx, GetModuleFileNameExW, LIST_MODULES_32BIT, LIST_MODULES_ALL},
        SystemInformation::GetSystemWow64DirectoryW,
        Threading::{
            CreateRemoteThread, IsWow64Process, OpenProcess, PROCESS_CREATE_THREAD, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
            PROCESS_VM_READ, PROCESS_VM_WRITE, WaitForSingleObject,
        },
    },
};

use crate::{elevated, handle::Handle, image::Image, process};

const LOAD_WITHIN_MS: u32 = 30_000;

type ThreadStart = unsafe extern "system" fn(*mut c_void) -> u32;

/// Loads `image` into `target` and starts `export` with a pointer to a copy of `arg`, which stays
/// in the target: the thread reads it after this returns. Returns once the thread runs; the DLL
/// stays mapped for the process's lifetime. `target` is checked against the opened process: a
/// reused pid must not receive the DLL. A process that denies us (a game run as administrator) is loaded by an
/// elevated copy of the host executable, once the user accepts UAC's prompt (`serve_elevated`).
pub fn load(target: &Process, image: &Image, export: &str, arg: &[u8]) -> Result<()> {
    let pid = target.pid;
    let start = pe::export(image.path(), export).with_context(|| format!("{} has no {export}", image.path().display()))?;
    let process = match open(pid) {
        Ok(process) => process,
        Err(denied) if marked(&denied, FailureKind::Protected) && elevated::can_elevate() => {
            return elevated::load(target, image, export, arg);
        }
        Err(e) => return Err(e),
    };
    process::verify(&process, target)?;
    let mut wow = 0;
    // SAFETY: `process` is a live handle and `wow` outlives the call.
    ensure!(unsafe { IsWow64Process(process.0, &mut wow) } != 0, "cannot query process {pid}: {}", io::Error::last_os_error());
    let wow64 = wow != 0;
    if wow64 == image.x64() {
        let (game, dll) = if wow64 { ("32-bit", "x64") } else { ("64-bit", "x86") };
        let error = anyhow!("process {pid} is {game}, {} is {dll}", image.path().display());
        return Err(error.context(Failure::new(FailureKind::Architecture)));
    }
    let load_library = if wow64 { wow64_load_library(&process, pid)? } else { load_library()? };

    let wide: Vec<u8> = image.path().as_os_str().encode_wide().chain([0]).flat_map(u16::to_le_bytes).collect();
    let path = write(&process, &wide)?;
    let loader = spawn(&process, load_library, path.ptr)?;
    if let Err(err) = wait(&loader) {
        // The loader may yet read the path and open the file, so both must outlive this call.
        path.leak();
        image.keep_locked();
        return Err(err);
    }
    drop(path);
    // The thread's exit code is only the low half of the module handle, so the module list decides.
    // Exactly the image's path: a copy of the same file name from elsewhere is a different module.
    let wanted = image.path().to_string_lossy();
    let base = module_base(&process, pid, LIST_MODULES_ALL, |path| path.eq_ignore_ascii_case(&wanted))?
        .with_context(|| format!("{} did not load in process {pid}: a dependency may be missing", image.path().display()))?;

    let arg = write(&process, arg)?;
    spawn(&process, start_address(base, start), arg.ptr)?;
    arg.leak();
    Ok(())
}

/// Memory allocated in the target, released on drop unless `leak`ed.
struct Remote<'a> {
    process: &'a Handle,
    ptr: *mut c_void,
}

impl Remote<'_> {
    fn leak(self) {
        std::mem::forget(self);
    }
}

impl Drop for Remote<'_> {
    fn drop(&mut self) {
        // SAFETY: `ptr` came from `VirtualAllocEx` on this process and is released once.
        unsafe { VirtualFreeEx(self.process.0, self.ptr, 0, MEM_RELEASE) };
    }
}

fn open(pid: u32) -> Result<Handle> {
    let access = PROCESS_CREATE_THREAD | PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_WRITE | PROCESS_VM_READ;
    // SAFETY: plain Win32 call; the handle is owned by the returned guard.
    let handle = unsafe { OpenProcess(access, 0, pid) };
    if handle.is_null() {
        let error = io::Error::last_os_error();
        let denied = error.raw_os_error() == Some(ERROR_ACCESS_DENIED as i32);
        let error = anyhow!("cannot open process {pid}: {error}");
        // Run as administrator, or kept by its protection.
        return Err(if denied { error.context(Failure::new(FailureKind::Protected)) } else { error });
    }
    Ok(Handle(handle))
}

pub(crate) fn marked(error: &anyhow::Error, kind: FailureKind) -> bool {
    error.downcast_ref::<Failure>().is_some_and(|failure| failure.kind == kind)
}

fn write<'a>(process: &'a Handle, data: &[u8]) -> Result<Remote<'a>> {
    // SAFETY: plain Win32 call; a null result is checked below.
    let ptr = unsafe { VirtualAllocEx(process.0, ptr::null(), data.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };
    if ptr.is_null() {
        return Err(guarded("cannot allocate in the process"));
    }
    let remote = Remote { process, ptr };
    let mut written = 0;
    // SAFETY: the buffer is valid for `data.len()` bytes and the target block was sized for it.
    let copied = unsafe { WriteProcessMemory(process.0, ptr, data.as_ptr().cast(), data.len(), &mut written) } != 0;
    if !copied || written != data.len() {
        return Err(guarded("cannot write to the process"));
    }
    Ok(remote)
}

/// Starts a thread at `start(arg)` in the process; it runs on if the handle is dropped.
fn spawn(process: &Handle, start: ThreadStart, arg: *mut c_void) -> Result<Handle> {
    // SAFETY: `start` is a function of the target taking one pointer (its kernel32's `LoadLibraryW`, or
    // base plus RVA for the loaded image), and `arg` points into the target.
    let thread = unsafe { CreateRemoteThread(process.0, ptr::null(), 0, Some(start), arg, 0, ptr::null_mut()) };
    if thread.is_null() {
        return Err(guarded("cannot start a thread in the process"));
    }
    Ok(Handle(thread))
}

/// The last error of a step on a process that opened: a denial there is its protection (an anti-cheat strips the
/// handle's rights), which elevation does not lift.
fn guarded(what: &str) -> anyhow::Error {
    let error = io::Error::last_os_error();
    let denied = error.raw_os_error() == Some(ERROR_ACCESS_DENIED as i32);
    let error = anyhow!("{what}: {error}");
    if denied { error.context(Failure::new(FailureKind::Protected)) } else { error }
}

fn wait(thread: &Handle) -> Result<()> {
    // SAFETY: the handle is a live thread handle.
    let done = unsafe { WaitForSingleObject(thread.0, LOAD_WITHIN_MS) } == WAIT_OBJECT_0;
    ensure!(done, "the remote thread did not finish in time");
    Ok(())
}

/// kernel32 maps at one address in every x64 process of a boot, so ours is the target's.
fn load_library() -> Result<ThreadStart> {
    let kernel32: Vec<u16> = "kernel32.dll\0".encode_utf16().collect();
    // SAFETY: kernel32 is always loaded; both strings are NUL-terminated.
    let address = unsafe { GetProcAddress(GetModuleHandleW(kernel32.as_ptr()), c"LoadLibraryW".as_ptr().cast()) };
    let address = address.context("LoadLibraryW not found")?;
    // SAFETY: LoadLibraryW takes one pointer, which is a thread routine's shape.
    Ok(unsafe { transmute::<unsafe extern "system" fn() -> isize, ThreadStart>(address) })
}

/// A 32-bit process has its own kernel32, at its own address.
fn wow64_load_library(process: &Handle, pid: u32) -> Result<ThreadStart> {
    let mut dir = [0u16; 260];
    // SAFETY: `dir` is valid for the length passed.
    let len = unsafe { GetSystemWow64DirectoryW(dir.as_mut_ptr(), dir.len() as u32) } as usize;
    ensure!(len != 0 && len < dir.len(), "no 32-bit system folder: {}", io::Error::last_os_error());
    let kernel32 = PathBuf::from(OsString::from_wide(&dir[..len])).join("kernel32.dll");
    let rva = pe::export(&kernel32, "LoadLibraryW").context("the 32-bit LoadLibraryW not found")?;
    // Recorded under System32, which WOW64 redirects: the file name finds it.
    let named = |path: &str| Path::new(path).file_name().is_some_and(|name| name.eq_ignore_ascii_case("kernel32.dll"));
    let base = module_base(process, pid, LIST_MODULES_32BIT, named)?.with_context(|| format!("process {pid} has no 32-bit kernel32"))?;
    Ok(start_address(base, rva))
}

fn start_address(base: usize, rva: u32) -> ThreadStart {
    // SAFETY: `base + rva` is the image's exported start routine, a thread routine.
    unsafe { transmute::<usize, ThreadStart>(base + rva as usize) }
}

/// The load address of the first module of `filter` whose path `matches`. Not Toolhelp: its
/// snapshot fails outright when any module of the target has a path beyond MAX_PATH.
fn module_base(process: &Handle, pid: u32, filter: u32, matches: impl Fn(&str) -> bool) -> Result<Option<usize>> {
    let mut modules: Vec<HMODULE> = vec![ptr::null_mut(); 1024];
    loop {
        let mut needed = 0;
        // SAFETY: `modules` is valid for the byte length passed and `needed` outlives the call.
        let listed = unsafe {
            EnumProcessModulesEx(process.0, modules.as_mut_ptr(), (modules.len() * size_of::<HMODULE>()) as u32, &mut needed, filter)
        };
        ensure!(listed != 0, "cannot list the modules of process {pid}: {}", io::Error::last_os_error());
        let count = needed as usize / size_of::<HMODULE>();
        if count <= modules.len() {
            modules.truncate(count);
            break;
        }
        // More modules loaded since: the room needed, with some to spare.
        modules.resize(count + 16, ptr::null_mut());
    }
    let mut name = vec![0u16; 32_768];
    for module in modules {
        // SAFETY: `name` is valid for the length passed.
        let len = unsafe { GetModuleFileNameExW(process.0, module, name.as_mut_ptr(), name.len() as u32) } as usize;
        if len != 0 && matches(&String::from_utf16_lossy(&name[..len])) {
            return Ok(Some(module as usize));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        process::{Child, Command, Stdio},
        thread::sleep,
        time::Duration,
    };

    use super::*;

    const KERNEL32: &str = r"C:\Windows\System32\kernel32.dll";
    const KERNEL32_X86: &str = r"C:\Windows\SysWOW64\kernel32.dll";

    /// A 32-bit process that waits on its stdin, killed on drop.
    struct Wow64(Child);

    impl Drop for Wow64 {
        fn drop(&mut self) {
            self.0.kill().ok();
            self.0.wait().ok();
        }
    }

    #[test]
    fn an_image_without_the_export_is_refused_before_a_process_is_opened() {
        let nobody = Process { pid: u32::MAX, path: PathBuf::new(), created: 0 };
        // The pid is bogus: only the first check can be what fails.
        let err = load(&nobody, &Image::unchecked(KERNEL32), "NoSuchExport", &[]).unwrap_err();
        assert!(err.to_string().contains("NoSuchExport"), "{err}");
    }

    #[test]
    fn a_process_that_is_not_the_one_found_is_never_injected() {
        let image = Image::unchecked(KERNEL32);
        assert!(pe::export(image.path(), "Sleep").is_some(), "the export must pass the first check");
        let mut me = process::of(std::process::id()).unwrap();
        // A pid that was reused has another creation time: nothing may happen before this error.
        me.created += 1;
        let err = load(&me, &image, "Sleep", &[]).unwrap_err();
        assert!(err.to_string().contains("reused"), "{err}");
    }

    #[test]
    fn a_32_bit_process_loads_an_x86_image_and_starts_its_export() {
        // A copy outside the system folders loads by its own path, as a trainer's DLL does.
        let copy = env::current_exe().unwrap().with_file_name(format!("wow64-{}.dll", std::process::id()));
        fs::copy(r"C:\Windows\SysWOW64\dbghelp.dll", &copy).unwrap();
        // `/D`: no AutoRun command may end it early.
        let mut shell = Command::new(r"C:\Windows\SysWOW64\cmd.exe");
        shell.arg("/D").stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null());
        let mut game = Wow64(shell.spawn().unwrap());
        let target = process::of(game.0.id()).unwrap();
        let image = Image::unchecked_x86(copy.to_str().unwrap());
        // SymCleanup takes one handle: a pointer to these bytes is none, so it only fails.
        let mut loaded = load(&target, &image, "SymCleanup", &[0; 4]);
        // Its loader maps kernel32 a moment after the process is made; nothing remote ran before that check.
        for _ in 0..100 {
            if !loaded.as_ref().is_err_and(|e| e.to_string().contains("no 32-bit kernel32")) {
                break;
            }
            sleep(Duration::from_millis(100));
            loaded = load(&target, &image, "SymCleanup", &[0; 4]);
        }
        let refused = load(&target, &Image::unchecked(KERNEL32), "Sleep", &[]);
        // A wrong start address would have crashed it.
        sleep(Duration::from_millis(300));
        let alive = game.0.try_wait().unwrap().is_none();
        drop(game);
        fs::remove_file(&copy).ok();
        loaded.unwrap();
        assert!(alive, "the export crashed the process");
        let err = refused.unwrap_err();
        assert!(marked(&err, FailureKind::Architecture), "{err}");
    }

    #[test]
    fn a_64_bit_process_loads_an_x64_image_and_starts_its_export() {
        // Our kernel32's LoadLibraryW must be the target's: true on Windows, and checked here under Wine.
        let copy = env::current_exe().unwrap().with_file_name(format!("x64-{}.dll", std::process::id()));
        fs::copy(r"C:\Windows\System32\dbghelp.dll", &copy).unwrap();
        let mut shell = Command::new(r"C:\Windows\System32\cmd.exe");
        shell.arg("/D").stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null());
        let mut game = Wow64(shell.spawn().unwrap());
        let target = process::of(game.0.id()).unwrap();
        let mut loaded = load(&target, &Image::unchecked(copy.to_str().unwrap()), "SymCleanup", &[0; 8]);
        // A process just made may not have mapped its modules yet.
        for _ in 0..100 {
            if loaded.is_ok() {
                break;
            }
            sleep(Duration::from_millis(100));
            loaded = load(&target, &Image::unchecked(copy.to_str().unwrap()), "SymCleanup", &[0; 8]);
        }
        sleep(Duration::from_millis(300));
        let alive = game.0.try_wait().unwrap().is_none();
        drop(game);
        fs::remove_file(&copy).ok();
        loaded.unwrap();
        assert!(alive, "the export crashed the process");
    }

    #[test]
    fn an_x86_image_never_enters_an_x64_process() {
        let me = process::of(std::process::id()).unwrap();
        let err = load(&me, &Image::unchecked_x86(KERNEL32_X86), "lstrlenW", &[]).unwrap_err();
        assert!(marked(&err, FailureKind::Architecture), "{err}");
    }
}
