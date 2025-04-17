//! BMFF box type implementations.
//!
//! This module contains implementations for the various box types defined in the BMFF specification.

pub mod ftyp;
pub mod hdlr;
pub mod iinf;
pub mod iloc;
pub mod ispe;
pub mod pixi;
pub mod colr;
pub mod av1c;
pub mod pitm;
pub mod meta;
pub mod moov;
pub mod trak;
pub mod mdia;
pub mod minf;
pub mod stbl;
pub mod mdat;
pub mod free;
pub mod idat;

#[cfg(test)]
mod tests;

pub use ftyp::FileTypeBox;
pub use hdlr::HandlerBox;
pub use iinf::ItemInfoBox;
pub use iloc::ItemLocationBox;
pub use ispe::ImageSpatialExtentsBox;
pub use pixi::PixelInformationBox;
pub use colr::ColourInformationBox;
pub use av1c::AV1ConfigurationBox;
pub use pitm::PrimaryItemBox;
pub use mdat::MediaDataBox;
pub use free::FreeSpaceBox;
pub use idat::ItemDataBox;