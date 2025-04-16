# libavif Rust Rewrite Plan

This document outlines the comprehensive plan for rewriting security-critical components of libavif in Rust, focusing on the BMFF parser, decoder core, memory management, and stream handling.

## Overview

The goal is to incrementally rewrite libavif's security-critical components in Rust to improve memory safety and security while maintaining compatibility with the original C implementation.

## Progress Legend

- ✅ Complete
- 🔄 In Progress
- ❌ Not Started

## Phase 1: Complete the BMFF Parser Implementation

### 1.1 Add Box Writing Support (High Priority) ✅

- **Tasks:**
  - ✅ Implement `BoxMarker` type for tracking box positions
  - ✅ Add `write_box` and `write_full_box` methods to `WriteStream`
  - ✅ Implement `finish_box` method for updating box sizes
  - ✅ Add support for writing common box types (ftyp, meta, etc.)
  - ✅ Create tests for box writing functionality

### 1.2 Implement Bit-level Operations (Medium Priority) ✅

- **Tasks:**
  - ✅ Add bit-level reading methods to `ReadStream` (read_bits_u8, read_bits_u16, read_bits_u32)
  - ✅ Add bit-level writing methods to `WriteStream` (write_bits_u8, write_bits_u16, write_bits_u32)
  - ✅ Implement bit skipping functionality
  - ✅ Add tests for bit-level operations

### 1.3 Add Support for Additional Box Types (Medium Priority) 🔄

- **Tasks:**
  - ❌ Implement `moov` box and related structures for AVIF sequences
  - ❌ Implement `trak`, `mdia`, `minf` boxes for track information
  - ❌ Implement `stbl`, `stsd`, `stts`, `stsc`, `stsz`, `stco` boxes for sample tables
  - ❌ Add support for `moof` and `traf` boxes for fragments
  - ❌ Create tests for new box types

### 1.4 Enhance Error Handling and Diagnostics (Medium Priority) 🔄

- **Tasks:**
  - ❌ Implement a `Diagnostics` struct similar to the C implementation
  - ❌ Integrate diagnostics with stream operations
  - ❌ Add more detailed error messages and context
  - ❌ Improve error propagation across the codebase
  - ❌ Create tests for error handling

## Phase 2: Complete the Stream Handling Implementation

### 2.1 Enhance WriteStream Implementation (High Priority) ✅

- **Tasks:**
  - ✅ Add support for dynamic buffer resizing
  - ✅ Implement methods for writing various data types (u8, u16, u32, u64, strings)
  - ✅ Add support for writing zeros and padding
  - ✅ Implement methods for writing box headers and full boxes
  - ✅ Create tests for WriteStream functionality

### 2.2 Implement Advanced Stream Features (Medium Priority) 🔄

- **Tasks:**
  - ✅ Add support for seeking within streams
  - ✅ Implement stream position tracking
  - ❌ Add support for stream cloning and sub-streams
  - ❌ Implement stream concatenation
  - ❌ Create tests for advanced stream features

### 2.3 Optimize Stream Performance (Low Priority) 🔄

- **Tasks:**
  - ❌ Benchmark stream operations
  - ❌ Optimize buffer management for performance
  - ❌ Implement buffer pooling or reuse
  - ❌ Reduce memory allocations where possible
  - ❌ Compare performance with C implementation

## Phase 3: Complete the Decoder Implementation

### 3.1 Implement Core Decoder Functionality (High Priority) 🔄

- **Tasks:**
  - ❌ Connect decoder with BMFF parser
  - ❌ Implement image decoding pipeline
  - ❌ Add support for AV1 configuration
  - ❌ Implement color conversion
  - ❌ Create tests for decoder functionality

### 3.2 Add Support for AVIF Features (Medium Priority) 🔄

- **Tasks:**
  - ❌ Implement alpha channel support
  - ❌ Add support for color profiles (ICC, NCLX)
  - ❌ Implement grid images
  - ❌ Add support for AVIF sequences
  - ❌ Create tests for AVIF features

