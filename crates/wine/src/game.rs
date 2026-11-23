//! The game Steam asked Proton to start, when the launch options put Impulse in its place:
//! `proton waitforexitandrun Impulse.exe --game <game.exe> <args>`. Proton turned only Impulse's own path into a
//! Windows one and waits for Impulse, so Impulse starts the game as Proton would have and lives as long as it.

use std::{
    ffi::c_void,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::PathBuf,
    ptr, slice,
};

use anyhow::{Result, bail};
use windows_sys::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Environment::GetCommandLineW,
        IO::{CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED},
        JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JOBOBJECT_ASSOCIATE_COMPLETION_PORT, JobObjectAssociateCompletionPortInformation,
            SetInformationJobObject,
        },
        SystemServices::JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO,
        Threading::{
            CREATE_NEW_CONSOLE, CREATE_SUSPENDED, CREATE_UNICODE_ENVIRONMENT, GetExitCodeProcess, INFINITE, PROCESS_INFORMATION,
            ResumeThread, STARTUPINFOW, WaitForSingleObject,
        },
    },
};

use crate::path::dos_path;

/// Put before the game's command by the Linux launcher; the first argument when present.
pub const FLAG: &str = "--game";
const CONSOLE: u16 = 3;

/// The game's command as Proton gave it: its executable, then the rest of the line exactly as written.
#[derive(Debug, PartialEq)]
pub struct Command {
    exe: String,
    rest: Vec<u16>,
}

/// The game to start when Impulse's first argument is `--game`, else `None`.
pub fn command() -> Option<Command> {
    // SAFETY: the process's command line: NUL-terminated and valid for the process's life.
    let line = unsafe { GetCommandLineW() };
    if line.is_null() {
        return None;
    }
    // SAFETY: as above, so the scan and the slice stay inside it.
    let line = unsafe { slice::from_raw_parts(line, (0..).take_while(|&i| *line.add(i) != 0).count()) };
    parse(line)
}

fn parse(line: &[u16]) -> Option<Command> {
    // Our own path first, then the flag.
    let (_, rest) = token(line)?;
    let rest = rest.strip_prefix(&wide(FLAG)[..])?;
    if !rest.first().is_some_and(|&c| is_blank(c)) {
        return None;
    }
    let (exe, rest) = token(rest)?;
    Some(Command { exe: String::from_utf16(exe).ok()?, rest: rest.to_vec() })
}

/// The first argument after leading blanks, without its quotes, and what follows it. As Proton reads one:
/// a quote runs to the next quote, anything else to the next blank.
fn token(line: &[u16]) -> Option<(&[u16], &[u16])> {
    let line = &line[line.iter().position(|&c| !is_blank(c))?..];
    match line.split_first() {
        Some((&QUOTE, inside)) => {
            let end = inside.iter().position(|&c| c == QUOTE)?;
            let after = &inside[end + 1..];
            Some((&inside[..end], &after[after.iter().position(|&c| !is_blank(c)).unwrap_or(after.len())..]))
        }
        _ => {
            let end = line.iter().position(|&c| is_blank(c)).unwrap_or(line.len());
            let after = &line[end..];
            Some((&line[..end], &after[after.iter().position(|&c| !is_blank(c)).unwrap_or(after.len())..]))
        }
    }
}

const QUOTE: u16 = b'"' as u16;

