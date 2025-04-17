//! Item Data Box implementation.

use crate::error::Result;

/// Item Data Box.
/// 
/// This box contains data referenced by one or more items.
#[derive(Debug)]
pub struct ItemDataBox {
    /// Data.
    pub data: Vec<u8>,
}

impl ItemDataBox {
    /// Parse an item data box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        // Item data box is just a container for data
        Ok(Self {
            data: data.to_vec(),
        })
    }
}
