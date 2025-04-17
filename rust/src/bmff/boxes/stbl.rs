//! Sample Table Box implementation.
//!
//! The Sample Table Box (`stbl`) contains all the time and data indexing of the
//! media samples in a track.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Sample Table Box.
#[derive(Debug)]
pub struct SampleTableBox {
    /// Sample Description Box.
    pub stsd: Option<SampleDescriptionBox>,
    /// Time to Sample Box.
    pub stts: Option<TimeToSampleBox>,
    /// Sample to Chunk Box.
    pub stsc: Option<SampleToChunkBox>,
    /// Sample Size Box.
    pub stsz: Option<SampleSizeBox>,
    /// Chunk Offset Box.
    pub stco: Option<ChunkOffsetBox>,
    /// Sync Sample Box.
    pub stss: Option<SyncSampleBox>,
}

/// Sample Description Box.
#[derive(Debug)]
pub struct SampleDescriptionBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Sample descriptions.
    pub entries: Vec<SampleDescription>,
}

/// Sample Description.
#[derive(Debug)]
pub enum SampleDescription {
    /// Visual Sample Description.
    Visual(VisualSampleDescription),
    /// Audio Sample Description.
    Audio(AudioSampleDescription),
    /// Unknown Sample Description.
    Unknown {
        /// Sample description type.
        sample_type: [u8; 4],
        /// Sample description data.
        data: Vec<u8>,
    },
}

/// Visual Sample Description.
#[derive(Debug)]
pub struct VisualSampleDescription {
    /// Sample description type.
    pub sample_type: [u8; 4],
    /// Width.
    pub width: u16,
    /// Height.
    pub height: u16,
    /// Horizontal resolution.
    pub horizresolution: u32,
    /// Vertical resolution.
    pub vertresolution: u32,
    /// Frame count.
    pub frame_count: u16,
    /// Compressor name.
    pub compressorname: String,
    /// Depth.
    pub depth: u16,
    /// Configuration boxes.
    pub config_boxes: Vec<ConfigBox>,
}

/// Audio Sample Description.
#[derive(Debug)]
pub struct AudioSampleDescription {
    /// Sample description type.
    pub sample_type: [u8; 4],
    /// Channel count.
    pub channelcount: u16,
    /// Sample size.
    pub samplesize: u16,
    /// Sample rate.
    pub samplerate: u32,
    /// Configuration boxes.
    pub config_boxes: Vec<ConfigBox>,
}

/// Configuration Box.
#[derive(Debug)]
pub struct ConfigBox {
    /// Box type.
    pub box_type: [u8; 4],
    /// Box data.
    pub data: Vec<u8>,
}

/// Time to Sample Box.
#[derive(Debug)]
pub struct TimeToSampleBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Entries.
    pub entries: Vec<TimeToSampleEntry>,
}

/// Time to Sample Entry.
#[derive(Debug)]
pub struct TimeToSampleEntry {
    /// Sample count.
    pub sample_count: u32,
    /// Sample delta.
    pub sample_delta: u32,
}

/// Sample to Chunk Box.
#[derive(Debug)]
pub struct SampleToChunkBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Entries.
    pub entries: Vec<SampleToChunkEntry>,
}

/// Sample to Chunk Entry.
#[derive(Debug)]
pub struct SampleToChunkEntry {
    /// First chunk.
    pub first_chunk: u32,
    /// Samples per chunk.
    pub samples_per_chunk: u32,
    /// Sample description index.
    pub sample_description_index: u32,
}

/// Sample Size Box.
#[derive(Debug)]
pub struct SampleSizeBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Sample size.
    pub sample_size: u32,
    /// Sample count.
    pub sample_count: u32,
    /// Entry sizes.
    pub entry_sizes: Vec<u32>,
}

