//! Media Box implementation.
//!
//! The Media Box (`mdia`) is a container box for the media information in a track.
//! It contains a Media Header Box (`mdhd`), a Handler Reference Box (`hdlr`),
//! and a Media Information Box (`minf`).

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Media Box.
#[derive(Debug)]
pub struct MediaBox {
    /// Media Header Box.
    pub mdhd: MediaHeaderBox,
    /// Handler Reference Box.
    pub hdlr: super::hdlr::HandlerBox,
    /// Media Information Box.
    pub minf: Option<super::minf::MediaInformationBox>,
}

/// Media Header Box.
#[derive(Debug)]
pub struct MediaHeaderBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Creation time.
    pub creation_time: u64,
    /// Modification time.
    pub modification_time: u64,
    /// Timescale.
    pub timescale: u32,
    /// Duration.
    pub duration: u64,
    /// Language.
    pub language: String,
}

impl MediaBox {
    /// Parse a media box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut mdhd = None;
        let mut hdlr = None;
        let mut minf = None;
        
        while stream.has_more() {
            // Read box header
            let box_size = stream.read_u32()? as usize;
            if box_size < 8 {
                return Err(Error::BmffParse("Invalid box size".to_string()));
            }
            
            let mut box_type = [0u8; 4];
            stream.read_exact(&mut box_type)?;
            
            // Calculate box data size
            let data_size = box_size - 8;
            
            // Parse box based on type
            match &box_type {
                b"mdhd" => {
                    if mdhd.is_some() {
                        return Err(Error::BmffParse("Duplicate mdhd box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    mdhd = Some(MediaHeaderBox::parse(&box_data)?);
                },
                b"hdlr" => {
                    if hdlr.is_some() {
                        return Err(Error::BmffParse("Duplicate hdlr box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    hdlr = Some(super::hdlr::HandlerBox::parse(&box_data)?);
                },
                b"minf" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    minf = Some(super::minf::MediaInformationBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        // Ensure we have a media header box and handler box
        let mdhd = mdhd.ok_or_else(|| Error::BmffParse("Missing mdhd box".to_string()))?;
        let hdlr = hdlr.ok_or_else(|| Error::BmffParse("Missing hdlr box".to_string()))?;
        
        Ok(Self {
            mdhd,
            hdlr,
            minf,
        })
    }
}

impl MediaHeaderBox {
    /// Parse a media header box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;
        
        // Read creation and modification times
        let (creation_time, modification_time) = if version == 1 {
            (stream.read_u64()?, stream.read_u64()?)
        } else {
            (stream.read_u32()? as u64, stream.read_u32()? as u64)
        };
        
        // Read timescale
        let timescale = stream.read_u32()?;
        
        // Read duration
        let duration = if version == 1 {
            stream.read_u64()?
        } else {
            stream.read_u32()? as u64
        };
        
        // Read language (ISO-639-2/T code)
        let language_code = stream.read_u16()?;
        
        // Convert language code to string
        // The language code is stored as a packed ISO-639-2/T code: 
        // each character is stored as the difference between its ASCII value and 0x60
        let mut language = String::with_capacity(3);
        language.push((((language_code >> 10) & 0x1F) + 0x60) as u8 as char);
        language.push((((language_code >> 5) & 0x1F) + 0x60) as u8 as char);
        language.push(((language_code & 0x1F) + 0x60) as u8 as char);
        
        // Skip pre-defined
        stream.skip(2)?;
        
        Ok(Self {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
        })
    }
}