fn is_blank(c: u16) -> bool {
    c == u16::from(b' ') || c == u16::from(b'\t')
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

impl Command {
    /// The executable as Windows names it: a Unix path from Steam goes through Wine.
    pub fn exe(&self) -> PathBuf {
        if self.exe.starts_with('/') { dos_path(&self.exe).unwrap_or_else(|| self.exe.clone().into()) } else { self.exe.clone().into() }
    }

    /// The line the game gets: its executable quoted, then the rest as Proton passed it.
    fn line(&self) -> Vec<u16> {
        let exe = self.exe();
        let mut line = wide("\"");
        line.extend(exe.to_string_lossy().encode_utf16());
        line.push(QUOTE);
        if !self.rest.is_empty() {
            line.push(u16::from(b' '));
            line.extend_from_slice(&self.rest);
        }
        line.push(0);
        line
    }

    /// Starts the game in this process's folder and environment, as Proton does, inside a job of its own so
    /// that `wait` sees every process it starts end too (a launcher that starts the game and exits).
    pub fn start(&self) -> Result<Game> {
        let exe = self.exe();
        let mut flags = CREATE_UNICODE_ENVIRONMENT | CREATE_SUSPENDED;
        // Proton gives a console program its own console.
        if subsystem(&exe).is_some_and(|subsystem| subsystem == CONSOLE) {
            flags |= CREATE_NEW_CONSOLE;
        }
        let mut line = self.line();
        let startup = STARTUPINFOW { cb: size_of::<STARTUPINFOW>() as u32, ..Default::default() };
        let mut started = PROCESS_INFORMATION::default();
        // SAFETY: `line` is a writable NUL-terminated buffer and the structures outlive the call.
        let ok = unsafe {
            windows_sys::Win32::System::Threading::CreateProcessW(
                ptr::null(),
                line.as_mut_ptr(),
                ptr::null(),
                ptr::null(),
                0,
                flags,
                ptr::null(),
                ptr::null(),
                &startup,
                &mut started,
            )
        };
        if ok == 0 {
            bail!("cannot start {}: {}", exe.display(), io::Error::last_os_error());
        }
        let (process, thread) = (Handle(started.hProcess), Handle(started.hThread));
        // Without the job only the first process is waited for: a launcher's exit ends the wait early.
        let job = job(&process).inspect_err(|e| log::warn!("only the game's first process is followed: {e:#}")).ok();
        // SAFETY: the thread handle is live and the thread was made suspended.
        if unsafe { ResumeThread(thread.0) } == u32::MAX {
            bail!("cannot resume {}: {}", exe.display(), io::Error::last_os_error());
        }
        Ok(Game { process, job, pid: started.dwProcessId })
    }
}

/// A job holding the process, with a port that says when its last process ends.
fn job(process: &Handle) -> Result<(Handle, Handle)> {
    // SAFETY: plain Win32 calls; every handle is owned by a guard before the next call.
    unsafe {
        let job = Handle(CreateJobObjectW(ptr::null(), ptr::null()));
        if job.0.is_null() {
            bail!("cannot make a job: {}", io::Error::last_os_error());
        }
        let port = Handle(CreateIoCompletionPort(INVALID_HANDLE_VALUE, ptr::null_mut(), 0, 1));
        if port.0.is_null() {
            bail!("cannot make a completion port: {}", io::Error::last_os_error());
        }
        let link = JOBOBJECT_ASSOCIATE_COMPLETION_PORT { CompletionKey: job.0, CompletionPort: port.0 };
        let info: *const c_void = (&raw const link).cast();
        if SetInformationJobObject(job.0, JobObjectAssociateCompletionPortInformation, info, size_of_val(&link) as u32) == 0 {
            bail!("cannot watch the job: {}", io::Error::last_os_error());
        }
        if AssignProcessToJobObject(job.0, process.0) == 0 {
            bail!("cannot put the game in a job: {}", io::Error::last_os_error());
        }
        Ok((job, port))
    }
}

/// The PE subsystem of an executable: 2 for a window, 3 for a console.
fn subsystem(exe: &std::path::Path) -> Option<u16> {
    let mut file = File::open(exe).ok()?;
    let mut dos = [0u8; 64];
    file.read_exact(&mut dos).ok()?;
    if &dos[..2] != b"MZ" {
        return None;
    }
    let pe = u32::from_le_bytes(dos[60..64].try_into().ok()?);
    // The signature (4) and the file header (20), then the optional header, whose subsystem is at 68 in both formats.
    file.seek(SeekFrom::Start(u64::from(pe) + 24 + 68)).ok()?;
    let mut field = [0u8; 2];
    file.read_exact(&mut field).ok()?;
    Some(u16::from_le_bytes(field))
}

/// The game Impulse started for Proton.
pub struct Game {
    process: Handle,
    job: Option<(Handle, Handle)>,
    pid: u32,
}

impl Game {
    /// The id of the game's first process.
    pub fn pid(&self) -> u32 {
        self.pid
    }

    /// Whether the game's whole process tree is followed; else only its first process.
    pub fn followed(&self) -> bool {
        self.job.is_some()
    }

    /// Blocks until the game, and every process it started, has ended. The first process's exit code: what
    /// Proton hands back to Steam.
    pub fn wait(&self) -> u32 {
        if let Some((_, port)) = &self.job {
            loop {
                let (mut message, mut key, mut overlapped) = (0u32, 0usize, ptr::null_mut::<OVERLAPPED>());
                // SAFETY: the port is live and the outputs outlive the call.
                let ok = unsafe { GetQueuedCompletionStatus(port.0, &mut message, &mut key, &mut overlapped, INFINITE) };
                // A failed port leaves the first process to wait for.
                if ok == 0 || message == JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO {
                    break;
                }
            }
        }
        // SAFETY: the process handle is live.
        unsafe { WaitForSingleObject(self.process.0, INFINITE) };
        let mut code = 1;
        // SAFETY: as above; `code` outlives the call.
        unsafe { GetExitCodeProcess(self.process.0, &mut code) };
        code
    }
}

/// A kernel handle, closed on drop.
struct Handle(HANDLE);

// SAFETY: a kernel handle may be used and closed from any thread.
unsafe impl Send for Handle {}
// SAFETY: as above; the calls made through a shared one (waits, queries) are thread-safe.
unsafe impl Sync for Handle {}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.0.is_null() && self.0 != INVALID_HANDLE_VALUE {
            // SAFETY: the handle is owned by this guard and closed once.
            unsafe { CloseHandle(self.0) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn command(line: &str) -> Option<Command> {
        parse(&wide(line))
    }

    #[test]
    fn the_game_follows_the_flag_and_the_rest_stays_as_written() {
        let got = command(r#""Z:\games\Impulse.exe" --game /home/me/Game Dir/game.exe -windowed  "a b""#).unwrap();
        assert_eq!(got.exe, "/home/me/Game");
        assert_eq!(String::from_utf16(&got.rest).unwrap(), r#"Dir/game.exe -windowed  "a b""#);
        let got = command(r#"Impulse.exe --game "/home/me/Game Dir/game.exe" -x"#).unwrap();
        assert_eq!((got.exe.as_str(), String::from_utf16(&got.rest).unwrap().as_str()), ("/home/me/Game Dir/game.exe", "-x"));
        let bare = command(r#"Impulse.exe --game C:\g.exe"#).unwrap();
        assert_eq!((bare.exe.as_str(), bare.rest.len()), (r"C:\g.exe", 0));
    }

    #[test]
    fn without_the_flag_first_there_is_no_game() {
        assert_eq!(command(r#""C:\Impulse.exe""#), None);
        assert_eq!(command(r#""C:\Impulse.exe" --restarted 12 --game x.exe"#), None);
        assert_eq!(command(r#""C:\Impulse.exe" --gamex x.exe"#), None);
        assert_eq!(command(r#""C:\Impulse.exe" --game"#), None);
        assert_eq!(command(r#""C:\Impulse.exe" --game "unclosed"#), None);
    }

    #[test]
    fn the_games_line_quotes_its_executable_and_keeps_the_rest() {
        let got = command(r#"Impulse.exe --game "C:\My Game\g.exe" -a "b c""#).unwrap();
        let line = got.line();
        assert_eq!(String::from_utf16(&line[..line.len() - 1]).unwrap(), r#""C:\My Game\g.exe" -a "b c""#);
    }

    #[test]
    fn a_console_program_is_told_from_a_window_one() {
        let me = std::env::current_exe().unwrap();
        // Test binaries are console programs.
        assert_eq!(subsystem(&me), Some(CONSOLE));
        assert_eq!(subsystem(std::path::Path::new(r"C:\no\such.exe")), None);
    }
}
