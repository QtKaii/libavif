//! # libavif Rust Implementation
//!
//! This crate provides a memory-safe implementation of security-critical
//! components of libavif, focusing on the BMFF parser, decoder core,
//! memory management, and stream handling.

// Module declarations
mod avif_image;
mod bmff;
mod decoder;
mod error;
mod ffi;
mod io;
mod io_tests;
mod memory;

// Re-exports for FFI
pub use ffi::*;

// Re-exports for convenience
pub use bmff::{BoxType, Box, BoxHeader, Parser as BmffParser, parse_avif, AvifMetadata};
pub use bmff::{ColorPrimaries, TransferCharacteristics, MatrixCoefficients, ColorRange};
pub use error::{Error, Result};
pub use io::{BoxMarker, ReadStream, WriteStream};
pub use memory::{Buffer, RwData};
