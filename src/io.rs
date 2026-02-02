use std::{io, mem::MaybeUninit, slice};

use crate::generated::*;
use crate::{buffer_len, to_io_result};

pub struct IoDriver {
    inner: cmt_io_driver_t,
}

impl IoDriver {
    pub fn new() -> io::Result<Self> {
        let mut driver = MaybeUninit::<cmt_io_driver_t>::uninit();
        let rc = unsafe { cmt_io_init(driver.as_mut_ptr()) };
        to_io_result(rc)?;
        let inner = unsafe { driver.assume_init() };
        Ok(Self { inner })
    }

    pub fn tx_buffer(&mut self) -> &mut [u8] {
        let buf = unsafe { cmt_io_get_tx(&mut self.inner) };
        unsafe { slice::from_raw_parts_mut(buf.begin, buffer_len(&buf)) }
    }

    pub fn rx_buffer(&mut self) -> &[u8] {
        let buf = unsafe { cmt_io_get_rx(&mut self.inner) };
        unsafe { slice::from_raw_parts(buf.begin, buffer_len(&buf)) }
    }

    pub fn yield_request(&mut self, request: &mut cmt_io_yield_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_io_yield(&mut self.inner, request) })
    }
}

impl Drop for IoDriver {
    fn drop(&mut self) {
        unsafe { cmt_io_fini(&mut self.inner) }
    }
}
