//! Image Spatial Extents Box implementation.

use crate::error::Result;
use crate::io::ReadStream;

/// Image spatial extents box.
#[derive(Debug, Clone)]
pub struct ImageSpatialExtentsBox {
    /// Image width.
    pub width: u32,
    /// Image height.
    pub height: u32,
}

impl ImageSpatialExtentsBox {
    /// Parse an image spatial extents box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Skip version and flags (4 bytes)
        stream.skip(4)?;
        
        // Read width and height
        let width = stream.read_u32()?;
        let height = stream.read_u32()?;
        
        Ok(Self {
            width,
            height,
        })
    }
}
