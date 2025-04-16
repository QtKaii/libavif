//! BMFF parser for the Rust implementation of libavif.

use crate::error::{Error, Result};
use crate::memory::Buffer;
use std::convert::TryFrom;

/// BMFF box type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxType([u8; 4]);

impl BoxType {
    /// Create a new BoxType from a 4-byte array.
    pub fn new(type_code: [u8; 4]) -> Self {
        Self(type_code)
    }

    /// Get the type code as a string.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("????")
    }
}

impl TryFrom<&[u8]> for BoxType {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 4 {
            return Err(Error::BmffParse(
                "Box type must be 4 bytes long".to_string(),
            ));
        }
        let mut type_code = [0u8; 4];
        type_code.copy_from_slice(value);
        Ok(Self(type_code))
    }
}

/// BMFF box header.
#[derive(Debug)]
pub struct BoxHeader {
    /// Box size.
    pub size: u64,
    /// Box type.
    pub box_type: BoxType,
    /// Extended type (for 'uuid' boxes).
    pub extended_type: Option<[u8; 16]>,
}

/// BMFF box.
#[derive(Debug)]
pub struct Box {
    /// Box header.
    pub header: BoxHeader,
    /// Box data.
    pub data: Buffer,
}

/// BMFF parser.
#[derive(Debug)]
pub struct Parser<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser from the given data.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// Parse the next box.
    pub fn next_box(&mut self) -> Result<Option<Box>> {
        if self.position >= self.data.len() {
            return Ok(None);
        }

        // Parse box header
        if self.data.len() - self.position < 8 {
            return Err(Error::BmffParse(
                "Not enough data to parse box header".to_string(),
            ));
        }

        let size_bytes = &self.data[self.position..self.position + 4];
        let size = u32::from_be_bytes([
            size_bytes[0],
            size_bytes[1],
            size_bytes[2],
            size_bytes[3],
        ]) as u64;

        let type_bytes = &self.data[self.position + 4..self.position + 8];
        let box_type = BoxType::try_from(type_bytes)?;

        let mut header_size = 8;
        let mut extended_type = None;

        // Handle extended size
        let mut box_size = size;
        if size == 1 {
            if self.data.len() - self.position < 16 {
                return Err(Error::BmffParse(
                    "Not enough data to parse extended size".to_string(),
                ));
            }
            let extended_size_bytes = &self.data[self.position + 8..self.position + 16];
            box_size = u64::from_be_bytes([
                extended_size_bytes[0],
                extended_size_bytes[1],
                extended_size_bytes[2],
                extended_size_bytes[3],
                extended_size_bytes[4],
                extended_size_bytes[5],
                extended_size_bytes[6],
                extended_size_bytes[7],
            ]);
            header_size = 16;
        }

        // Handle uuid type
        if box_type.as_str() == "uuid" {
            if self.data.len() - self.position < header_size + 16 {
                return Err(Error::BmffParse(
                    "Not enough data to parse uuid type".to_string(),
                ));
            }
            let mut uuid = [0u8; 16];
            uuid.copy_from_slice(
                &self.data[self.position + header_size..self.position + header_size + 16],
            );
            extended_type = Some(uuid);
            header_size += 16;
        }

        // Ensure we have enough data for the entire box
        if box_size == 0 {
            // Box extends to the end of the file
            box_size = self.data.len() as u64 - self.position as u64;
        } else if box_size < header_size as u64 {
            return Err(Error::BmffParse(
                "Box size is smaller than header size".to_string(),
            ));
        } else if self.position + box_size as usize > self.data.len() {
            return Err(Error::BmffParse("Box extends beyond end of data".to_string()));
        }

        // Create box header
        let header = BoxHeader {
            size: box_size,
            box_type,
            extended_type,
        };

        // Extract box data
        let data_start = self.position + header_size;
        let data_end = self.position + box_size as usize;
        let data = Buffer::from_slice(&self.data[data_start..data_end]);

        // Update position
        self.position = data_end;

        Ok(Some(Box { header, data }))
    }

    /// Reset the parser to the beginning of the data.
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Get the current position of the parser.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the position of the parser.
    pub fn set_position(&mut self, position: usize) -> Result<()> {
        if position > self.data.len() {
            return Err(Error::InvalidParameter(
                "Position is beyond end of data".to_string(),
            ));
        }
        self.position = position;
        Ok(())
    }

    /// Get the remaining data.
    pub fn remaining(&self) -> &[u8] {
        &self.data[self.position..]
    }

    /// Check if there is more data to parse.
    pub fn has_more(&self) -> bool {
        self.position < self.data.len()
    }
}
