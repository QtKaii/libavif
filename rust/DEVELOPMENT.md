# Development Guide for libavif Rust Implementation

This guide provides information for developers working on the Rust implementation of libavif.

## Project Structure

The Rust implementation is organized into the following modules:

- `avif_image.rs`: AVIF image representation
- `bmff.rs`: BMFF parser
- `decoder.rs`: AVIF decoder
- `error.rs`: Error handling
- `ffi.rs`: FFI bindings
- `io.rs`: I/O handling
- `memory.rs`: Memory management

## Building

The Rust implementation is built as part of the normal CMake build process:

```bash
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug
cmake --build .
```

## Testing

To run the Rust tests:

```bash
cd rust
cargo test
```

## Adding New Features

When adding new features to the Rust implementation:

1. Implement the feature in the appropriate Rust module
2. Add unit tests for the feature
3. Update the FFI bindings in `ffi.rs`
4. Update the cbindgen.toml file if necessary
5. Update the documentation

## FFI Guidelines

When working with FFI:

1. Use `#[repr(C)]` for all types that cross the FFI boundary
2. Use `libc` types for C-compatible types
3. Handle null pointers and error conditions
4. Ensure proper memory management across the FFI boundary
5. Document all FFI functions

## Memory Management

The Rust implementation uses Rust's ownership model to ensure memory safety:

1. Use `Box` for heap-allocated objects
2. Use `Vec` for dynamically-sized arrays
3. Use RAII for resource management
4. Avoid unsafe code when possible
5. When unsafe code is necessary, document the safety invariants

## Error Handling

Error handling in the Rust implementation follows these principles:

1. Use `Result<T, Error>` for functions that can fail
2. Use the `thiserror` crate for error types
3. Provide detailed error messages
4. Convert Rust errors to C error codes at the FFI boundary

## Documentation

All code should be documented:

1. Use doc comments (`///`) for public items
2. Use module-level doc comments (`//!`) for modules
3. Include examples where appropriate
4. Document safety invariants for unsafe code

## Code Style

The Rust implementation follows the standard Rust code style:

1. Use `rustfmt` to format code
2. Use `clippy` to catch common mistakes
3. Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
4. Use meaningful variable and function names

## Performance Considerations

When writing performance-critical code:

1. Use benchmarks to measure performance
2. Avoid unnecessary allocations
3. Use efficient data structures
4. Consider using SIMD instructions for performance-critical operations
5. Profile the code to identify bottlenecks

## Security Considerations

Security is a primary goal of the Rust implementation:

1. Use Rust's type system to prevent memory safety issues
2. Validate all input data
3. Use fuzzing to test parsing code
4. Avoid integer overflows
5. Handle out-of-memory conditions gracefully