### 3.3 Optimize Decoder Performance (Low Priority) 🔄

- **Tasks:**
  - ❌ Benchmark decoder operations
  - ❌ Optimize memory usage
  - ❌ Implement parallel decoding
  - ❌ Reduce memory allocations
  - ❌ Compare performance with C implementation

## Phase 4: Complete the FFI Integration

### 4.1 Implement FFI Bindings (High Priority) 🔄

- **Tasks:**
  - ❌ Create C-compatible structs for all Rust types
  - ❌ Implement FFI functions for all public API methods
  - ❌ Add proper error handling across the FFI boundary
  - ❌ Ensure memory safety for all FFI operations
  - ❌ Create tests for FFI bindings

### 4.2 Integrate with C Codebase (Medium Priority) 🔄

- **Tasks:**
  - ❌ Update CMake configuration for Rust integration
  - ❌ Implement C wrappers for Rust functions
  - ❌ Add support for C callbacks
  - ❌ Ensure proper memory management across the boundary
  - ❌ Create tests for C integration

### 4.3 Ensure API Compatibility (Medium Priority) 🔄

- **Tasks:**
  - ❌ Verify that all C API functions have Rust equivalents
  - ❌ Ensure consistent behavior between C and Rust implementations
  - ❌ Add compatibility tests
  - ❌ Document any differences or limitations
  - ❌ Create migration guide for C users

## Phase 5: Testing and Documentation

### 5.1 Enhance Testing (High Priority) 🔄

- **Tasks:**
  - 🔄 Add comprehensive unit tests for all components
  - ❌ Implement integration tests with the C codebase
  - ❌ Add fuzzing tests for the parser
  - ❌ Create benchmarks for performance comparison
  - ❌ Set up continuous integration

### 5.2 Improve Documentation (Medium Priority) 🔄

- **Tasks:**
  - 🔄 Document all public API functions
  - ❌ Add examples for common use cases
  - ❌ Create API reference documentation
  - ❌ Write developer guide for contributors
  - ❌ Document design decisions and architecture

### 5.3 Security Audit (High Priority) 🔄

- **Tasks:**
  - 🔄 Review code for security vulnerabilities
  - 🔄 Ensure proper bounds checking
  - 🔄 Verify memory safety
  - ❌ Test with malformed inputs
  - ❌ Document security considerations

## Immediate Next Steps

1. **Implement Additional Box Types**
   - Start with `moov` box implementation
   - Add support for track-related boxes
   - Create tests for new box types

2. **Enhance Error Handling**
   - Implement `Diagnostics` struct
   - Integrate with stream operations
   - Improve error messages and context

3. **Connect Decoder with BMFF Parser**
   - Implement image decoding pipeline
   - Add support for AV1 configuration
   - Create tests for decoder functionality

## Success Criteria

The Rust rewrite will be considered successful when:

1. All security-critical components (BMFF parser, decoder core, memory management, stream handling) are implemented in Rust
2. The implementation passes all tests, including fuzzing tests
3. The API is compatible with the original C implementation
4. The performance is comparable to or better than the C implementation
5. The code is well-documented and maintainable

## Design Principles

1. **Safety First**: Leverage Rust's memory safety guarantees to eliminate memory-related vulnerabilities
2. **Compatibility**: Maintain compatibility with the original C implementation
3. **Performance**: Ensure performance is comparable to or better than the C implementation
4. **Maintainability**: Write clean, well-documented code that is easy to maintain
5. **Incremental Approach**: Replace components one at a time to minimize risk

## References

- [Original libavif Repository](https://github.com/AOMediaCodec/libavif)
- [AVIF Specification](https://aomediacodec.github.io/av1-avif/)
- [ISO Base Media File Format Specification](https://www.iso.org/standard/68960.html)
