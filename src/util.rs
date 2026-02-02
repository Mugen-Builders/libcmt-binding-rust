/// Utility helpers exposed by `libcmt`.
use std::{ffi::CStr, io};

use crate::generated::{
    cmt_util_debug_enabled, cmt_util_read_whole_file, cmt_util_write_whole_file,
};
use crate::to_io_result;

pub fn debug_enabled() -> bool {
    unsafe { cmt_util_debug_enabled() }
}

pub fn read_whole_file(name: &CStr, buffer: &mut [u8]) -> io::Result<usize> {
    let mut length = 0usize;
    to_io_result(unsafe {
        cmt_util_read_whole_file(
            name.as_ptr(),
            buffer.len(),
            buffer.as_mut_ptr() as *mut _,
            &mut length,
        )
    })?;
    Ok(length)
}

pub fn write_whole_file(name: &CStr, data: &[u8]) -> io::Result<()> {
    to_io_result(unsafe {
        cmt_util_write_whole_file(name.as_ptr(), data.len(), data.as_ptr() as *const _)
    })
}
