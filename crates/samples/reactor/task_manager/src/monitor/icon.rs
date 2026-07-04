//! Extracts an executable's small icon as premultiplied BGRA8 pixels.
//!
//! Pure Win32/GDI; produces plain [`IconPixels`] with no WinUI dependency so the
//! UI layer can turn them into an image source on the UI thread.

use std::mem::size_of;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{
    BI_RGB, BITMAP, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, DeleteObject, GetDC, GetDIBits,
    GetObjectW, HBITMAP, HGDIOBJ, ReleaseDC,
};
use windows::Win32::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES;
use windows::Win32::UI::Shell::{SHFILEINFOW, SHGFI_ICON, SHGFI_SMALLICON, SHGetFileInfoW};
use windows::Win32::UI::WindowsAndMessaging::{DestroyIcon, GetIconInfo, HICON, ICONINFO};
use windows::core::PCWSTR;

use super::IconPixels;

/// Load the small icon for `path` and return its pixels, or `None` if the file
/// has no icon or extraction failed.
pub fn extract_icon_bgra(path: &str) -> Option<IconPixels> {
    let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();

    let mut sfi = SHFILEINFOW::default();
    let ok = unsafe {
        SHGetFileInfoW(
            PCWSTR(wide.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(&mut sfi),
            size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_SMALLICON,
        )
    };
    if ok == 0 || sfi.hIcon.is_invalid() {
        return None;
    }

    let pixels = unsafe { icon_to_bgra(sfi.hIcon) };
    unsafe {
        let _ = DestroyIcon(sfi.hIcon);
    }
    pixels
}

unsafe fn icon_to_bgra(icon: HICON) -> Option<IconPixels> {
    let mut info = ICONINFO::default();
    unsafe { GetIconInfo(icon, &mut info) }.ok()?;

    let color = info.hbmColor;
    // The mask bitmap is unused; free it once we have the colour bitmap.
    let cleanup_mask = info.hbmMask;

    let result = unsafe { color_bitmap_to_bgra(color) };

    unsafe {
        if !color.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(color.0));
        }
        if !cleanup_mask.is_invalid() {
            let _ = DeleteObject(HGDIOBJ(cleanup_mask.0));
        }
    }
    result
}

unsafe fn color_bitmap_to_bgra(bmp: HBITMAP) -> Option<IconPixels> {
    if bmp.is_invalid() {
        return None;
    }

    let mut bm = BITMAP::default();
    let wrote = unsafe {
        GetObjectW(
            HGDIOBJ(bmp.0),
            size_of::<BITMAP>() as i32,
            Some(&mut bm as *mut _ as *mut _),
        )
    };
    if wrote == 0 || bm.bmWidth <= 0 || bm.bmHeight <= 0 {
        return None;
    }

    let width = bm.bmWidth;
    let height = bm.bmHeight;

    let mut header = BITMAPINFO::default();
    header.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    header.bmiHeader.biWidth = width;
    // Negative height requests top-down rows, matching WriteableBitmap.
    header.bmiHeader.biHeight = -height;
    header.bmiHeader.biPlanes = 1;
    header.bmiHeader.biBitCount = 32;
    header.bmiHeader.biCompression = BI_RGB.0;

    let mut bgra = vec![0u8; (width * height * 4) as usize];

    let hdc = unsafe { GetDC(Some(HWND::default())) };
    let lines = unsafe {
        GetDIBits(
            hdc,
            bmp,
            0,
            height as u32,
            Some(bgra.as_mut_ptr() as *mut _),
            &mut header,
            DIB_RGB_COLORS,
        )
    };
    unsafe { ReleaseDC(Some(HWND::default()), hdc) };

    if lines == 0 {
        return None;
    }

    premultiply(&mut bgra);

    Some(IconPixels {
        width,
        height,
        bgra,
    })
}

/// Premultiply alpha in place. Icons whose colour bitmap carries no alpha
/// channel (every alpha byte zero) are treated as fully opaque.
fn premultiply(bgra: &mut [u8]) {
    let has_alpha = bgra.chunks_exact(4).any(|px| px[3] != 0);
    for px in bgra.chunks_exact_mut(4) {
        let a = if has_alpha { px[3] as u32 } else { 255 };
        px[0] = (px[0] as u32 * a / 255) as u8;
        px[1] = (px[1] as u32 * a / 255) as u8;
        px[2] = (px[2] as u32 * a / 255) as u8;
        px[3] = a as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn premultiply_opaque_when_no_alpha() {
        // All alpha bytes zero -> treated as opaque, colours preserved.
        let mut px = vec![10, 20, 30, 0];
        premultiply(&mut px);
        assert_eq!(px, vec![10, 20, 30, 255]);
    }

    #[test]
    fn premultiply_scales_by_alpha() {
        let mut px = vec![100, 200, 255, 128];
        premultiply(&mut px);
        assert_eq!(px[3], 128);
        assert_eq!(px[0], (100 * 128 / 255) as u8);
        assert_eq!(px[2], (255 * 128 / 255) as u8);
    }
}
