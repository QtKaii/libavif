//! I/O handling for the Rust implementation of libavif.

use crate::error::{Error, Result};
use std::io::{Read, Seek, SeekFrom, Write};

/// A safe equivalent to avifROStream.
#[derive(Debug)]
pub struct ReadStream<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> ReadStream<'a> {
    /// Create a new ReadStream from the given data.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// Read bytes into the given buffer.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let available = self.data.len() - self.position;
        let to_read = std::cmp::min(available, buf.len());
        if to_read == 0 {
            return Ok(0);
        }
        buf[..to_read].copy_from_slice(&self.data[self.position..self.position + to_read]);
        self.position += to_read;
        Ok(to_read)
    }

    /// Read exactly `buf.len()` bytes into the given buffer.
    pub fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        if self.position + buf.len() > self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream",
            )));
        }
        buf.copy_from_slice(&self.data[self.position..self.position + buf.len()]);
        self.position += buf.len();
        Ok(())
    }

    /// Read a single byte.
    pub fn read_u8(&mut self) -> Result<u8> {
        if self.position >= self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream",
            )));
        }
        let byte = self.data[self.position];
        self.position += 1;
        Ok(byte)
    }

    /// Read a 16-bit unsigned integer in big-endian format.
    pub fn read_u16(&mut self) -> Result<u16> {
        if self.position + 2 > self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream",
            )));
        }
        let value = u16::from_be_bytes([
            self.data[self.position],
            self.data[self.position + 1],
        ]);
        self.position += 2;
        Ok(value)
    }

    /// Read a 32-bit unsigned integer in big-endian format.
    pub fn read_u32(&mut self) -> Result<u32> {
        if self.position + 4 > self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream",
            )));
        }
        let value = u32::from_be_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ]);
        self.position += 4;
        Ok(value)
    }

    /// Read a 64-bit unsigned integer in big-endian format.
    pub fn read_u64(&mut self) -> Result<u64> {
        if self.position + 8 > self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream",
            )));
        }
        let value = u64::from_be_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
        ]);
        self.position += 8;
        Ok(value)
    }

    /// Read a null-terminated string.
    pub fn read_string(&mut self) -> Result<String> {
        let mut result = String::new();
        let mut byte = [0u8; 1];

        while self.has_more() {
            self.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            result.push(byte[0] as char);
        }

        Ok(result)
    }

    /// Read a fixed-length string.
    pub fn read_fixed_string(&mut self, len: usize) -> Result<String> {
        let mut bytes = vec![0u8; len];
        self.read_exact(&mut bytes)?;

        // Remove trailing nulls
        while !bytes.is_empty() && bytes[bytes.len() - 1] == 0 {
            bytes.pop();
        }

        Ok(String::from_utf8_lossy(&bytes).to_string())
    }

    /// Read version and flags from a full box.
    pub fn read_version_and_flags(&mut self) -> Result<(u8, u32)> {
        let version = self.read_u8()?;
        let flags_high = self.read_u8()? as u32;
        let flags_mid = self.read_u8()? as u32;
        let flags_low = self.read_u8()? as u32;

        let flags = (flags_high << 16) | (flags_mid << 8) | flags_low;

        Ok((version, flags))
    }

    /// Skip the given number of bytes.
    pub fn skip(&mut self, count: usize) -> Result<()> {
        if self.position + count > self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "End of stream",
            )));
        }
        self.position += count;
        Ok(())
    }

    /// Get the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the current position.
    pub fn set_position(&mut self, position: usize) -> Result<()> {
        if position > self.data.len() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Position is beyond end of stream",
            )));
        }
        self.position = position;
        Ok(())
    }

    /// Get the remaining bytes.
    pub fn remaining(&self) -> &[u8] {
        &self.data[self.position..]
    }

    /// Get the number of remaining bytes.
    pub fn remaining_bytes(&self) -> usize {
        self.data.len() - self.position
    }

    /// Check if there are more bytes to read.
    pub fn has_more(&self) -> bool {
        self.position < self.data.len()
    }

    /// Get the underlying data.
    pub fn data(&self) -> &'a [u8] {
        self.data
    }
}