/// Chunk Offset Box.
#[derive(Debug)]
pub enum ChunkOffsetBox {
    /// 32-bit chunk offsets.
    Stco {
        /// Version.
        version: u8,
        /// Flags.
        flags: u32,
        /// Entries.
        entries: Vec<u32>,
    },
    /// 64-bit chunk offsets.
    Co64 {
        /// Version.
        version: u8,
        /// Flags.
        flags: u32,
        /// Entries.
        entries: Vec<u64>,
    },
}

/// Sync Sample Box.
#[derive(Debug)]
pub struct SyncSampleBox {
    /// Version.
    pub version: u8,
    /// Flags.
    pub flags: u32,
    /// Entries.
    pub entries: Vec<u32>,
}

impl SampleTableBox {
    /// Parse a sample table box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);
        let mut stsd = None;
        let mut stts = None;
        let mut stsc = None;
        let mut stsz = None;
        let mut stco = None;
        let mut stss = None;

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
                b"stsd" => {
                    if stsd.is_some() {
                        return Err(Error::BmffParse("Duplicate stsd box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stsd = Some(SampleDescriptionBox::parse(&box_data)?);
                },
                b"stts" => {
                    if stts.is_some() {
                        return Err(Error::BmffParse("Duplicate stts box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stts = Some(TimeToSampleBox::parse(&box_data)?);
                },
                b"stsc" => {
                    if stsc.is_some() {
                        return Err(Error::BmffParse("Duplicate stsc box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stsc = Some(SampleToChunkBox::parse(&box_data)?);
                },
                b"stsz" => {
                    if stsz.is_some() {
                        return Err(Error::BmffParse("Duplicate stsz box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stsz = Some(SampleSizeBox::parse(&box_data)?);
                },
                b"stco" => {
                    if stco.is_some() {
                        return Err(Error::BmffParse("Duplicate stco box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stco = Some(ChunkOffsetBox::parse_stco(&box_data)?);
                },
                b"co64" => {
                    if stco.is_some() {
                        return Err(Error::BmffParse("Duplicate chunk offset box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stco = Some(ChunkOffsetBox::parse_co64(&box_data)?);
                },
                b"stss" => {
                    if stss.is_some() {
                        return Err(Error::BmffParse("Duplicate stss box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    stss = Some(SyncSampleBox::parse(&box_data)?);
                },
                _ => {
                    // Skip unknown box
                    stream.skip(data_size)?;
                }
            }
        }

        Ok(Self {
            stsd,
            stts,
            stsc,
            stsz,
            stco,
            stss,
        })
    }
}

impl SampleDescriptionBox {
    /// Parse a sample description box from a buffer.
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
                return Err(Error::BmffParse("Invalid sample description entry size".to_string()));
            }

            // Read entry type
            let mut sample_type = [0u8; 4];
            stream.read_exact(&mut sample_type)?;

            // Skip reserved
            stream.skip(6)?;

            // Read data reference index
            let _data_reference_index = stream.read_u16()?;

            // Parse entry based on type
            let entry = match &sample_type {
                b"avc1" | b"hvc1" | b"av01" => {
                    // Parse visual sample description

                    // Skip pre-defined and reserved
                    stream.skip(16)?;

                    // Read width and height
                    let width = stream.read_u16()?;
                    let height = stream.read_u16()?;

                    // Read horizontal and vertical resolution (fixed point 16.16)
                    let horizresolution = stream.read_u32()?;
                    let vertresolution = stream.read_u32()?;

                    // Skip reserved
                    stream.skip(4)?;

                    // Read frame count
                    let frame_count = stream.read_u16()?;

                    // Read compressor name
                    let mut compressorname = [0u8; 32];
                    stream.read_exact(&mut compressorname)?;
                    let compressorname_len = compressorname[0] as usize;
                    let compressorname = String::from_utf8_lossy(&compressorname[1..compressorname_len+1]).to_string();

                    // Read depth
                    let depth = stream.read_u16()?;

                    // Skip pre-defined
                    stream.skip(2)?;

                    // Read configuration boxes
                    let mut config_boxes = Vec::new();
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

                        // Read box data
                        let data = stream.read_exact_buffer(data_size)?;

                        config_boxes.push(ConfigBox {
                            box_type,
                            data: data.to_vec(),
                        });
                    }

                    SampleDescription::Visual(VisualSampleDescription {
                        sample_type,
                        width,
                        height,
                        horizresolution,
                        vertresolution,
                        frame_count,
                        compressorname,
                        depth,
                        config_boxes,
                    })
                },
                b"mp4a" => {
                    // Parse audio sample description

                    // Skip version and revision
                    stream.skip(4)?;

                    // Skip vendor
                    stream.skip(4)?;

                    // Read channel count
                    let channelcount = stream.read_u16()?;

                    // Read sample size
                    let samplesize = stream.read_u16()?;

                    // Skip compression ID and packet size
                    stream.skip(4)?;

                    // Read sample rate (fixed point 16.16)
                    let samplerate = stream.read_u32()?;

                    // Read configuration boxes
                    let mut config_boxes = Vec::new();
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

                        // Read box data
                        let data = stream.read_exact_buffer(data_size)?;

                        config_boxes.push(ConfigBox {
                            box_type,
                            data: data.to_vec(),
                        });
                    }

                    SampleDescription::Audio(AudioSampleDescription {
                        sample_type,
                        channelcount,
                        samplesize,
                        samplerate,
                        config_boxes,
                    })
                },
                _ => {
                    // Unknown sample description
                    let data_size = entry_size - 16; // 16 = size(4) + type(4) + reserved(6) + data_reference_index(2)
                    let data = stream.read_exact_buffer(data_size)?;

                    SampleDescription::Unknown {
                        sample_type,
                        data: data.to_vec(),
                    }
                }
            };

            entries.push(entry);
        }

        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}

impl TimeToSampleBox {
    /// Parse a time to sample box from a buffer.
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
            let sample_count = stream.read_u32()?;
            let sample_delta = stream.read_u32()?;

            entries.push(TimeToSampleEntry {
                sample_count,
                sample_delta,
            });
        }

        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}

impl SampleToChunkBox {
    /// Parse a sample to chunk box from a buffer.
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
            let first_chunk = stream.read_u32()?;
            let samples_per_chunk = stream.read_u32()?;
            let sample_description_index = stream.read_u32()?;

            entries.push(SampleToChunkEntry {
                first_chunk,
                samples_per_chunk,
                sample_description_index,
            });
        }

        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}

impl SampleSizeBox {
    /// Parse a sample size box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);

        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;

        // Read sample size
        let sample_size = stream.read_u32()?;

        // Read sample count
        let sample_count = stream.read_u32()? as usize;

        // Read entry sizes if sample_size is 0
        let mut entry_sizes = Vec::new();
        if sample_size == 0 {
            entry_sizes.reserve(sample_count);
            for _ in 0..sample_count {
                entry_sizes.push(stream.read_u32()?);
            }
        }

        Ok(Self {
            version,
            flags,
            sample_size,
            sample_count: sample_count as u32,
            entry_sizes,
        })
    }
}

impl ChunkOffsetBox {
    /// Parse a 32-bit chunk offset box from a buffer.
    pub fn parse_stco(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);

        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;

        // Read entry count
        let entry_count = stream.read_u32()? as usize;

        // Read entries
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            entries.push(stream.read_u32()?);
        }

        Ok(Self::Stco {
            version,
            flags,
            entries,
        })
    }

    /// Parse a 64-bit chunk offset box from a buffer.
    pub fn parse_co64(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);

        // Read version and flags
        let version = stream.read_u8()?;
        let flags = stream.read_u24()?;

        // Read entry count
        let entry_count = stream.read_u32()? as usize;

        // Read entries
        let mut entries = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            entries.push(stream.read_u64()?);
        }

        Ok(Self::Co64 {
            version,
            flags,
            entries,
        })
    }
}

impl SyncSampleBox {
    /// Parse a sync sample box from a buffer.
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
            entries.push(stream.read_u32()?);
        }

        Ok(Self {
            version,
            flags,
            entries,
        })
    }
}
