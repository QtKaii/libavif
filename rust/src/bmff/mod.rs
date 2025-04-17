//! BMFF parser for the Rust implementation of libavif.
//!
//! This module provides a memory-safe implementation of the ISO Base Media File Format (BMFF)
//! parser, which is used to parse AVIF files. It handles all the box types defined in the
//! AVIF specification and provides proper error handling for malformed inputs.

mod boxes;
mod parser;
#[cfg(test)]
mod tests;

pub use parser::{Box, BoxHeader, BoxType, Parser, parse_avif, AvifMetadata};

// Re-export box types for convenience
pub use boxes::ftyp::FileTypeBox;
pub use boxes::hdlr::HandlerBox;
pub use boxes::iinf::ItemInfoBox;
pub use boxes::iloc::ItemLocationBox;
pub use boxes::ispe::ImageSpatialExtentsBox;
pub use boxes::pixi::PixelInformationBox;
pub use boxes::colr::ColourInformationBox;
pub use boxes::av1c::AV1ConfigurationBox;
pub use boxes::pitm::PrimaryItemBox;
pub use boxes::meta::MetaBox;
pub use boxes::moov::MovieBox;
pub use boxes::trak::TrackBox;
pub use boxes::mdia::MediaBox;
pub use boxes::minf::MediaInformationBox;
pub use boxes::stbl::SampleTableBox;

// Re-export color types for convenience
pub use boxes::colr::{ColorPrimaries, TransferCharacteristics, MatrixCoefficients, ColorRange};
