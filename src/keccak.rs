//! Keccak-256 helpers exposed by `libcmt`.

use std::ffi::CString;

use crate::cmt_keccak_t;
use crate::generated::{
    CMT_KECCAK_LENGTH, cmt_keccak_data, cmt_keccak_final, cmt_keccak_funsel, cmt_keccak_init,
    cmt_keccak_update,
};

const KECCAK_LEN: usize = CMT_KECCAK_LENGTH as usize;

pub fn init(state: &mut cmt_keccak_t) {
    unsafe { cmt_keccak_init(state) }
}

pub fn update(state: &mut cmt_keccak_t, data: &[u8]) {
    unsafe { cmt_keccak_update(state, data.len(), data.as_ptr() as *const _) }
}

pub fn finalize(state: &mut cmt_keccak_t, out: &mut [u8; KECCAK_LEN]) {
    unsafe { cmt_keccak_final(state, out.as_mut_ptr() as *mut _) }
}

pub fn data(length: usize, input: &[u8], out: &mut [u8; KECCAK_LEN]) -> *mut u8 {
    unsafe { cmt_keccak_data(length, input.as_ptr() as *const _, out.as_mut_ptr()) }
}

pub fn funsel(decl: &str) -> u32 {
    let c_decl = CString::new(decl).expect("ABI declaration must be null free");
    unsafe { cmt_keccak_funsel(c_decl.as_ptr()) }
}
