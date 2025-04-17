//! Free Space Box implementation.

use crate::error::Result;

/// Free Space Box.
/// 
/// This box type is used to skip unused space in the file.
/// There are two variants: 'free' and 'skip', which are functionally identical.
#[derive(Debug)]
pub struct FreeSpaceBox {
    /// Box type ('free' or 'skip').
    pub box_type: [u8; 4],
    /// Data.
    pub data: Vec<u8>,
}

impl FreeSpaceBox {
    /// Parse a free space box from a buffer.
    pub fn parse(box_type: [u8; 4], data: &[u8]) -> Result<Self> {
        // Free space box is just a container for unused data
        Ok(Self {
            box_type,
            data: data.to_vec(),
        })
    }
}
