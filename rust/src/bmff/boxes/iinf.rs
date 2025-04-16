//! Item Information Box implementation.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Item information entry.
#[derive(Debug)]
pub struct ItemInfoEntry {
    /// Item ID.
    pub item_id: u32,
    /// Item type.
    pub item_type: [u8; 4],
    /// Item name.
    pub item_name: String,
    /// Content type.
    pub content_type: Option<String>,
}

impl ItemInfoEntry {
    /// Parse an item information entry from a buffer.
    pub fn parse(data: &[u8], version: u8) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read item ID
        let item_id = if version == 0 {
            stream.read_u16()? as u32
        } else {
            stream.read_u32()?
        };
        
        // Skip item protection index (2 bytes)
        stream.skip(2)?;
        
        // Read item type (4 bytes)
        let mut item_type = [0u8; 4];
        stream.read_exact(&mut item_type)?;
        
        // Read item name (null-terminated string)
        let item_name = stream.read_string()?;
        
        // Read content type if there's more data
        let content_type = if stream.has_more() {
            Some(stream.read_string()?)
        } else {
            None
        };
        
        Ok(Self {
            item_id,
            item_type,
            item_name,
            content_type,
        })
    }
    
    /// Check if this is an image item.
    pub fn is_image(&self) -> bool {
        // Check for common image types
        self.item_type == *b"avis" || // AVIF sequence
        self.item_type == *b"av01" || // AV1 image
        self.item_type == *b"av02" || // AV2 image
        self.item_type == *b"grid" || // Image grid
        self.item_type == *b"iden" || // Identity image
        self.item_type == *b"iovl"    // Image overlay
    }
}

/// Item information box.
#[derive(Debug)]
pub struct ItemInfoBox {
    /// Version.
    pub version: u8,
    /// Entries.
    pub entries: Vec<ItemInfoEntry>,
}

impl ItemInfoBox {
    /// Parse an item information box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let (version, _) = stream.read_version_and_flags()?;
        
        // Read entry count
        let entry_count = if version == 0 {
            stream.read_u16()? as u32
        } else {
            stream.read_u32()?
        };
        
        // Read entries
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            // Read entry size
            let entry_size = stream.read_u16()? as usize;
            if entry_size < 4 {
                return Err(Error::BmffParse(
                    format!("Item info entry size too small: {}", entry_size)
                ));
            }
            
            // Read entry data
            let entry_start = stream.position();
            let entry = ItemInfoEntry::parse(stream.remaining(), version)?;
            entries.push(entry);
            
            // Skip to the end of the entry
            let entry_end = entry_start + entry_size;
            if entry_end > stream.data().len() {
                return Err(Error::BmffParse(
                    format!("Item info entry extends beyond end of box")
                ));
            }
            stream.set_position(entry_end)?;
        }
        
        Ok(Self {
            version,
            entries,
        })
    }
    
    /// Find an entry by item ID.
    pub fn find_entry(&self, item_id: u32) -> Option<&ItemInfoEntry> {
        self.entries.iter().find(|entry| entry.item_id == item_id)
    }
}
