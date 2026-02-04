use std::{ffi::OsStr, io, mem::MaybeUninit, slice};

use crate::generated::*;
use crate::{path_to_cstring, to_io_result};

pub struct Rollup {
    inner: cmt_rollup_t,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub chain_id: u64,
    pub block_number: u64,
    pub block_timestamp: u64,
    pub index: u64,
}

#[derive(Debug, Clone)]
pub struct Advance {
    pub metadata: Metadata,
    pub app_contract: [u8; 20],
    pub msg_sender: [u8; 20],
    pub prev_randao: [u8; 32],
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Inspect {
    pub payload: Vec<u8>,
}

fn convert_advance(c_adv: cmt_rollup_advance_t) -> Advance {
    // NOTE: field names may differ slightly depending on bindgen output.
    // Adjust as needed.

    let metadata = Metadata {
        chain_id: c_adv.chain_id as u64,
        block_number: c_adv.block_number as u64,
        block_timestamp: c_adv.block_timestamp as u64,
        index: c_adv.index as u64,
    };

    // Copy 20-byte addresses
    let mut app_contract = [0u8; 20];
    let mut msg_sender = [0u8; 20];

    unsafe {
        // If bindgen produced something like `app_contract.data: [u8; 20]`, you can do:
        // app_contract.copy_from_slice(&c_adv.app_contract.data);

        app_contract.copy_from_slice(&c_adv.app_contract.data[..20]);
        msg_sender.copy_from_slice(&c_adv.msg_sender.data[..20]);
    }

    // Copy 32-byte prev_randao
    let mut prev_randao = [0u8; 32];
    unsafe {
        prev_randao.copy_from_slice(&c_adv.prev_randao.data[..32]);
    }

    // Copy payload into Vec<u8> (Rust-owned)
    let payload = unsafe {
        let len = c_adv.payload.length as usize;
        let ptr = c_adv.payload.data as *const u8;

        if len > 0 && !ptr.is_null() {
            slice::from_raw_parts(ptr, len).to_vec()
        } else {
            Vec::new()
        }
    };

    Advance {
        metadata,
        app_contract,
        msg_sender,
        prev_randao,
        payload,
    }
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

    pub fn read_advance_state(&mut self) -> io::Result<Advance> {
        let mut c_adv = MaybeUninit::<cmt_rollup_advance_t>::uninit();
        to_io_result(unsafe {
            cmt_rollup_read_advance_state(&mut self.inner, c_adv.as_mut_ptr())
        })?;
        let c_adv = unsafe { c_adv.assume_init() };
        Ok(convert_advance(c_adv))
    }

    pub fn read_inspect_state(&mut self) -> io::Result<Inspect> {
        let mut c_inspect = MaybeUninit::<cmt_rollup_inspect_t>::uninit();
        to_io_result(unsafe {
            cmt_rollup_read_inspect_state(&mut self.inner, c_inspect.as_mut_ptr())
        })?;
        let c_inspect = unsafe { c_inspect.assume_init() };
        let payload = unsafe {
            let len = c_inspect.payload.length as usize;
            let ptr = c_inspect.payload.data as *const u8;

            if len > 0 && !ptr.is_null() {
                slice::from_raw_parts(ptr, len).to_vec()
            } else {
                Vec::new()
            }
        };

        Ok(Inspect { payload })
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
