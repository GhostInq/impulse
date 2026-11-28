//! GPU to CPU: the frame's texture copied through a staging texture into tight rows.

use anyhow::{Context, Result, bail};
use windows::Graphics::Capture::Direct3D11CaptureFrame;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_BOX, D3D11_CPU_ACCESS_READ, D3D11_MAP_READ, D3D11_MAPPED_SUBRESOURCE, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING, ID3D11Device,
    ID3D11DeviceContext, ID3D11Texture2D,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use windows::Win32::System::WinRT::Direct3D11::IDirect3DDxgiInterfaceAccess;
use windows::core::Interface;

use crate::Shot;

pub fn read(device: &ID3D11Device, context: &ID3D11DeviceContext, frame: &Direct3D11CaptureFrame) -> Result<Shot> {
    let surface = frame.Surface().context("get the frame surface")?;
    let access: IDirect3DDxgiInterfaceAccess = surface.cast().context("reach the DXGI surface")?;
    // SAFETY: the surface of a capture frame is backed by a D3D11 texture.
    let texture: ID3D11Texture2D = unsafe { access.GetInterface() }.context("get the frame texture")?;
    let mut desc = D3D11_TEXTURE2D_DESC::default();
    // SAFETY: desc is a valid out pointer for the call.
    unsafe { texture.GetDesc(&mut desc) };

    // The texture can be larger than the window's content.
    let content = frame.ContentSize().context("read the content size")?;
    let width = (content.Width.max(0) as u32).min(desc.Width);
    let height = (content.Height.max(0) as u32).min(desc.Height);
    if width == 0 || height == 0 {
        bail!("the frame is empty");
    }

    let staging_desc = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: desc.Format,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: D3D11_USAGE_STAGING,
        BindFlags: 0,
        CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32,
        MiscFlags: 0,
    };
    let mut staging = None;
    // SAFETY: staging_desc is valid and the out pointer outlives the call.
    unsafe { device.CreateTexture2D(&staging_desc, None, Some(&mut staging)) }.context("create the staging texture")?;
    let staging = staging.context("create the staging texture")?;

    let region = D3D11_BOX { left: 0, top: 0, front: 0, right: width, bottom: height, back: 1 };
    // SAFETY: both textures are live on this device and the box lies inside the source.
    unsafe { context.CopySubresourceRegion(&staging, 0, 0, 0, 0, &texture, 0, Some(&region)) };

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    // SAFETY: staging has CPU read access and is not mapped yet; mapped is a valid out pointer.
    unsafe { context.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)) }.context("map the staging texture")?;
    let bgra = tight_rows(&mapped, width as usize, height as usize);
    // SAFETY: pairs with the successful Map above.
    unsafe { context.Unmap(&staging, 0) };

    Ok(Shot { width, height, bgra })
}

// Infallible, so the caller always reaches Unmap.
fn tight_rows(mapped: &D3D11_MAPPED_SUBRESOURCE, width: usize, height: usize) -> Vec<u8> {
    let row = width * 4;
    let pitch = mapped.RowPitch as usize;
    // The last row has no padding after it.
    let len = pitch * (height - 1) + row;
    // SAFETY: a mapped subresource spans RowPitch bytes per row but the last, valid until Unmap.
    let data = unsafe { std::slice::from_raw_parts(mapped.pData as *const u8, len) };
    let mut out = Vec::with_capacity(row * height);
    for line in data.chunks(pitch).take(height) {
        out.extend_from_slice(&line[..row]);
    }
    out
}
