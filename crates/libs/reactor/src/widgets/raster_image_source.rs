use super::*;

/// An in-memory image built from raw pixels, displayed by handing it to
/// [`Image::new`](crate::Image::new). Unlike [`SurfaceImageSource`], it needs no
/// Direct2D device and nothing is written to disk — the pixels are copied
/// straight into a WinUI `WriteableBitmap`.
///
/// Create it on the UI thread (for example inside your render function).
#[derive(Clone, PartialEq, Debug)]
pub struct RasterImageSource {
    source: bindings::WriteableBitmap,
}

impl RasterImageSource {
    /// Build from premultiplied BGRA8 pixels, row-major, top-down. `bgra` must
    /// hold `width * height * 4` bytes; extra bytes are ignored and a short
    /// buffer leaves the remainder transparent.
    pub fn from_bgra8(width: i32, height: i32, bgra: &[u8]) -> Result<Self> {
        let source = bindings::WriteableBitmap::CreateInstanceWithDimensions(width, height)?;
        let capacity = width.max(0) as usize * height.max(0) as usize * 4;
        let access = source.PixelBuffer()?.cast::<bindings::IBufferByteAccess>()?;
        unsafe {
            let dst = access.Buffer()?;
            let n = bgra.len().min(capacity);
            core::ptr::copy_nonoverlapping(bgra.as_ptr(), dst, n);
        }
        Ok(Self { source })
    }

    /// Cast the underlying bitmap to the `ImageSource` the backend assigns to
    /// `Image.Source`.
    pub fn image_source(&self) -> Result<bindings::ImageSource> {
        self.source.cast()
    }
}
