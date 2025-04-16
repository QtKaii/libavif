//! AVIF image representation for the Rust implementation of libavif.

use crate::error::{Error, Result};
use crate::memory::RwData;

/// AVIF pixel format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// YUV format with 4:4:4 chroma subsampling.
    Yuv444,
    /// YUV format with 4:2:2 chroma subsampling.
    Yuv422,
    /// YUV format with 4:2:0 chroma subsampling.
    Yuv420,
    /// Monochrome format.
    Monochrome,
}

/// AVIF color range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorRange {
    /// Limited range (16-235 for 8-bit).
    Limited,
    /// Full range (0-255 for 8-bit).
    Full,
}

/// AVIF chroma sample position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromaSamplePosition {
    /// Unknown chroma sample position.
    Unknown,
    /// Horizontally co-located with luma samples, vertically centered between luma samples.
    Vertical,
    /// Co-located with luma samples.
    Colocated,
}

/// AVIF image plane.
#[derive(Debug)]
pub struct Plane {
    /// Plane data.
    pub data: Vec<u8>,
    /// Plane row bytes.
    pub row_bytes: usize,
    /// Plane width.
    pub width: usize,
    /// Plane height.
    pub height: usize,
}

impl Plane {
    /// Create a new empty plane.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            row_bytes: 0,
            width: 0,
            height: 0,
        }
    }

    /// Allocate plane data.
    pub fn allocate(&mut self, width: usize, height: usize, depth: usize) -> Result<()> {
        self.width = width;
        self.height = height;
        let bytes_per_pixel = if depth > 8 { 2 } else { 1 };
        self.row_bytes = width * bytes_per_pixel;
        self.data = vec![0; self.row_bytes * height];
        Ok(())
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

/// AVIF image.
#[derive(Debug)]
pub struct AvifImage {
    /// Image width.
    pub width: u32,
    /// Image height.
    pub height: u32,
    /// Image depth.
    pub depth: u32,
    /// YUV format.
    pub yuv_format: PixelFormat,
    /// YUV range.
    pub yuv_range: ColorRange,
    /// YUV chroma sample position.
    pub yuv_chroma_sample_position: ChromaSamplePosition,
    /// Y plane.
    pub y_plane: Option<Plane>,
    /// U plane.
    pub u_plane: Option<Plane>,
    /// V plane.
    pub v_plane: Option<Plane>,
    /// Alpha plane.
    pub alpha_plane: Option<Plane>,
    /// Whether alpha is premultiplied.
    pub alpha_premultiplied: bool,
    /// ICC profile.
    pub icc: RwData,
    /// EXIF metadata.
    pub exif: RwData,
    /// XMP metadata.
    pub xmp: RwData,
}

impl AvifImage {
    /// Create a new empty image.
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 8,
            yuv_format: PixelFormat::Yuv420,
            yuv_range: ColorRange::Full,
            yuv_chroma_sample_position: ChromaSamplePosition::Unknown,
            y_plane: None,
            u_plane: None,
            v_plane: None,
            alpha_plane: None,
            alpha_premultiplied: false,
            icc: RwData::new(),
            exif: RwData::new(),
            xmp: RwData::new(),
        }
    }

    /// Create a new image with the given dimensions and format.
    pub fn with_params(width: u32, height: u32, depth: u32, yuv_format: PixelFormat) -> Result<Self> {
        if width == 0 || height == 0 {
            return Err(Error::InvalidParameter(
                "Width and height must be non-zero".to_string(),
            ));
        }
        if depth == 0 || depth > 16 {
            return Err(Error::InvalidParameter(
                "Depth must be between 1 and 16".to_string(),
            ));
        }

        Ok(Self {
            width,
            height,
            depth,
            yuv_format,
            yuv_range: ColorRange::Full,
            yuv_chroma_sample_position: ChromaSamplePosition::Unknown,
            y_plane: None,
            u_plane: None,
            v_plane: None,
            alpha_plane: None,
            alpha_premultiplied: false,
            icc: RwData::new(),
            exif: RwData::new(),
            xmp: RwData::new(),
        })
    }

    /// Allocate planes for the image.
    pub fn allocate_planes(&mut self, planes_mask: u32) -> Result<()> {
        let has_color = (planes_mask & 0x7) != 0;
        let has_alpha = (planes_mask & 0x8) != 0;

        if has_color {
            // Allocate Y plane
            let mut y_plane = Plane::new();
            y_plane.allocate(self.width as usize, self.height as usize, self.depth as usize)?;
            self.y_plane = Some(y_plane);

            // Allocate UV planes if not monochrome
            if self.yuv_format != PixelFormat::Monochrome {
                let (uv_width, uv_height) = match self.yuv_format {
                    PixelFormat::Yuv444 => (self.width, self.height),
                    PixelFormat::Yuv422 => (self.width / 2, self.height),
                    PixelFormat::Yuv420 => (self.width / 2, self.height / 2),
                    PixelFormat::Monochrome => unreachable!(),
                };

                let mut u_plane = Plane::new();
                u_plane.allocate(
                    uv_width as usize,
                    uv_height as usize,
                    self.depth as usize,
                )?;
                self.u_plane = Some(u_plane);

                let mut v_plane = Plane::new();
                v_plane.allocate(
                    uv_width as usize,
                    uv_height as usize,
                    self.depth as usize,
                )?;
                self.v_plane = Some(v_plane);
            }
        }

        if has_alpha {
            let mut alpha_plane = Plane::new();
            alpha_plane.allocate(
                self.width as usize,
                self.height as usize,
                self.depth as usize,
            )?;
            self.alpha_plane = Some(alpha_plane);
        }

        Ok(())
    }

    /// Set the ICC profile.
    pub fn set_icc_profile(&mut self, icc: &[u8]) -> Result<()> {
        self.icc = RwData::from_slice(icc);
        Ok(())
    }

    /// Set the EXIF metadata.
    pub fn set_exif(&mut self, exif: &[u8]) -> Result<()> {
        self.exif = RwData::from_slice(exif);
        Ok(())
    }

    /// Set the XMP metadata.
    pub fn set_xmp(&mut self, xmp: &[u8]) -> Result<()> {
        self.xmp = RwData::from_slice(xmp);
        Ok(())
    }
}

impl Default for AvifImage {
    fn default() -> Self {
        Self::new()
    }
}
