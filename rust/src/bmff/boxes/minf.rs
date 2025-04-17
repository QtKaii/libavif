//! Media Information Box implementation.
//!
//! The Media Information Box (`minf`) contains objects that declare characteristic
//! information about the media data within the track.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Media Information Box.
#[derive(Debug)]
pub struct MediaInformationBox {
    /// Video Media Header Box.
    pub vmhd: Option<VideoMediaHeaderBox>,
    /// Sound Media Header Box.
    pub smhd: Option<SoundMediaHeaderBox>,
    /// Data Information Box.
    pub dinf: Option<DataInformationBox>,
    /// Sample Table Box.
    pub stbl: Option<super::stbl::SampleTableBox>,
}

/// Video Media Header Box.
#[derive(Debug)]
pub struct VideoMediaHeaderBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Graphics mode.
    pub graphics_mode: u16,
    /// Opcolor.
    pub opcolor: [u16; 3],
}

/// Sound Media Header Box.
#[derive(Debug)]
pub struct SoundMediaHeaderBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Balance.
    pub balance: i16,
}

/// Data Information Box.
#[derive(Debug)]
pub struct DataInformationBox {
    /// Data Reference Box.
    pub dref: DataReferenceBox,
}

/// Data Reference Box.
#[derive(Debug)]
pub struct DataReferenceBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Data references.
    pub entries: Vec<DataReferenceEntry>,
}

/// Data Reference Entry.
#[derive(Debug)]
pub struct DataReferenceEntry {
    /// Entry type.
    pub entry_type: [u8; 4],
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Data.
    pub data: Vec<u8>,
}

impl MediaInformationBox {
    /// Parse a media information box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut vmhd = None;
        let mut smhd = None;
        let mut dinf = None;
        let mut stbl = None;
        
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
                b"vmhd" => {
                    if vmhd.is_some() {
                        return Err(Error::BmffParse("Duplicate vmhd box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    vmhd = Some(VideoMediaHeaderBox::parse(&box_data)?);
                },
                b"smhd" => {
                    if smhd.is_some() {
                        return Err(Error::BmffParse("Duplicate smhd box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    smhd = Some(SoundMediaHeaderBox::parse(&box_data)?);
                },
                b"dinf" => {
                    if dinf.is_some() {
                        return Err(Error::BmffParse("Duplicate dinf box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    dinf = Some(DataInformationBox::parse(&box_data)?);
                },
                b"stbl" => {
                    if stbl.is_some() {
                        return Err(Error::BmffParse("Duplicate stbl box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    stbl = Some(super::stbl::SampleTableBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        Ok(Self {
            vmhd,
            smhd,
            dinf,
            stbl,
        })
    }
}

impl VideoMediaHeaderBox {
    /// Parse a video media header box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;
        
        // Read graphics mode
        let graphics_mode = stream.read_u16()?;
        
        // Read opcolor
        let opcolor = [
            stream.read_u16()?,
            stream.read_u16()?,
            stream.read_u16()?,
        ];
        
        Ok(Self {
            version,
            flags,
            graphics_mode,
            opcolor,
        })
    }
}

impl SoundMediaHeaderBox {
    /// Parse a sound media header box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;
        
        // Read balance
        let balance = stream.read_i16()?;
        
        // Skip reserved
        stream.skip(2)?;
        
        Ok(Self {
            version,
            flags,
            balance,
        })
    }
}

impl DataInformationBox {
    /// Parse a data information box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut dref = None;
        
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
                b"dref" => {
                    if dref.is_some() {
                        return Err(Error::BmffParse("Duplicate dref box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    dref = Some(DataReferenceBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        // Ensure we have a data reference box
        let dref = dref.ok_or_else(|| Error::BmffParse("Missing dref box".to_string()))?;
        
        Ok(Self {
            dref,
        })
    }
}

impl DataReferenceBox {
    /// Parse a data reference box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        
        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;
        
        // Read entry count
        let entry_count = stream.read_u32()? as usize;
        
        // Read entries
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            // Read entry size
            let entry_size = stream.read_u32()? as usize;
            if entry_size < 8 {
                return Err(Error::BmffParse("Invalid data reference entry size".to_string()));
            }
            
            // Read entry type
            let mut entry_type = [0u8; 4];
            stream.read_exact(&mut entry_type)?;
            
            // Read entry version and flags
            let entry_version = stream.read_u8()?;
            let entry_flags = stream.read_u24()?;
            
            // Read entry data
            let data_size = entry_size - 8;
            let data = stream.read_exact_buffer(data_size)?;
            
            entries.push(DataReferenceEntry {
                entry_type,
                version: entry_version,
                flags: entry_flags,
                data: data.to_vec(),
            });
        }
        
        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}
