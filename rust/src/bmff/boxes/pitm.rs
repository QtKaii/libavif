//! Primary Item Box implementation.

use crate::error::Result;
use crate::io::ReadStream;

/// Primary item box.
#[derive(Debug, Clone)]
pub struct PrimaryItemBox {
    /// Version.
    pub version: u8,
    /// Item ID.
    pub item_id: u32,
}

impl PrimaryItemBox {
    /// Parse a primary item box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let (version, _) = stream.read_version_and_flags()?;
        
        // Read item ID
        let item_id = if version == 0 {
            stream.read_u16()? as u32
        } else {
            stream.read_u32()?
        };
        
        Ok(Self {
            version,
            item_id,
        })
    }
}
