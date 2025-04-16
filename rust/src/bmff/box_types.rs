//! BMFF box type implementations.
//!
//! This module contains implementations for the various box types defined in the BMFF specification.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// File type box.
#[derive(Debug)]
pub struct FileTypeBox {
    /// Major brand.
    pub major_brand: [u8; 4],
    /// Minor version.
    pub minor_version: [u8; 4],
    /// Compatible brands.
    pub compatible_brands: Vec<[u8; 4]>,
}

impl FileTypeBox {
    /// Parse a file type box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read major brand (4 bytes)
        let mut major_brand = [0u8; 4];
        stream.read_exact(&mut major_brand)?;
        
        // Read minor version (4 bytes)
        let mut minor_version = [0u8; 4];
        stream.read_exact(&mut minor_version)?;
        
        // Read compatible brands (remainder of the box)
        let remaining = stream.remaining().len();
        if remaining % 4 != 0 {
            return Err(Error::BmffParse(
                format!("Compatible brands section size ({}) is not divisible by 4", remaining)
            ));
        }
        
        let brand_count = remaining / 4;
        let mut compatible_brands = Vec::with_capacity(brand_count);
        
        for _ in 0..brand_count {
            let mut brand = [0u8; 4];
            stream.read_exact(&mut brand)?;
            compatible_brands.push(brand);
        }
        
        Ok(Self {
            major_brand,
            minor_version,
            compatible_brands,
        })
    }
    
    /// Check if this file type box has a specific brand.
    pub fn has_brand(&self, brand: &str) -> bool {
        if brand.len() != 4 {
            return false;
        }
        
        let brand_bytes = brand.as_bytes();
        if self.major_brand == brand_bytes {
            return true;
        }
        
        for compatible_brand in &self.compatible_brands {
            if compatible_brand == brand_bytes {
                return true;
            }
        }
        
        false
    }
    
    /// Check if this is an AVIF file.
    pub fn is_avif(&self) -> bool {
        self.has_brand("avif") || self.has_brand("avis")
    }
}

/// Handler box.
#[derive(Debug)]
pub struct HandlerBox {
    /// Handler type.
    pub handler_type: [u8; 4],
    /// Handler name.
    pub name: String,
}

impl HandlerBox {
    /// Parse a handler box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Skip version and flags (4 bytes)
        stream.skip(4)?;
        
        // Skip pre-defined (4 bytes)
        stream.skip(4)?;
        
        // Read handler type (4 bytes)
        let mut handler_type = [0u8; 4];
        stream.read_exact(&mut handler_type)?;
        
        // Skip reserved (12 bytes)
        stream.skip(12)?;
        
        // Read name (null-terminated string)
        let name = stream.read_string()?;
        
        Ok(Self {
            handler_type,
            name,
        })
    }
}

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
}
