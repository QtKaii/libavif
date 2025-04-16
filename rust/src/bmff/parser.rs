//! BMFF parser implementation.

use crate::error::{Error, Result};
use crate::io::ReadStream;
use crate::memory::Buffer;
use std::convert::TryFrom;
use std::fmt;

use super::boxes::*;

/// BMFF box type.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoxType([u8; 4]);

impl BoxType {
    /// Create a new BoxType from a 4-byte array.
    pub fn new(type_code: [u8; 4]) -> Self {
        Self(type_code)
    }

    /// Create a new BoxType from a string.
    pub fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err(Error::BmffParse(
                format!("Box type string must be 4 bytes long, got {}", s.len())
            ));
        }
        let mut type_code = [0u8; 4];
        type_code.copy_from_slice(s.as_bytes());
        Ok(Self(type_code))
    }

    /// Get the type code as a string.
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("????")
    }

    /// Check if this box type matches a string.
    pub fn matches(&self, s: &str) -> bool {
        if s.len() != 4 {
            return false;
        }
        self.0 == s.as_bytes()
    }

    /// Get the raw type code.
    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }
}

impl fmt::Debug for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoxType({:?})", self.as_str())
    }
}

impl fmt::Display for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&[u8]> for BoxType {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 4 {
            return Err(Error::BmffParse(
                format!("Box type must be 4 bytes long, got {}", value.len())
            ));
        }
        let mut type_code = [0u8; 4];
        type_code.copy_from_slice(value);
        Ok(Self(type_code))
    }
}

/// BMFF box header.
#[derive(Debug, Clone)]
pub struct BoxHeader {
    /// Box size.
    pub size: u64,
    /// Box type.
    pub box_type: BoxType,
    /// Extended type (for 'uuid' boxes).
    pub extended_type: Option<[u8; 16]>,
    /// Version (for full boxes).
    pub version: Option<u8>,
    /// Flags (for full boxes).
    pub flags: Option<u32>,
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
    /// The data being parsed.
    data: &'a [u8],
    /// Current position in the data.
    position: usize,
    /// Diagnostic context for error messages.
    context: String,
}

