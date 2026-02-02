//! Sparse Merkle helpers exposed by `libcmt`.

use std::{
    ffi::{CStr, c_void},
    io,
};

use crate::generated::{
    CMT_KECCAK_LENGTH, cmt_merkle_fini, cmt_merkle_get_leaf_count, cmt_merkle_get_root_hash,
    cmt_merkle_init, cmt_merkle_load, cmt_merkle_push_back, cmt_merkle_push_back_data,
    cmt_merkle_reset, cmt_merkle_save, cmt_merkle_t,
};
use crate::to_io_result;

const KECCAK_LEN: usize = CMT_KECCAK_LENGTH as usize;

pub fn init(me: &mut cmt_merkle_t) {
    unsafe { cmt_merkle_init(me) }
}

pub fn reset(me: &mut cmt_merkle_t) {
    unsafe { cmt_merkle_reset(me) }
}

pub fn fini(me: &mut cmt_merkle_t) {
    unsafe { cmt_merkle_fini(me) }
}

pub fn load(me: &mut cmt_merkle_t, path: &CStr) -> io::Result<()> {
    to_io_result(unsafe { cmt_merkle_load(me, path.as_ptr()) })
}

pub fn save(me: &mut cmt_merkle_t, path: &CStr) -> io::Result<()> {
    to_io_result(unsafe { cmt_merkle_save(me, path.as_ptr()) })
}

pub fn leaf_count(me: &mut cmt_merkle_t) -> u64 {
    unsafe { cmt_merkle_get_leaf_count(me) }
}

pub fn push_back(me: &mut cmt_merkle_t, hash: &[u8; KECCAK_LEN]) -> io::Result<()> {
    to_io_result(unsafe { cmt_merkle_push_back(me, hash.as_ptr()) })
}

pub fn push_back_data(me: &mut cmt_merkle_t, data: &[u8]) -> io::Result<()> {
    to_io_result(unsafe {
        cmt_merkle_push_back_data(me, data.len(), data.as_ptr() as *const c_void)
    })
}

pub fn root_hash(me: &mut cmt_merkle_t, out: &mut [u8; KECCAK_LEN]) {
    unsafe { cmt_merkle_get_root_hash(me, out.as_mut_ptr()) }
}
