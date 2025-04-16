//! Pixel Information Box implementation.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Pixel information box.
#[derive(Debug, Clone)]
pub struct PixelInformationBox {
    /// Bits per channel.
    pub bits_per_channel: Vec<u8>,
}

impl PixelInformationBox {
    /// Parse a pixel information box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Skip version and flags (4 bytes)
        stream.skip(4)?;
        
        // Read number of channels
        let num_channels = stream.read_u8()?;
        if num_channels == 0 {
            return Err(Error::BmffParse(
                format!("Invalid number of channels in pixi box: {}", num_channels)
            ));
        }
        
        // Read bits per channel
        let mut bits_per_channel = Vec::with_capacity(num_channels as usize);
        for _ in 0..num_channels {
            bits_per_channel.push(stream.read_u8()?);
        }
        
        Ok(Self {
            bits_per_channel,
        })
    }
    
    /// Get the number of channels.
    pub fn num_channels(&self) -> usize {
        self.bits_per_channel.len()
    }
    
    /// Check if this is a monochrome image.
    pub fn is_monochrome(&self) -> bool {
        self.num_channels() == 1
    }
    
    /// Check if this is a color image with alpha.
    pub fn has_alpha(&self) -> bool {
        self.num_channels() == 4
    }
}
