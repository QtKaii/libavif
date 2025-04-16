//! Colour Information Box implementation.

use crate::error::{Error, Result};
use crate::io::ReadStream;

/// Color primaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorPrimaries {
    /// Unknown color primaries.
    Unknown = 0,
    /// BT.709 color primaries.
    BT709 = 1,
    /// Unspecified color primaries.
    Unspecified = 2,
    /// Reserved color primaries.
    Reserved = 3,
    /// BT.470M color primaries.
    BT470M = 4,
    /// BT.470BG color primaries.
    BT470BG = 5,
    /// BT.601 color primaries.
    BT601 = 6,
    /// SMPTE 240 color primaries.
    SMPTE240 = 7,
    /// Generic film color primaries.
    GenericFilm = 8,
    /// BT.2020 color primaries.
    BT2020 = 9,
    /// XYZ color primaries.
    XYZ = 10,
    /// SMPTE RP 431-2 color primaries.
    SMPTE431 = 11,
    /// SMPTE EG 432-1 color primaries.
    SMPTE432 = 12,
    /// EBU Tech 3213-E color primaries.
    EBU3213 = 22,
}

impl From<u16> for ColorPrimaries {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::BT709,
            2 => Self::Unspecified,
            3 => Self::Reserved,
            4 => Self::BT470M,
            5 => Self::BT470BG,
            6 => Self::BT601,
            7 => Self::SMPTE240,
            8 => Self::GenericFilm,
            9 => Self::BT2020,
            10 => Self::XYZ,
            11 => Self::SMPTE431,
            12 => Self::SMPTE432,
            22 => Self::EBU3213,
            _ => Self::Unknown,
        }
    }
}

/// Transfer characteristics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferCharacteristics {
    /// Unknown transfer characteristics.
    Unknown = 0,
    /// BT.709 transfer characteristics.
    BT709 = 1,
    /// Unspecified transfer characteristics.
    Unspecified = 2,
    /// Reserved transfer characteristics.
    Reserved = 3,
    /// BT.470M transfer characteristics.
    BT470M = 4,
    /// BT.470BG transfer characteristics.
    BT470BG = 5,
    /// BT.601 transfer characteristics.
    BT601 = 6,
    /// SMPTE 240 transfer characteristics.
    SMPTE240 = 7,
    /// Linear transfer characteristics.
    Linear = 8,
    /// Logarithmic (100:1 range) transfer characteristics.
    Log100 = 9,
    /// Logarithmic (100 * sqrt(10):1 range) transfer characteristics.
    Log100Sqrt10 = 10,
    /// IEC 61966-2-4 transfer characteristics.
    IEC61966 = 11,
    /// BT.1361 transfer characteristics.
    BT1361 = 12,
    /// sRGB transfer characteristics.
    SRGB = 13,
    /// BT.2020 10-bit transfer characteristics.
    BT2020_10Bit = 14,
    /// BT.2020 12-bit transfer characteristics.
    BT2020_12Bit = 15,
    /// SMPTE ST 2084 (PQ) transfer characteristics.
    SMPTE2084 = 16,
    /// SMPTE ST 428-1 transfer characteristics.
    SMPTE428 = 17,
    /// HLG transfer characteristics.
    HLG = 18,
}

impl From<u16> for TransferCharacteristics {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::BT709,
            2 => Self::Unspecified,
            3 => Self::Reserved,
            4 => Self::BT470M,
            5 => Self::BT470BG,
            6 => Self::BT601,
            7 => Self::SMPTE240,
            8 => Self::Linear,
            9 => Self::Log100,
            10 => Self::Log100Sqrt10,
            11 => Self::IEC61966,
            12 => Self::BT1361,
            13 => Self::SRGB,
            14 => Self::BT2020_10Bit,
            15 => Self::BT2020_12Bit,
            16 => Self::SMPTE2084,
            17 => Self::SMPTE428,
            18 => Self::HLG,
            _ => Self::Unknown,
        }
    }
}

