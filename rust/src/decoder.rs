//! AVIF decoder for the Rust implementation of libavif.

use crate::avif_image::AvifImage;
use crate::bmff::Parser;
use crate::error::{Error, Result};
use std::io::Read;

/// AVIF decoder source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecoderSource {
    /// Automatically determine the source.
    Auto,
    /// Use the primary item.
    PrimaryItem,
    /// Use the tracks.
    Tracks,
}

/// AVIF decoder.
#[derive(Debug)]
pub struct Decoder {
    /// Maximum number of threads to use.
    pub max_threads: u32,
    /// Image size limit.
    pub image_size_limit: u32,
    /// Image dimension limit.
    pub image_dimension_limit: u32,
    /// Image count limit.
    pub image_count_limit: u32,
    /// Strict flags.
    pub strict_flags: u32,
    /// Requested source.
    pub requested_source: DecoderSource,
    /// Whether to ignore EXIF metadata.
    pub ignore_exif: bool,
    /// Whether to ignore XMP metadata.
    pub ignore_xmp: bool,
    /// Whether to allow progressive images.
    pub allow_progressive: bool,
    /// Whether to allow incremental decoding.
    pub allow_incremental: bool,
    /// Input data.
    data: Vec<u8>,
    /// Current image index.
    current_image_index: usize,
    /// Total number of images.
    image_count: usize,
}

impl Decoder {
    /// Create a new decoder.
    pub fn new() -> Self {
        Self {
            max_threads: 1,
            image_size_limit: 16777216, // 4096 * 4096
            image_dimension_limit: 32768,
            image_count_limit: 100000,
            strict_flags: 0,
            requested_source: DecoderSource::Auto,
            ignore_exif: false,
            ignore_xmp: false,
            allow_progressive: false,
            allow_incremental: false,
            data: Vec::new(),
            current_image_index: 0,
            image_count: 0,
        }
    }

    /// Set the input data.
    pub fn set_io_data(&mut self, data: &[u8]) -> Result<()> {
        self.data = data.to_vec();
        self.current_image_index = 0;
        self.image_count = 0;
        Ok(())
    }

    /// Set the input data from a reader.
    pub fn set_io_reader<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).map_err(Error::Io)?;
        self.set_io_data(&data)
    }

    /// Parse the input data.
    pub fn parse(&mut self) -> Result<()> {
        if self.data.is_empty() {
            return Err(Error::InvalidParameter("No input data".to_string()));
        }

        // Create a BMFF parser
        let mut parser = Parser::new(&self.data);

        // Parse the BMFF structure
        while let Some(box_) = parser.next_box()? {
            // Process the box based on its type
            match box_.header.box_type.as_str() {
                "ftyp" => {
                    // Process file type box
                    // This is where we would determine if this is an AVIF file
                }
                "meta" => {
                    // Process meta box
                    // This is where we would extract metadata and item information
                }
                "moov" => {
                    // Process movie box
                    // This is where we would extract track information for image sequences
                }
                _ => {
                    // Ignore other box types
                }
            }
        }

        // For now, just assume we have one image
        self.image_count = 1;

        Ok(())
    }

    /// Decode the next image.
    pub fn next_image(&mut self) -> Result<AvifImage> {
        if self.current_image_index >= self.image_count {
            return Err(Error::Decode("No more images".to_string()));
        }

        // Increment the current image index
        self.current_image_index += 1;

        // For now, just return a dummy image
        let mut image = AvifImage::new();
        image.width = 1;
        image.height = 1;
        image.depth = 8;
        image.allocate_planes(0x7)?; // Allocate color planes

        Ok(image)
    }

    /// Get the number of images.
    pub fn image_count(&self) -> usize {
        self.image_count
    }

    /// Get the current image index.
    pub fn current_image_index(&self) -> usize {
        self.current_image_index
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
