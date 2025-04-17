//! Media Data Box implementation.

use crate::error::Result;

/// Media Data Box.
#[derive(Debug)]
pub struct MediaDataBox {
    /// Data.
    pub data: Vec<u8>,
}

impl MediaDataBox {
    /// Parse a media data box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        // Media data box is just a container for data
        Ok(Self {
            data: data.to_vec(),
        })
    }
}
