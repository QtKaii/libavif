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
mod memory;

// Re-exports for FFI
pub use ffi::*;
