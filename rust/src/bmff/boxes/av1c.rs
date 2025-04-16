//! AV1 Configuration Box implementation.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// AV1 configuration box.
#[derive(Debug, Clone)]
pub struct AV1ConfigurationBox {
    /// Marker (must be 1).
    pub marker: u8,
    /// Version (must be 1).
    pub version: u8,
    /// Profile.
    pub profile: u8,
    /// Level.
    pub level: u8,
    /// Tier.
    pub tier: u8,
    /// High bit depth flag.
    pub high_bitdepth: bool,
    /// Twelve bit flag.
    pub twelve_bit: bool,
    /// Monochrome flag.
    pub monochrome: bool,
    /// Chroma subsampling X.
    pub chroma_subsampling_x: u8,
    /// Chroma subsampling Y.
    pub chroma_subsampling_y: u8,
    /// Chroma sample position.
    pub chroma_sample_position: u8,
    /// Initial presentation delay present flag.
    pub initial_presentation_delay_present: bool,
    /// Initial presentation delay minus one.
    pub initial_presentation_delay_minus_one: Option<u8>,
    /// Configuration OBUs.
    pub config_obus: Vec<u8>,
}

impl AV1ConfigurationBox {
    /// Parse an AV1 configuration box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read marker and version
        let marker_and_version = stream.read_u8()?;
        let marker = (marker_and_version >> 7) & 0x1;
        let version = marker_and_version & 0x7F;
        
        if marker != 1 {
            return Err(Error::BmffParse(
                format!("Invalid marker in AV1 configuration box: {}", marker)
            ));
        }
        
        if version != 1 {
            return Err(Error::BmffParse(
                format!("Unsupported version in AV1 configuration box: {}", version)
            ));
        }
        
        // Read profile and level
        let profile_and_level = stream.read_u8()?;
        let profile = (profile_and_level >> 5) & 0x7;
        let level = profile_and_level & 0x1F;
        
        // Read flags
        let flags = stream.read_u8()?;
        let tier = (flags >> 7) & 0x1;
        let high_bitdepth = ((flags >> 6) & 0x1) != 0;
        let twelve_bit = ((flags >> 5) & 0x1) != 0;
        let monochrome = ((flags >> 4) & 0x1) != 0;
        let chroma_subsampling_x = (flags >> 3) & 0x1;
        let chroma_subsampling_y = (flags >> 2) & 0x1;
        let chroma_sample_position = flags & 0x3;
        
        // Read delay
        let delay = stream.read_u8()?;
        let initial_presentation_delay_present = ((delay >> 4) & 0x1) != 0;
        let initial_presentation_delay_minus_one = if initial_presentation_delay_present {
            Some(delay & 0xF)
        } else {
            None
        };
        
        // Read configuration OBUs
        let config_obus = stream.remaining().to_vec();
        
        Ok(Self {
            marker,
            version,
            profile,
            level,
            tier,
            high_bitdepth,
            twelve_bit,
            monochrome,
            chroma_subsampling_x,
            chroma_subsampling_y,
            chroma_sample_position,
            initial_presentation_delay_present,
            initial_presentation_delay_minus_one,
            config_obus,
        })
    }
    
    /// Get the bit depth.
    pub fn bit_depth(&self) -> u8 {
        if self.high_bitdepth {
            if self.twelve_bit {
                12
            } else {
                10
            }
        } else {
            8
        }
    }
}
