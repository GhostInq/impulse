//! The capture session: own D3D11 device, item, pool, and the wait for the first frame.

use std::sync::mpsc;
use std::time::Duration;

use anyhow::{Context, Result, anyhow, bail};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession};
use windows::Graphics::DirectX::Direct3D11::IDirect3DDevice;
use windows::Graphics::DirectX::DirectXPixelFormat;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::System::WinRT::Direct3D11::CreateDirect3D11DeviceFromDXGIDevice;
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;
use windows::core::{Interface, factory};

use crate::{Shot, readback};

// Closes what a capture opened, whichever way the call ends.
struct Live {
    pool: Direct3D11CaptureFramePool,
    session: Option<GraphicsCaptureSession>,
    token: Option<i64>,
}

impl Drop for Live {
    fn drop(&mut self) {
        if let Some(token) = self.token {
            let _ = self.pool.RemoveFrameArrived(token);
        }
        if let Some(session) = &self.session {
            let _ = session.Close();
        }
        let _ = self.pool.Close();
    }
}

pub fn first_frame(hwnd: HWND, within: Duration) -> Result<Shot> {
    let (device, context, winrt_device) = create_device()?;

    let interop = factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>().context("get the capture item factory")?;
    // SAFETY: hwnd is only read by the call; a stale handle comes back as an error.
    let item: GraphicsCaptureItem = unsafe { interop.CreateForWindow(hwnd) }.context("create the capture item")?;
    let size = item.Size().context("read the item size")?;
    if size.Width <= 0 || size.Height <= 0 {
        bail!("the window has no size");
    }

    let pool = Direct3D11CaptureFramePool::CreateFreeThreaded(&winrt_device, DirectXPixelFormat::B8G8R8A8UIntNormalized, 1, size)
        .context("create the frame pool")?;
    let mut live = Live { pool, session: None, token: None };

    let (tx, rx) = mpsc::channel();
    let handler = TypedEventHandler::new(move |_, _| {
        // The receiver is gone once the call has ended.
        let _ = tx.send(());
        Ok(())
    });
    live.token = Some(live.pool.FrameArrived(&handler).context("watch for frames")?);

    let session = live.pool.CreateCaptureSession(&item).context("create the capture session")?;
    let session = live.session.insert(session);
    // Older Windows lack these switches (the border one may want access asked first): the cursor or border stays.
    if let Err(e) = session.SetIsCursorCaptureEnabled(false) {
        log::debug!("capture cursor stays: {e}");
    }
    if let Err(e) = session.SetIsBorderRequired(false) {
        log::debug!("capture border stays: {e}");
    }
    session.StartCapture().context("start the capture")?;

    rx.recv_timeout(within).map_err(|_| anyhow!("no frame within {within:?}"))?;
    let frame = live.pool.TryGetNextFrame().context("take the frame")?;
    let shot = readback::read(&device, &context, &frame);
    let _ = frame.Close();
    shot
}

// A device of our own: the caller's is never touched from this thread.
fn create_device() -> Result<(ID3D11Device, ID3D11DeviceContext, IDirect3DDevice)> {
    let mut device = None;
    let mut context = None;
    // SAFETY: the out pointers outlive the call; no adapter and no feature levels mean defaults.
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            None,
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut context),
        )
    }
    .context("create the D3D11 device")?;
    let device = device.context("create the D3D11 device")?;
    let context = context.context("get the device context")?;

    let dxgi: IDXGIDevice = device.cast().context("get the DXGI device")?;
    // SAFETY: dxgi is a live DXGI device of the device just created.
    let winrt = unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi) }.context("wrap the device for WinRT")?;
    let winrt = winrt.cast().context("get the WinRT device")?;
    Ok((device, context, winrt))
}
