#[repr(C)]
pub struct JpegXsEncoder {
    _private: [u8; 0],
}

#[repr(C)]
pub struct JpegXsDecoder {
    _private: [u8; 0],
}

#[no_mangle]
pub extern "C" fn jpegxs_encoder_create() -> *mut JpegXsEncoder {
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn jpegxs_encoder_destroy(_encoder: *mut JpegXsEncoder) {}

#[no_mangle]
pub extern "C" fn jpegxs_decoder_create() -> *mut JpegXsDecoder {
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn jpegxs_decoder_destroy(_decoder: *mut JpegXsDecoder) {}
