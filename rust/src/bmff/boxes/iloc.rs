//! Item Location Box implementation.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Item extent.
#[derive(Debug, Clone)]
pub struct ItemExtent {
    /// Extent offset.
    pub extent_offset: u64,
    /// Extent length.
    pub extent_length: u64,
    /// Extent index (for construction method 1).
    pub extent_index: Option<u64>,
}

/// Item location entry.
#[derive(Debug)]
pub struct ItemLocationEntry {
    /// Item ID.
    pub item_id: u32,
    /// Construction method.
    pub construction_method: u8,
    /// Data reference index.
    pub data_reference_index: u16,
    /// Base offset.
    pub base_offset: u64,
    /// Extents.
    pub extents: Vec<ItemExtent>,
}

/// Item location box.
#[derive(Debug)]
pub struct ItemLocationBox {
    /// Version.
    pub version: u8,
    /// Offset size.
    pub offset_size: u8,
    /// Length size.
    pub length_size: u8,
    /// Base offset size.
    pub base_offset_size: u8,
    /// Index size (only used in version 1 and 2).
    pub index_size: u8,
    /// Entries.
    pub entries: Vec<ItemLocationEntry>,
}

impl ItemLocationBox {
    /// Parse an item location box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let (version, _) = stream.read_version_and_flags()?;
        if version > 2 {
            return Err(Error::BmffParse(
                format!("Unsupported iloc version: {}", version)
            ));
        }
        
        // Read sizes
        let sizes = stream.read_u8()?;
        let offset_size = (sizes >> 4) & 0xF;
        let length_size = sizes & 0xF;
        
        let sizes2 = stream.read_u8()?;
        let base_offset_size = (sizes2 >> 4) & 0xF;
        let index_size = if version == 0 {
            0
        } else {
            sizes2 & 0xF
        };
        
        // Validate sizes
        if (offset_size != 0 && offset_size != 4 && offset_size != 8) ||
           (length_size != 0 && length_size != 4 && length_size != 8) ||
           (base_offset_size != 0 && base_offset_size != 4 && base_offset_size != 8) ||
           (index_size != 0 && index_size != 4 && index_size != 8) {
            return Err(Error::BmffParse(
                format!("Invalid size in iloc box: offset_size={}, length_size={}, base_offset_size={}, index_size={}",
                        offset_size, length_size, base_offset_size, index_size)
            ));
        }
        
        // Read item count
        let item_count = if version < 2 {
            stream.read_u16()? as u32
        } else {
            stream.read_u32()?
        };
        
        // Read entries
        let mut entries = Vec::with_capacity(item_count as usize);
        for _ in 0..item_count {
            // Read item ID
            let item_id = if version < 2 {
                stream.read_u16()? as u32
            } else {
                stream.read_u32()?
            };
            
            // Read construction method and data reference index
            let (construction_method, data_reference_index) = if version == 0 {
                (0, stream.read_u16()?)
            } else {
                let reserved = stream.read_u16()?;
                let construction_method = (reserved & 0xF) as u8;
                let data_reference_index = (reserved >> 4) & 0xFFF;
                
                if construction_method > 1 {
                    return Err(Error::BmffParse(
                        format!("Unsupported construction method: {}", construction_method)
                    ));
                }
                
                (construction_method, data_reference_index as u16)
            };
            
            // Read base offset
            let base_offset = match base_offset_size {
                0 => 0,
                4 => stream.read_u32()? as u64,
                8 => stream.read_u64()?,
                _ => unreachable!(),
            };
            
            // Read extent count
            let extent_count = stream.read_u16()?;
            
            // Read extents
            let mut extents = Vec::with_capacity(extent_count as usize);
            for _ in 0..extent_count {
                // Read index (only in version 1 and 2)
                let extent_index = if index_size > 0 {
                    let index = match index_size {
                        4 => stream.read_u32()? as u64,
                        8 => stream.read_u64()?,
                        _ => unreachable!(),
                    };
                    Some(index)
                } else {
                    None
                };
                
                // Read offset
                let extent_offset = match offset_size {
                    0 => 0,
                    4 => stream.read_u32()? as u64,
                    8 => stream.read_u64()?,
                    _ => unreachable!(),
                };
                
                // Read length
                let extent_length = match length_size {
                    0 => 0,
                    4 => stream.read_u32()? as u64,
                    8 => stream.read_u64()?,
                    _ => unreachable!(),
                };
                
                extents.push(ItemExtent {
                    extent_offset,
                    extent_length,
                    extent_index,
                });
            }
            
            entries.push(ItemLocationEntry {
                item_id,
                construction_method,
                data_reference_index,
                base_offset,
                extents,
            });
        }
        
        Ok(Self {
            version,
            offset_size,
            length_size,
            base_offset_size,
            index_size,
            entries,
        })
    }
    
    /// Find an entry by item ID.
    pub fn find_entry(&self, item_id: u32) -> Option<&ItemLocationEntry> {
        self.entries.iter().find(|entry| entry.item_id == item_id)
    }
}
