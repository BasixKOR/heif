use crate::error::{Error, Result};
use libheif_sys::{
    heif_context, heif_context_alloc, heif_context_free, heif_context_read_from_reader,
    heif_error_code_heif_error_Ok, heif_reader,
};
use std::{ffi::c_void, io::prelude::*};

mod heif_impl {
    use libheif_sys::{heif_reader, heif_reader_grow_status};
    use std::io::prelude::*;
    use std::{ffi::c_void, io::SeekFrom, os::raw::c_int};

    pub fn get_reader<R: Read + Seek>() -> heif_reader {
        heif_reader {
            reader_api_version: 1,
            get_position: Some(get_position::<R>),
            read: Some(read::<R>),
            seek: Some(seek::<R>),
            wait_for_file_size: Some(wait_for_file_size::<R>),
        }
    }

    unsafe extern "C" fn get_position<R: Seek>(userdata: *mut c_void) -> i64 {
        (*(userdata as *mut R)).seek(SeekFrom::Current(0)).unwrap() as _ // non-negative seek is infailable
    }

    unsafe extern "C" fn read<R: Read>(
        data: *mut c_void,
        size: usize,
        userdata: *mut c_void,
    ) -> c_int {
        unimplemented!();
    }

    unsafe extern "C" fn seek<R: Seek>(position: i64, userdata: *mut c_void) -> c_int {
        (*(userdata as *mut R))
            .seek(SeekFrom::Current(position))
            .unwrap() as _ // non-negative seek is infailable
    }

    unsafe extern "C" fn wait_for_file_size<R>(
        target_size: i64,
        userdata: *mut c_void,
    ) -> heif_reader_grow_status {
        unimplemented!();
    }
}

/// Libheif Decoder
pub struct Decoder<R> {
    reader: R,
    context: *mut heif_context,
    heif_reader: Box<heif_reader>,
}

impl<R: Read + Seek> Decoder<R> {
    /// Create a new `Decoder` using the reader `reader`.
    pub fn new(reader: R) -> Result<Self> {
        let context = unsafe { heif_context_alloc() };
        let mut heif_reader = Box::new(heif_impl::get_reader::<R>());

        unsafe {
            let reader = &reader as *const _ as *mut c_void; // userdata (should) just pass the pointer, not mutating it
            let result = heif_context_read_from_reader(
                context,
                heif_reader.as_mut(),
                reader,
                std::ptr::null(),
            );
            if result.code != heif_error_code_heif_error_Ok {
                return Err(Error::HeifError(result));
            }
        }

        Ok(Decoder {
            reader,
            context,
            heif_reader,
        })
    }

    pub fn decode(&mut self) -> Result<Vec<u8>> {
        unimplemented!()
    }
}

impl<R> Drop for Decoder<R> {
    fn drop(&mut self) {
        unsafe { heif_context_free(self.context) };
    }
}
