---
paths:
  - "crates/agent/**"
---

# agent

The game's process: find it, check what may enter it, load a DLL into it, start an export. Windows, x64 and x86 (WOW64) targets.

## Surface
- `find(&Rule)`: the matching process that is old enough, the one with the biggest working set (blocking). Age and working set never leave the crate.
- `wait(&Rule, within)`: polls `find` once a second; needs no runtime; dropping it stops within one poll. Its timeout is marked `GameNotFound`.
- `of(pid)`: the identity of one process now (blocking), `None` when it cannot be looked at.
- `Image::open(path, publishers)`, `Image::path`, `Image::x64` (else x86): the only way to name a DLL that enters a game.
- `signed(path, publishers)`: the signature check alone (blocking), for a program that runs on its own and never enters a game.
- `load(&Process, &Image, export, arg)`: runs `LoadLibraryW` in the target, then starts `export` with a copy of `arg`.
- `elevated()`: this process runs as administrator (`app` marks it in the title bar).
- `serve_elevated(publishers)`: first thing in the host's `main`. `Some(exit code)` when the command line asks for an elevated load; else makes this executable the one a denied `load` starts elevated.
- `Image::unchecked`: tests only.

## Rules
- A process is its `Process` (pid, image path, creation time), never a pid alone: a reused pid has another time. Every remote operation first opens a handle and calls `process::verify` on it. `load` does it before any remote step, then refuses an image of the other architecture; new operations do the same.
- A DLL enters a game only as an `Image`: a `.dll` (`LoadLibraryW` appends the extension and would load a sibling), Authenticode-signed with one of `publishers` as the signer's display name, x64 or x86, held open without write or delete sharing while the value lives. Its path is the opened file's final path on a local drive. No constructor from a bare path.
- A signer certificate that vouches for itself (a chain of one) is refused. Windows' policy refuses it for its untrusted root already; Wine's `WinVerifyTrust` passes it. Wine is not held to a trusted root: it lacks roots Windows trusts for code signing (DigiCert CS RSA4096 Root G5), so under Wine the check is the signature, the signer's name and an issuer.
- Revocation is not checked: it needs the network and a trainer must start offline.
- After a loader timeout `load` leaves the remote path buffer and keeps the file lock until exit (`keep_locked`): the remote thread may still open the file. The argument copy stays in the target too: the thread reads it after `load` returns.
- No handle (`Handle`, `HANDLE`) crosses the interface: only data (`Process`, `Image`, bytes, strings), so it can later run as a helper in a Proton prefix.
- `Image::unchecked` exists only under `cfg(test)` or the `test-support` feature, which only dev-dependencies enable.
- Protected processes that cannot be opened are skipped by `find`, not errors.
- A process that denies `load` (a game run as administrator) is loaded by the host executable started elevated through UAC (`runas`, the request in hex on its command line), only when a host is set and this process is not elevated. The copy checks the image against the host's publishers, never the request's choice, and the process, like any load.
- Marks: a declined prompt is `Elevation`, denied even elevated `Protected`, as is a denial once the handle is open (allocation, write, remote thread: the game's protection stripped its rights, elevation does not lift that), an image of the other architecture `Architecture`. A failure after the copy may have started its loader keeps the parent's image lock: the copy's ends with it.
- `find`, `of`, `Image::open` and `load` block: callers use `blocking::unblock`.

## Traps
- The module is found by its full path through `EnumProcessModulesEx`. A Toolhelp snapshot fails when any module path exceeds MAX_PATH. The remote thread's exit code is only half the module handle.
- `LoadLibraryW`'s address is ours in an x64 target: kernel32 maps at one address in every x64 process of a boot. A WOW64 target's is its 32-bit kernel32's base plus the RVA in the SysWOW64 file; that module is recorded under System32 (redirected), so it is found by file name.
- Image paths compare case-insensitively; creation time is in 100 ns ticks.
- `ShellExecuteExW` with `runas` returns once the prompt is answered; `ERROR_CANCELLED` is a declined one.

## New code
- Memory operations go in `agent::memory` (plan phase 5): data in, data out, identity-checked like `load`.