impl<'a> Read for ReadStream<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read(buf).map_err(|e| match e {
            Error::Io(io_error) => io_error,
            _ => std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
        })
    }
}

impl<'a> Seek for ReadStream<'a> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_position = match pos {
            SeekFrom::Start(offset) => offset as usize,
            SeekFrom::End(offset) => {
                if offset < 0 {
                    self.data
                        .len()
                        .checked_sub((-offset) as usize)
                        .ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Seek position is before start of stream",
                            )
                        })?
                } else {
                    self.data.len() + offset as usize
                }
            }
            SeekFrom::Current(offset) => {
                if offset < 0 {
                    self.position
                        .checked_sub((-offset) as usize)
                        .ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Seek position is before start of stream",
                            )
                        })?
                } else {
                    self.position + offset as usize
                }
            }
        };

        if new_position > self.data.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Seek position is beyond end of stream",
            ));
        }

        self.position = new_position;
        Ok(self.position as u64)
    }
}

/// A safe equivalent to avifRWStream.
#[derive(Debug)]
pub struct WriteStream {
    data: Vec<u8>,
    position: usize,
}

impl WriteStream {
    /// Create a new WriteStream.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            position: 0,
        }
    }

    /// Create a new WriteStream with the given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            position: 0,
        }
    }

    /// Write bytes from the given buffer.
    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        // Ensure we have enough capacity
        let required_len = self.position + buf.len();
        if required_len > self.data.len() {
            self.data.resize(required_len, 0);
        }

        // Copy the data
        self.data[self.position..self.position + buf.len()].copy_from_slice(buf);
        self.position += buf.len();

        Ok(buf.len())
    }

    /// Write a single byte.
    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write(&[value])?;
        Ok(())
    }

    /// Write a 16-bit unsigned integer in big-endian format.
    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        self.write(&value.to_be_bytes())?;
        Ok(())
    }

    /// Write a 32-bit unsigned integer in big-endian format.
    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        self.write(&value.to_be_bytes())?;
        Ok(())
    }

    /// Write a 64-bit unsigned integer in big-endian format.
    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        self.write(&value.to_be_bytes())?;
        Ok(())
    }

    /// Get the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the current position.
    pub fn set_position(&mut self, position: usize) -> Result<()> {
        if position > self.data.len() {
            self.data.resize(position, 0);
        }
        self.position = position;
        Ok(())
    }

    /// Get the data written so far.
    pub fn data(&self) -> &[u8] {
        &self.data[..self.position]
    }

    /// Take ownership of the data written so far.
    pub fn into_data(self) -> Vec<u8> {
        self.data[..self.position].to_vec()
    }
}

impl Write for WriteStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write(buf).map_err(|e| match e {
            Error::Io(io_error) => io_error,
            _ => std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // No-op for in-memory stream
        Ok(())
    }
}

impl Seek for WriteStream {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_position = match pos {
            SeekFrom::Start(offset) => offset as usize,
            SeekFrom::End(offset) => {
                if offset < 0 {
                    self.data
                        .len()
                        .checked_sub((-offset) as usize)
                        .ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Seek position is before start of stream",
                            )
                        })?
                } else {
                    self.data.len() + offset as usize
                }
            }
            SeekFrom::Current(offset) => {
                if offset < 0 {
                    self.position
                        .checked_sub((-offset) as usize)
                        .ok_or_else(|| {
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Seek position is before start of stream",
                            )
                        })?
                } else {
                    self.position + offset as usize
                }
            }
        };

        if new_position > self.data.len() {
            self.data.resize(new_position, 0);
        }

        self.position = new_position;
        Ok(self.position as u64)
    }
}
