//! Handler Box implementation.

use crate::error::Result;
use crate::io::ReadStream;

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
    
    /// Check if this is a picture handler.
    pub fn is_picture(&self) -> bool {
        self.handler_type == *b"pict"
    }
}
