mod generated {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use generated::*;

use std::{
    ffi::{CString, OsStr},
    io as std_io,
    os::unix::ffi::OsStrExt,
};

pub(crate) fn to_io_result(rc: i32) -> std_io::Result<()> {
    if rc == 0 {
        Ok(())
    } else {
        Err(std_io::Error::from_raw_os_error(-rc))
    }
}

pub(crate) fn path_to_cstring(path: &OsStr) -> std_io::Result<CString> {
    CString::new(path.as_bytes()).map_err(|_| {
        std_io::Error::new(
            std_io::ErrorKind::InvalidInput,
            "path contains interior null",
        )
    })
}

pub(crate) fn buffer_len(buf: &cmt_buf_t) -> usize {
    (buf.end as usize).saturating_sub(buf.begin as usize)
}

pub mod abi;
pub mod buf;
pub mod io;
pub mod keccak;
pub mod merkle;
pub mod rollup;
pub mod util;
