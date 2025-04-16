//! BMFF parser for the Rust implementation of libavif.
//!
//! This module provides a memory-safe implementation of the ISO Base Media File Format (BMFF)
//! parser, which is used to parse AVIF files. It handles all the box types defined in the
//! AVIF specification and provides proper error handling for malformed inputs.

mod box_types;
mod parser;
#[cfg(test)]
mod tests;

pub use parser::{Box, BoxHeader, BoxType, Parser, parse_avif};
pub use box_types::{FileTypeBox, HandlerBox, ItemInfoBox, ItemInfoEntry};