/// Matrix coefficients.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixCoefficients {
    /// Identity matrix coefficients.
    Identity = 0,
    /// BT.709 matrix coefficients.
    BT709 = 1,
    /// Unspecified matrix coefficients.
    Unspecified = 2,
    /// Reserved matrix coefficients.
    Reserved = 3,
    /// FCC matrix coefficients.
    FCC = 4,
    /// BT.470BG matrix coefficients.
    BT470BG = 5,
    /// BT.601 matrix coefficients.
    BT601 = 6,
    /// SMPTE 240 matrix coefficients.
    SMPTE240 = 7,
    /// YCgCo matrix coefficients.
    YCgCo = 8,
    /// BT.2020 non-constant luminance matrix coefficients.
    BT2020NCL = 9,
    /// BT.2020 constant luminance matrix coefficients.
    BT2020CL = 10,
    /// SMPTE ST 2085 matrix coefficients.
    SMPTE2085 = 11,
    /// Chromaticity-derived non-constant luminance matrix coefficients.
    ChromatNCL = 12,
    /// Chromaticity-derived constant luminance matrix coefficients.
    ChromatCL = 13,
    /// ICtCp matrix coefficients.
    ICtCp = 14,
}

impl From<u16> for MatrixCoefficients {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Identity,
            1 => Self::BT709,
            2 => Self::Unspecified,
            3 => Self::Reserved,
            4 => Self::FCC,
            5 => Self::BT470BG,
            6 => Self::BT601,
            7 => Self::SMPTE240,
            8 => Self::YCgCo,
            9 => Self::BT2020NCL,
            10 => Self::BT2020CL,
            11 => Self::SMPTE2085,
            12 => Self::ChromatNCL,
            13 => Self::ChromatCL,
            14 => Self::ICtCp,
            _ => Self::Unspecified,
        }
    }
}

/// Color range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorRange {
    /// Limited range (16-235 for 8-bit).
    Limited = 0,
    /// Full range (0-255 for 8-bit).
    Full = 1,
}

impl From<u16> for ColorRange {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Limited,
            1 => Self::Full,
            _ => Self::Full,
        }
    }
}

/// NCLX color information.
#[derive(Debug, Clone)]
pub struct NclxColorInformation {
    /// Color primaries.
    pub color_primaries: ColorPrimaries,
    /// Transfer characteristics.
    pub transfer_characteristics: TransferCharacteristics,
    /// Matrix coefficients.
    pub matrix_coefficients: MatrixCoefficients,
    /// Full range flag.
    pub full_range: bool,
}

/// Colour information box.
#[derive(Debug, Clone)]
pub struct ColourInformationBox {
    /// Color type.
    pub color_type: [u8; 4],
    /// ICC profile (if color_type is 'prof').
    pub icc_profile: Option<Vec<u8>>,
    /// NCLX color information (if color_type is 'nclx').
    pub nclx: Option<NclxColorInformation>,
}

impl ColourInformationBox {
    /// Parse a colour information box from a buffer.
    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut stream = ReadStream::new(data);

        // Read color type
        let mut color_type = [0u8; 4];
        stream.read_exact(&mut color_type)?;

        // Parse based on color type
        let (icc_profile, nclx) = match &color_type {
            b"nclx" => {
                // Parse NCLX color information
                let color_primaries = stream.read_u16()?;
                let transfer_characteristics = stream.read_u16()?;
                let matrix_coefficients = stream.read_u16()?;
                let full_range_flag = stream.read_u8()?;

                let nclx = NclxColorInformation {
                    color_primaries: ColorPrimaries::from(color_primaries),
                    transfer_characteristics: TransferCharacteristics::from(transfer_characteristics),
                    matrix_coefficients: MatrixCoefficients::from(matrix_coefficients),
                    full_range: full_range_flag & 0x80 != 0,
                };

                (None, Some(nclx))
            },
            b"prof" | b"rICC" => {
                // Parse ICC profile (or restricted ICC profile)
                let icc_data = stream.remaining().to_vec();
                (Some(icc_data), None)
            },
            _ => {
                return Err(Error::BmffParse(
                    format!("Unknown color type: {:?}", std::str::from_utf8(&color_type).unwrap_or("????"))
                ));
            }
        };

        Ok(Self {
            color_type,
            icc_profile,
            nclx,
        })
    }

    /// Check if this is an ICC profile.
    pub fn is_icc(&self) -> bool {
        self.color_type == *b"prof" || self.color_type == *b"rICC"
    }

    /// Check if this is an NCLX profile.
    pub fn is_nclx(&self) -> bool {
        self.color_type == *b"nclx"
    }
}
