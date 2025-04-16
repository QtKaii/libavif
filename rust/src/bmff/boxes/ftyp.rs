//! File Type Box implementation.

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
