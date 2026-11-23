//! Authenticode: an image reaches a game process only signed by a publisher its caller expects.

use std::{os::windows::ffi::OsStrExt, path::Path, ptr};

use anyhow::{Result, bail};
use windows_sys::Win32::{
    Foundation::HANDLE,
    Security::{
        Cryptography::{
            CERT_NAME_SIMPLE_DISPLAY_TYPE, CERT_TRUST_IS_NOT_SIGNATURE_VALID, CERT_TRUST_IS_PARTIAL_CHAIN, CERT_TRUST_IS_UNTRUSTED_ROOT,
            CertGetNameStringW,
        },
        WinTrust::{
            WINTRUST_ACTION_GENERIC_VERIFY_V2, WINTRUST_DATA, WINTRUST_DATA_0, WINTRUST_FILE_INFO, WTD_CACHE_ONLY_URL_RETRIEVAL,
            WTD_CHOICE_FILE, WTD_REVOKE_NONE, WTD_STATEACTION_CLOSE, WTD_STATEACTION_VERIFY, WTD_UI_NONE, WTHelperGetProvSignerFromChain,
            WTHelperProvDataFromStateData, WinVerifyTrust,
        },
    },
};

/// A chain with any of these ends nowhere trusted. Windows' policy already refuses them; Wine's `WinVerifyTrust` passes a
/// self-signed signer, though its chain says so.
const UNTRUSTED: u32 = CERT_TRUST_IS_UNTRUSTED_ROOT | CERT_TRUST_IS_PARTIAL_CHAIN | CERT_TRUST_IS_NOT_SIGNATURE_VALID;

/// Fails unless `path` has a valid embedded signature, chained to a trusted root, whose signer's display name is one of
/// `publishers`. Revocation is not checked: it needs the network, and a trainer must start offline.
pub fn check(path: &Path, publishers: &[&str]) -> Result<()> {
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain([0]).collect();
    let mut file =
        WINTRUST_FILE_INFO { cbStruct: size_of::<WINTRUST_FILE_INFO>() as u32, pcwszFilePath: wide.as_ptr(), ..Default::default() };
    let mut data = WINTRUST_DATA {
        cbStruct: size_of::<WINTRUST_DATA>() as u32,
        dwUIChoice: WTD_UI_NONE,
        fdwRevocationChecks: WTD_REVOKE_NONE,
        dwUnionChoice: WTD_CHOICE_FILE,
        Anonymous: WINTRUST_DATA_0 { pFile: &mut file },
        dwStateAction: WTD_STATEACTION_VERIFY,
        dwProvFlags: WTD_CACHE_ONLY_URL_RETRIEVAL,
        ..Default::default()
    };
    let mut action = WINTRUST_ACTION_GENERIC_VERIFY_V2;
    // SAFETY: `data` and `file` outlive both calls; the state is released by the close action.
    let (status, signer) = unsafe {
        let status = WinVerifyTrust(ptr::null_mut(), &mut action, (&raw mut data).cast());
        let signer = signer(data.hWVTStateData);
        data.dwStateAction = WTD_STATEACTION_CLOSE;
        WinVerifyTrust(ptr::null_mut(), &mut action, (&raw mut data).cast());
        (status, signer)
    };
    match (status, signer) {
        (0, Some(Signer { chain, .. })) if chain & UNTRUSTED != 0 => {
            bail!("{} is signed by a certificate no trusted root vouches for ({chain:#x})", path.display())
        }
        (0, Some(Signer { name: Some(name), .. })) if publishers.contains(&name.as_str()) => Ok(()),
        (0, signer) => {
            let name = signer.and_then(|signer| signer.name);
            let signer = name.as_deref().unwrap_or("an unknown publisher");
            bail!("{} is signed by {signer}, not by {}", path.display(), publishers.join(" or "))
        }
        (code, _) => bail!("{} has no valid signature ({code:#x})", path.display()),
    }
}

struct Signer {
    /// The leaf certificate's display name.
    name: Option<String>,
    /// The error bits of the signer's chain (`CERT_TRUST_*`); none when the chain is unknown.
    chain: u32,
}

unsafe fn signer(state: HANDLE) -> Option<Signer> {
    // SAFETY: the pointers come from WinTrust's own verification state and are checked for null.
    unsafe {
        let data = WTHelperProvDataFromStateData(state);
        if data.is_null() {
            return None;
        }
        let signer = WTHelperGetProvSignerFromChain(data, 0, 0, 0);
        if signer.is_null() {
            return None;
        }
        let context = (*signer).pChainContext;
        let chain = if context.is_null() { 0 } else { (*context).TrustStatus.dwErrorStatus };
        if (*signer).csCertChain == 0 {
            return Some(Signer { name: None, chain });
        }
        let cert = (*(*signer).pasCertChain).pCert;
        let mut name = [0u16; 256];
        let len = CertGetNameStringW(cert, CERT_NAME_SIMPLE_DISPLAY_TYPE, 0, ptr::null(), name.as_mut_ptr(), name.len() as u32) as usize;
        Some(Signer { name: (len > 1).then(|| String::from_utf16_lossy(&name[..len - 1])), chain })
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    /// Hand-run, chiefly under Wine, whose `WinVerifyTrust` builds the chain from its own root store: `IMPULSE_E2E_SIGNED`
    /// names a signed file and `IMPULSE_E2E_SIGNER` the display name of its signer; `IMPULSE_E2E_SELF_SIGNED` optionally
    /// names a file signed by a self-signed certificate of that name, which must be refused.
    #[test]
    #[ignore = "needs IMPULSE_E2E_SIGNED and IMPULSE_E2E_SIGNER"]
    fn a_named_file_is_signed_by_the_named_publisher() {
        let path = env::var("IMPULSE_E2E_SIGNED").expect("IMPULSE_E2E_SIGNED");
        let signer = env::var("IMPULSE_E2E_SIGNER").expect("IMPULSE_E2E_SIGNER");
        check(Path::new(&path), &[signer.as_str()]).unwrap();
        assert!(check(Path::new(&path), &["Someone Else"]).is_err());
        if let Ok(forged) = env::var("IMPULSE_E2E_SELF_SIGNED") {
            let err = check(Path::new(&forged), &[signer.as_str()]).unwrap_err();
            assert!(err.to_string().contains("no trusted root"), "{err}");
        }
    }
}
