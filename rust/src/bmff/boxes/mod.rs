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

pub use ftyp::FileTypeBox;
pub use hdlr::HandlerBox;
pub use iinf::ItemInfoBox;
pub use iloc::ItemLocationBox;
pub use ispe::ImageSpatialExtentsBox;
pub use pixi::PixelInformationBox;
pub use colr::ColourInformationBox;
pub use av1c::AV1ConfigurationBox;
pub use pitm::PrimaryItemBox;

// These are used internally
#[doc(hidden)]
pub(crate) use iinf::ItemInfoEntry;
#[doc(hidden)]
pub(crate) use iloc::{ItemLocationEntry, ItemExtent};
