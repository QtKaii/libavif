//! Movie Box implementation.
//!
//! The Movie Box (`moov`) is a container box for all the metadata related to a movie.
//! It contains a Movie Header Box (`mvhd`) and one or more Track Boxes (`trak`).

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Movie Box.
#[derive(Debug)]
pub struct MovieBox {
    /// Movie Header Box.
    pub mvhd: MovieHeaderBox,
    /// Track Boxes.
    pub tracks: Vec<super::trak::TrackBox>,
}

/// Movie Header Box.
#[derive(Debug)]
pub struct MovieHeaderBox {
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
    /// Next track ID.
    pub next_track_id: u32,
}

impl MovieBox {
    /// Parse a movie box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        println!("MovieBox::parse data length: {}", data.len());
        let mut stream = ReadStream::new(data);
        let mut mvhd = None;
        let mut tracks = Vec::new();

        while stream.has_more() {
            // Read box header
            if stream.remaining_bytes() < 8 {
                println!("Not enough bytes for box header: {}", stream.remaining_bytes());
                break;
            }

            let box_size = stream.read_u32()? as usize;
            if box_size < 8 {
                return Err(Error::BmffParse("Invalid box size".to_string()));
            }

            let mut box_type = [0u8; 4];
            stream.read_exact(&mut box_type)?;

            // Calculate box data size
            let data_size = box_size - 8;
            println!("Found box: {} with size: {}", String::from_utf8_lossy(&box_type), box_size);

            // Parse box based on type
            match &box_type {
                b"mvhd" => {
                    if mvhd.is_some() {
                        return Err(Error::BmffParse("Duplicate mvhd box".to_string()));
                    }

                    let box_data = stream.read_exact_buffer(data_size)?;
                    mvhd = Some(MovieHeaderBox::parse(&box_data)?);
                },
                b"trak" => {
                    let box_data = stream.read_exact_buffer(data_size)?;
                    let track = super::trak::TrackBox::parse(&box_data)?;
                    tracks.push(track);
                },
                _ => {
                    // Skip unknown box
                    println!("Skipping unknown box: {}", String::from_utf8_lossy(&box_type));
                    stream.skip(data_size)?;
                }
            }
        }

        // Ensure we have a movie header box
        let mvhd = mvhd.ok_or_else(|| Error::BmffParse("Missing mvhd box".to_string()))?;

        Ok(Self {
            mvhd,
            tracks,
        })
    }
}

impl MovieHeaderBox {
    /// Parse a movie header box from a buffer.
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

        // Read rate and volume
        let _rate = stream.read_u32()?;
        let _volume = stream.read_u16()?;

        // Skip reserved
        stream.skip(2)?;
        stream.skip(8)?;

        // Skip matrix
        stream.skip(36)?;

        // Skip pre-defined
        stream.skip(24)?;

        // Read next track ID
        let next_track_id = stream.read_u32()?;

        Ok(Self {
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            next_track_id,
        })
    }
}
