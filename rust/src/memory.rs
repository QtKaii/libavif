//! Memory management for the Rust implementation of libavif.

use crate::error::Result;
use std::ops::{Deref, DerefMut};

/// A safe wrapper around a byte buffer.
#[derive(Debug)]
pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Create a new buffer with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Create a new buffer from the given data.
    pub fn from_slice(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    /// Resize the buffer to the given size.
    pub fn resize(&mut self, new_size: usize) -> Result<()> {
        self.data.resize(new_size, 0);
        Ok(())
    }

    /// Get a reference to the underlying data.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Get a mutable reference to the underlying data.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Get the length of the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear the buffer.
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

/// A safe equivalent to avifRWData.
#[derive(Debug)]
pub struct RwData {
    buffer: Buffer,
}

impl RwData {
    /// Create a new empty RwData.
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
        }
    }

    /// Create a new RwData with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Buffer::with_capacity(capacity),
        }
    }

    /// Create a new RwData from the given data.
    pub fn from_slice(data: &[u8]) -> Self {
        Self {
            buffer: Buffer::from_slice(data),
        }
    }

    /// Resize the RwData to the given size.
    pub fn resize(&mut self, new_size: usize) -> Result<()> {
        self.buffer.resize(new_size)
    }

    /// Get a reference to the underlying data.
    pub fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Get a mutable reference to the underlying data.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    /// Get the length of the RwData.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the RwData is empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Clear the RwData.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Default for RwData {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for RwData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for RwData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}
