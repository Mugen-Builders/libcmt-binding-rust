use std::{ffi::OsStr, io, mem::MaybeUninit};

use crate::generated::*;
use crate::{path_to_cstring, to_io_result};

pub struct Rollup {
    inner: cmt_rollup_t,
}

impl Rollup {
    pub fn new() -> io::Result<Self> {
        let mut state = MaybeUninit::<cmt_rollup_t>::uninit();
        let rc = unsafe { cmt_rollup_init(state.as_mut_ptr()) };
        to_io_result(rc)?;
        let inner = unsafe { state.assume_init() };
        Ok(Self { inner })
    }

    pub fn emit_voucher(
        &mut self,
        address: &cmt_abi_address_t,
        value: &cmt_abi_u256_t,
        payload: &cmt_abi_bytes_t,
    ) -> io::Result<u64> {
        let mut index = 0u64;
        to_io_result(unsafe {
            cmt_rollup_emit_voucher(&mut self.inner, address, value, payload, &mut index)
        })?;
        Ok(index)
    }

    pub fn emit_delegate_call_voucher(
        &mut self,
        address: &cmt_abi_address_t,
        payload: &cmt_abi_bytes_t,
    ) -> io::Result<u64> {
        let mut index = 0u64;
        to_io_result(unsafe {
            cmt_rollup_emit_delegate_call_voucher(&mut self.inner, address, payload, &mut index)
        })?;
        Ok(index)
    }

    pub fn emit_notice(&mut self, payload: &cmt_abi_bytes_t) -> io::Result<u64> {
        let mut index = 0u64;
        to_io_result(unsafe { cmt_rollup_emit_notice(&mut self.inner, payload, &mut index) })?;
        Ok(index)
    }

    pub fn emit_report(&mut self, payload: &cmt_abi_bytes_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_rollup_emit_report(&mut self.inner, payload) })
    }

    pub fn emit_exception(&mut self, payload: &cmt_abi_bytes_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_rollup_emit_exception(&mut self.inner, payload) })
    }

    pub fn progress(&mut self, value: u32) -> io::Result<()> {
        to_io_result(unsafe { cmt_rollup_progress(&mut self.inner, value) })
    }

    pub fn read_advance_state(&mut self, advance: &mut cmt_rollup_advance_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_rollup_read_advance_state(&mut self.inner, advance) })
    }

    pub fn read_inspect_state(&mut self, inspect: &mut cmt_rollup_inspect_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_rollup_read_inspect_state(&mut self.inner, inspect) })
    }

    pub fn finish(&mut self, finish: &mut cmt_rollup_finish_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_rollup_finish(&mut self.inner, finish) })
    }

    pub fn gio_request(&mut self, request: &mut cmt_gio_t) -> io::Result<()> {
        to_io_result(unsafe { cmt_gio_request(&mut self.inner, request) })
    }

    pub fn load_merkle<P: AsRef<OsStr>>(&mut self, path: P) -> io::Result<()> {
        let path = path_to_cstring(path.as_ref())?;
        to_io_result(unsafe { cmt_rollup_load_merkle(&mut self.inner, path.as_ptr()) })
    }

    pub fn save_merkle<P: AsRef<OsStr>>(&mut self, path: P) -> io::Result<()> {
        let path = path_to_cstring(path.as_ref())?;
        to_io_result(unsafe { cmt_rollup_save_merkle(&mut self.inner, path.as_ptr()) })
    }

    pub fn reset_merkle(&mut self) {
        unsafe { cmt_rollup_reset_merkle(&mut self.inner) }
    }
}

impl Drop for Rollup {
    fn drop(&mut self) {
        unsafe { cmt_rollup_fini(&mut self.inner) }
    }
}
