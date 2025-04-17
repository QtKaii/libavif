//! Track Box implementation.
//!
//! The Track Box (`trak`) is a container box for a single track of a presentation.
//! It contains a Track Header Box (`tkhd`) and a Media Box (`mdia`).

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Track Box.
#[derive(Debug)]
pub struct TrackBox {
    /// Track Header Box.
    pub tkhd: TrackHeaderBox,
    /// Media Box.
    pub mdia: Option<super::mdia::MediaBox>,
    /// Edit Box.
    pub edts: Option<EditBox>,
    /// Track Reference Box.
    pub tref: Option<TrackReferenceBox>,
    /// Meta Box.
    pub meta: Option<super::meta::MetaBox>,
}

/// Track Header Box.
#[derive(Debug)]
pub struct TrackHeaderBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Creation time.
    pub creation_time: u64,
    /// Modification time.
    pub modification_time: u64,
    /// Track ID.
    pub track_id: u32,
    /// Duration.
    pub duration: u64,
    /// Width.
    pub width: u32,
    /// Height.
    pub height: u32,
}

/// Edit Box.
#[derive(Debug)]
pub struct EditBox {
    /// Edit List Box.
    pub elst: EditListBox,
}

/// Edit List Box.
#[derive(Debug)]
pub struct EditListBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Entries.
    pub entries: Vec<EditListEntry>,
}

/// Edit List Entry.
#[derive(Debug)]
pub struct EditListEntry {
    /// Segment duration.
    pub segment_duration: u64,
    /// Media time.
    pub media_time: i64,
    /// Media rate.
    pub media_rate: i16,
    /// Media rate fraction.
    pub media_rate_fraction: i16,
}

/// Track Reference Box.
#[derive(Debug)]
pub struct TrackReferenceBox {
    /// References.
    pub references: Vec<TrackReference>,
}

/// Track Reference.
#[derive(Debug)]
pub struct TrackReference {
    /// Reference type.
    pub reference_type: [u8; 4],
    /// Track IDs.
    pub track_ids: Vec<u32>,
}

impl TrackBox {
    /// Parse a track box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut tkhd = None;
        let mut mdia = None;
        let mut edts = None;
        let mut tref = None;
        let mut meta = None;
        
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
                b"tkhd" => {
                    if tkhd.is_some() {
                        return Err(Error::BmffParse("Duplicate tkhd box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    tkhd = Some(TrackHeaderBox::parse(&box_data)?);
                },
                b"mdia" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    mdia = Some(super::mdia::MediaBox::parse(&box_data)?);
                },
                b"edts" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    edts = Some(EditBox::parse(&box_data)?);
                },
                b"tref" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    tref = Some(TrackReferenceBox::parse(&box_data)?);
                },
                b"meta" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    meta = Some(super::meta::MetaBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        // Ensure we have a track header box
        let tkhd = tkhd.ok_or_else(|| Error::BmffParse("Missing tkhd box".to_string()))?;
        
        Ok(Self {
            tkhd,
            mdia,
            edts,
            tref,
            meta,
        })
    }
}

impl TrackHeaderBox {
    /// Parse a track header box from a buffer.
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
        
        // Read track ID
        let track_id = stream.read_u32()?;
        
        // Skip reserved
        stream.skip(4)?;
        
        // Read duration
        let duration = if version == 1 {
            stream.read_u64()?
        } else {
            stream.read_u32()? as u64
        };
        
        // Skip reserved, layer, alternate_group, volume, reserved
        stream.skip(16)?;
        
        // Skip matrix
        stream.skip(36)?;
        
        // Read width and height (fixed point 16.16)
        let width_fixed = stream.read_u32()?;
        let height_fixed = stream.read_u32()?;
        
        // Convert from fixed point to integer
        let width = width_fixed >> 16;
        let height = height_fixed >> 16;
        
        Ok(Self {
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            width,
            height,
        })
    }
}

impl EditBox {
    /// Parse an edit box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut elst = None;
        
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
                b"elst" => {
                    if elst.is_some() {
                        return Err(Error::BmffParse("Duplicate elst box".to_string()));
                    }
                    
                    let box_data = stream.read_exact_buffer(data_size)?;
                    elst = Some(EditListBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }
        
        // Ensure we have an edit list box
        let elst = elst.ok_or_else(|| Error::BmffParse("Missing elst box".to_string()))?;
        
        Ok(Self {
            elst,
        })
    }
}

impl EditListBox {
    /// Parse an edit list box from a buffer.
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
            let segment_duration = if version == 1 {
                stream.read_u64()?
            } else {
                stream.read_u32()? as u64
            };
            
            let media_time = if version == 1 {
                stream.read_i64()?
            } else {
                stream.read_i32()? as i64
            };
            
            let media_rate = stream.read_i16()?;
            let media_rate_fraction = stream.read_i16()?;
            
            entries.push(EditListEntry {
                segment_duration,
                media_time,
                media_rate,
                media_rate_fraction,
            });
        }
        
        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}

impl TrackReferenceBox {
    /// Parse a track reference box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut references = Vec::new();
        
        while stream.has_more() {
            // Read reference type
            let mut reference_type = [0u8; 4];
            stream.read_exact(&mut reference_type)?;
            
            // Read track IDs
            let mut track_ids = Vec::new();
            while stream.has_more() {
                track_ids.push(stream.read_u32()?);
            }
            
            references.push(TrackReference {
                reference_type,
                track_ids,
            });
        }
        
        Ok(Self {
            references,
        })
    }
}
