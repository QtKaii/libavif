//! FFI bindings for the Rust implementation of libavif.

use crate::decoder::Decoder;
use crate::error::AvifResult;
use libc::{c_int, c_uint, c_void, size_t};

/// FFI-compatible AVIF decoder.
#[repr(C)]
pub struct AvifDecoder {
    /// Maximum number of threads to use.
    pub max_threads: c_uint,
    /// Image size limit.
    pub image_size_limit: c_uint,
    /// Image dimension limit.
    pub image_dimension_limit: c_uint,
    /// Image count limit.
    pub image_count_limit: c_uint,
    /// Strict flags.
    pub strict_flags: c_uint,
    /// Requested source.
    pub requested_source: c_int,
    /// Whether to ignore EXIF metadata.
    pub ignore_exif: c_int,
    /// Whether to ignore XMP metadata.
    pub ignore_xmp: c_int,
    /// Whether to allow progressive images.
    pub allow_progressive: c_int,
    /// Whether to allow incremental decoding.
    pub allow_incremental: c_int,
    /// Internal decoder.
    decoder: *mut Decoder,
}

/// Create a new AVIF decoder.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_create() -> *mut AvifDecoder {
    let decoder = Box::new(Decoder::new());
    let ffi_decoder = Box::new(AvifDecoder {
        max_threads: decoder.max_threads,
        image_size_limit: decoder.image_size_limit,
        image_dimension_limit: decoder.image_dimension_limit,
        image_count_limit: decoder.image_count_limit,
        strict_flags: decoder.strict_flags,
        requested_source: decoder.requested_source as c_int,
        ignore_exif: decoder.ignore_exif as c_int,
        ignore_xmp: decoder.ignore_xmp as c_int,
        allow_progressive: decoder.allow_progressive as c_int,
        allow_incremental: decoder.allow_incremental as c_int,
        decoder: Box::into_raw(decoder),
    });
    Box::into_raw(ffi_decoder)
}

/// Destroy an AVIF decoder.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_destroy(decoder: *mut AvifDecoder) {
    if !decoder.is_null() {
        unsafe {
            let ffi_decoder = Box::from_raw(decoder);
            if !ffi_decoder.decoder.is_null() {
                let _ = Box::from_raw(ffi_decoder.decoder);
            }
        }
    }
}

/// Set the IO for an AVIF decoder from memory.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_set_io_memory(
    decoder: *mut AvifDecoder,
    data: *const u8,
    size: size_t,
) -> AvifResult {
    if decoder.is_null() || data.is_null() {
        return AvifResult::InvalidParameter;
    }

    unsafe {
        let ffi_decoder = &mut *decoder;
        let rust_decoder = &mut *ffi_decoder.decoder;
        let slice = std::slice::from_raw_parts(data, size);
        match rust_decoder.set_io_data(slice) {
            Ok(_) => AvifResult::Ok,
            Err(e) => e.into(),
        }
    }
}

/// Parse an AVIF file.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_parse(decoder: *mut AvifDecoder) -> AvifResult {
    if decoder.is_null() {
        return AvifResult::InvalidParameter;
    }

    unsafe {
        let ffi_decoder = &mut *decoder;
        let rust_decoder = &mut *ffi_decoder.decoder;
        match rust_decoder.parse() {
            Ok(_) => AvifResult::Ok,
            Err(e) => e.into(),
        }
    }
}

/// Get the next image from an AVIF decoder.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_next_image(
    decoder: *mut AvifDecoder,
    image: *mut c_void,
) -> AvifResult {
    if decoder.is_null() || image.is_null() {
        return AvifResult::InvalidParameter;
    }

    unsafe {
        let ffi_decoder = &mut *decoder;
        let rust_decoder = &mut *ffi_decoder.decoder;
        match rust_decoder.next_image() {
            Ok(_) => {
                // Here we would convert the Rust image to a C image
                // For now, just return success
                AvifResult::Ok
            }
            Err(e) => e.into(),
        }
    }
}

/// Get the image count from an AVIF decoder.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_get_image_count(decoder: *const AvifDecoder) -> c_int {
    if decoder.is_null() {
        return 0;
    }

    unsafe {
        let ffi_decoder = &*decoder;
        let rust_decoder = &*ffi_decoder.decoder;
        rust_decoder.image_count() as c_int
    }
}

/// Get the current image index from an AVIF decoder.
#[no_mangle]
pub extern "C" fn rust_avif_decoder_get_image_index(decoder: *const AvifDecoder) -> c_int {
    if decoder.is_null() {
        return 0;
    }

    unsafe {
        let ffi_decoder = &*decoder;
        let rust_decoder = &*ffi_decoder.decoder;
        rust_decoder.current_image_index() as c_int
    }
}