impl<'a> Parser<'a> {
    /// Create a new parser from the given data.
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            context: "BMFF".to_string(),
        }
    }

    /// Create a new parser with a specific context.
    pub fn with_context(data: &'a [u8], context: &str) -> Self {
        Self {
            data,
            position: 0,
            context: context.to_string(),
        }
    }

    /// Set the diagnostic context.
    pub fn set_context(&mut self, context: &str) {
        self.context = context.to_string();
    }

    /// Parse the next box.
    pub fn next_box(&mut self) -> Result<Option<Box>> {
        if self.position >= self.data.len() {
            return Ok(None);
        }

        // Parse box header
        if self.data.len() - self.position < 8 {
            return Err(Error::BmffParse(
                format!("{}: Not enough data to parse box header", self.context)
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
        let mut version = None;
        let mut flags = None;

        // Handle extended size
        let mut box_size = size;
        if size == 1 {
            if self.data.len() - self.position < 16 {
                return Err(Error::BmffParse(
                    format!("{}: Not enough data to parse extended size", self.context)
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
        if box_type.matches("uuid") {
            if self.data.len() - self.position < header_size + 16 {
                return Err(Error::BmffParse(
                    format!("{}: Not enough data to parse uuid type", self.context)
                ));
            }
            let mut uuid = [0u8; 16];
            uuid.copy_from_slice(
                &self.data[self.position + header_size..self.position + header_size + 16],
            );
            extended_type = Some(uuid);
            header_size += 16;
        }

        // Check for full box
        let is_full_box = matches!(
            box_type.as_str(),
            "meta" | "hdlr" | "pitm" | "iloc" | "iinf" | "iref" | "iprp" | "ipco" | "ispe" | "pixi" | "auxC" | "colr" | "av1C" | "tkhd" | "mdhd" | "mvhd"
        );

        if is_full_box {
            if self.data.len() - self.position < header_size + 4 {
                return Err(Error::BmffParse(
                    format!("{}: Not enough data to parse full box", self.context)
                ));
            }

            let version_byte = self.data[self.position + header_size];
            let flags_bytes = &self.data[self.position + header_size + 1..self.position + header_size + 4];
            let flags_value = ((flags_bytes[0] as u32) << 16) | ((flags_bytes[1] as u32) << 8) | (flags_bytes[2] as u32);

            version = Some(version_byte);
            flags = Some(flags_value);
            header_size += 4;
        }

        // Ensure we have enough data for the entire box
        if box_size == 0 {
            // Box extends to the end of the file
            box_size = self.data.len() as u64 - self.position as u64;
        } else if box_size < header_size as u64 {
            return Err(Error::BmffParse(
                format!("{}: Box size ({}) is smaller than header size ({})", self.context, box_size, header_size)
            ));
        } else if self.position + box_size as usize > self.data.len() {
            return Err(Error::BmffParse(
                format!("{}: Box extends beyond end of data", self.context)
            ));
        }

        // Create box header
        let header = BoxHeader {
            size: box_size,
            box_type,
            extended_type,
            version,
            flags,
        };

        // Extract box data
        let data_start = self.position + header_size;
        let data_end = self.position + box_size as usize;
        let data = Buffer::from_slice(&self.data[data_start..data_end]);

        // Update position
        self.position = data_end;

        Ok(Some(Box { header, data }))
    }

    /// Parse a specific box type.
    pub fn parse_box(&mut self, expected_type: &str) -> Result<Box> {
        let box_opt = self.next_box()?;
        match box_opt {
            Some(box_) => {
                if !box_.header.box_type.matches(expected_type) {
                    return Err(Error::BmffParse(
                        format!("{}: Expected box type '{}', got '{}'", self.context, expected_type, box_.header.box_type)
                    ));
                }
                Ok(box_)
            }
            None => Err(Error::BmffParse(
                format!("{}: Expected box type '{}', but no more boxes", self.context, expected_type)
            )),
        }
    }

    /// Parse a file type box.
    pub fn parse_ftyp(&mut self) -> Result<FileTypeBox> {
        let box_ = self.parse_box("ftyp")?;
        FileTypeBox::parse(&box_.data)
    }

    /// Parse a handler box.
    pub fn parse_hdlr(&mut self) -> Result<HandlerBox> {
        let box_ = self.parse_box("hdlr")?;
        HandlerBox::parse(&box_.data)
    }

    /// Parse an item info box.
    pub fn parse_iinf(&mut self) -> Result<ItemInfoBox> {
        let box_ = self.parse_box("iinf")?;
        ItemInfoBox::parse(&box_.data)
    }

    /// Parse an item location box.
    pub fn parse_iloc(&mut self) -> Result<ItemLocationBox> {
        let box_ = self.parse_box("iloc")?;
        ItemLocationBox::parse(&box_.data)
    }

    /// Parse an image spatial extents box.
    pub fn parse_ispe(&mut self) -> Result<ImageSpatialExtentsBox> {
        let box_ = self.parse_box("ispe")?;
        ImageSpatialExtentsBox::parse(&box_.data)
    }

    /// Parse a pixel information box.
    pub fn parse_pixi(&mut self) -> Result<PixelInformationBox> {
        let box_ = self.parse_box("pixi")?;
        PixelInformationBox::parse(&box_.data)
    }

    /// Parse a colour information box.
    pub fn parse_colr(&mut self) -> Result<ColourInformationBox> {
        let box_ = self.parse_box("colr")?;
        ColourInformationBox::parse(&box_.data)
    }

    /// Parse an AV1 configuration box.
    pub fn parse_av1c(&mut self) -> Result<AV1ConfigurationBox> {
        let box_ = self.parse_box("av1C")?;
        AV1ConfigurationBox::parse(&box_.data)
    }

    /// Parse a primary item box.
    pub fn parse_pitm(&mut self) -> Result<PrimaryItemBox> {
        let box_ = self.parse_box("pitm")?;
        PrimaryItemBox::parse(&box_.data)
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
                format!("{}: Position is beyond end of data", self.context)
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

    /// Create a sub-parser for a box's data.
    pub fn sub_parser<'b>(&self, box_: &'b Box) -> Parser<'b> {
        Parser::with_context(&box_.data, &format!("{}.{}", self.context, box_.header.box_type))
    }

    /// Read version and flags, and enforce that the version matches the expected value.
    pub fn read_and_enforce_version(&mut self, enforced_version: u8) -> Result<u32> {
        let mut stream = ReadStream::new(self.data);
        stream.set_position(self.position)?;

        let (version, flags) = stream.read_version_and_flags()?;
        if version != enforced_version {
            return Err(Error::BmffParse(
                format!("{}: Expecting box version {}, got version {}", self.context, enforced_version, version)
            ));
        }

        self.position = stream.position();
        Ok(flags)
    }
}

/// AVIF file metadata.
#[derive(Debug)]
pub struct AvifMetadata {
    /// File type box.
    pub ftyp: FileTypeBox,
    /// Primary item ID.
    pub primary_item_id: Option<u32>,
    /// Handler box.
    pub hdlr: Option<HandlerBox>,
    /// Item information box.
    pub iinf: Option<ItemInfoBox>,
    /// Item location box.
    pub iloc: Option<ItemLocationBox>,
    /// Image spatial extents box.
    pub ispe: Option<ImageSpatialExtentsBox>,
    /// Pixel information box.
    pub pixi: Option<PixelInformationBox>,
    /// Colour information box.
    pub colr: Option<ColourInformationBox>,
    /// AV1 configuration box.
    pub av1c: Option<AV1ConfigurationBox>,
}

/// Parse an AVIF file.
///
/// This function parses an AVIF file and returns the metadata.
/// It also validates that the file is a valid AVIF file.
pub fn parse_avif(data: &[u8]) -> Result<AvifMetadata> {
    let mut parser = Parser::new(data);

    // Parse the file type box
    let ftyp = parser.parse_ftyp()?;

    // Validate that this is an AVIF file
    if !ftyp.is_avif() {
        return Err(Error::BmffParse(
            format!("Not an AVIF file: major brand is '{}'", std::str::from_utf8(&ftyp.major_brand).unwrap_or("????"))
        ));
    }

    // Parse the meta box
    let mut primary_item_id = None;
    let mut hdlr = None;
    let mut iinf = None;
    let mut iloc = None;
    let mut ispe = None;
    let mut pixi = None;
    let mut colr = None;
    let mut av1c = None;

    // Continue parsing boxes
    while let Some(box_) = parser.next_box()? {
        match box_.header.box_type.as_str() {
            "meta" => {
                // Parse meta box contents
                let mut meta_parser = parser.sub_parser(&box_);

                // Read and enforce version 0
                let _flags = meta_parser.read_and_enforce_version(0)?;

                // Parse meta box contents
                while let Some(meta_box) = meta_parser.next_box()? {
                    match meta_box.header.box_type.as_str() {
                        "hdlr" => {
                            hdlr = Some(HandlerBox::parse(&meta_box.data)?);
                        },
                        "pitm" => {
                            let pitm = PrimaryItemBox::parse(&meta_box.data)?;
                            primary_item_id = Some(pitm.item_id);
                        },
                        "iloc" => {
                            iloc = Some(ItemLocationBox::parse(&meta_box.data)?);
                        },
                        "iinf" => {
                            iinf = Some(ItemInfoBox::parse(&meta_box.data)?);
                        },
                        "iprp" => {
                            // Parse item properties box
                            let mut iprp_parser = meta_parser.sub_parser(&meta_box);

                            // Parse ipco box
                            if let Some(ipco_box) = iprp_parser.next_box()? {
                                if ipco_box.header.box_type.as_str() == "ipco" {
                                    // Parse item property container box
                                    let mut ipco_parser = iprp_parser.sub_parser(&ipco_box);

                                    // Parse property boxes
                                    while let Some(prop_box) = ipco_parser.next_box()? {
                                        match prop_box.header.box_type.as_str() {
                                            "ispe" => {
                                                ispe = Some(ImageSpatialExtentsBox::parse(&prop_box.data)?);
                                            },
                                            "pixi" => {
                                                pixi = Some(PixelInformationBox::parse(&prop_box.data)?);
                                            },
                                            "colr" => {
                                                colr = Some(ColourInformationBox::parse(&prop_box.data)?);
                                            },
                                            "av1C" => {
                                                av1c = Some(AV1ConfigurationBox::parse(&prop_box.data)?);
                                            },
                                            _ => {
                                                // Ignore other property boxes
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => {
                            // Ignore other meta box contents
                        }
                    }
                }
            },
            _ => {
                // Ignore other top-level boxes
            }
        }
    }

    Ok(AvifMetadata {
        ftyp,
        primary_item_id,
        hdlr,
        iinf,
        iloc,
        ispe,
        pixi,
        colr,
        av1c,
    })
}
