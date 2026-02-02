//! Safe wrappers around the ABI helpers from `libcmt`.

use std::{ffi::c_void, io};

use crate::generated::{
    CMT_ABI_U256_LENGTH, cmt_abi_check_funsel, cmt_abi_decode_uint, cmt_abi_decode_uint_nn,
    cmt_abi_decode_uint_nr, cmt_abi_encode_uint, cmt_abi_encode_uint_nn, cmt_abi_encode_uint_nr,
    cmt_abi_funsel, cmt_abi_get_address, cmt_abi_get_bool, cmt_abi_get_bytes_d,
    cmt_abi_get_bytes_s, cmt_abi_get_uint, cmt_abi_get_uint_be, cmt_abi_get_uint256,
    cmt_abi_mark_frame, cmt_abi_peek_bytes_d, cmt_abi_peek_funsel, cmt_abi_put_address,
    cmt_abi_put_bool, cmt_abi_put_bytes_d, cmt_abi_put_bytes_s, cmt_abi_put_funsel,
    cmt_abi_put_uint, cmt_abi_put_uint_be, cmt_abi_put_uint256, cmt_abi_reserve_bytes_d,
    cmt_abi_start_frame,
};
use crate::{cmt_abi_address_t, cmt_abi_bytes_t, cmt_abi_u256_t, cmt_buf_t, to_io_result};

const ABI_U256_LEN: usize = CMT_ABI_U256_LENGTH as usize;

pub fn funsel(a: u8, b: u8, c: u8, d: u8) -> u32 {
    unsafe { cmt_abi_funsel(a, b, c, d) }
}

pub fn mark_frame(me: &mut cmt_buf_t, frame: &mut cmt_buf_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_mark_frame(me, frame) })
}

pub fn put_funsel(me: &mut cmt_buf_t, value: u32) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_funsel(me, value) })
}

pub fn put_uint(me: &mut cmt_buf_t, data_length: usize, data: *const c_void) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_uint(me, data_length, data) })
}

pub fn put_uint_be(me: &mut cmt_buf_t, length: usize, data: *const c_void) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_uint_be(me, length, data) })
}

pub fn put_uint256(me: &mut cmt_buf_t, value: &cmt_abi_u256_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_uint256(me, value) })
}

pub fn put_bool(me: &mut cmt_buf_t, value: bool) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_bool(me, value) })
}

pub fn put_address(me: &mut cmt_buf_t, address: &cmt_abi_address_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_address(me, address) })
}

pub fn put_bytes_s(me: &mut cmt_buf_t, offset: &mut cmt_buf_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_bytes_s(me, offset) })
}

pub fn put_bytes_d(
    me: &mut cmt_buf_t,
    offset: &mut cmt_buf_t,
    frame: &cmt_buf_t,
    payload: &cmt_abi_bytes_t,
) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_put_bytes_d(me, offset, frame, payload) })
}

pub fn reserve_bytes_d(
    me: &mut cmt_buf_t,
    of: &mut cmt_buf_t,
    n: usize,
    out: &mut cmt_buf_t,
    start: *const c_void,
) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_reserve_bytes_d(me, of, n, out, start) })
}

pub fn peek_funsel(me: &mut cmt_buf_t) -> u32 {
    unsafe { cmt_abi_peek_funsel(me) }
}

pub fn check_funsel(me: &mut cmt_buf_t, expected: u32) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_check_funsel(me, expected) })
}

pub fn get_uint256(me: &mut cmt_buf_t, value: &mut cmt_abi_u256_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_get_uint256(me, value) })
}

pub fn get_uint(me: &mut cmt_buf_t, n: usize, data: *mut c_void) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_get_uint(me, n, data) })
}

pub fn get_uint_be(me: &mut cmt_buf_t, n: usize, data: *mut c_void) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_get_uint_be(me, n, data) })
}

pub fn get_bool(me: &mut cmt_buf_t, value: &mut bool) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_get_bool(me, value) })
}

pub fn get_address(me: &mut cmt_buf_t, value: &mut cmt_abi_address_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_get_address(me, value) })
}

pub fn start_frame(me: &mut cmt_buf_t, frame: *mut c_void) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_start_frame(me, frame) })
}

pub fn get_bytes_s(me: &mut cmt_buf_t, of: &mut cmt_buf_t) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_get_bytes_s(me, of) })
}

pub fn get_bytes_d(
    start: &cmt_buf_t,
    of: &mut cmt_buf_t,
    n: &mut usize,
    data: &mut *mut c_void,
) -> io::Result<()> {
    to_io_result(unsafe {
        cmt_abi_get_bytes_d(start, of, n as *mut usize, data as *mut *mut c_void)
    })
}

pub fn peek_bytes_d(
    start: &cmt_buf_t,
    of: &mut cmt_buf_t,
    bytes: &mut cmt_buf_t,
) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_peek_bytes_d(start, of, bytes) })
}

pub fn encode_uint(n: usize, data: *const c_void, out: &mut [u8; ABI_U256_LEN]) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_encode_uint(n, data, out.as_mut_ptr()) })
}

pub fn encode_uint_nr(n: usize, data: *const u8, out: &mut [u8; ABI_U256_LEN]) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_encode_uint_nr(n, data, out.as_mut_ptr()) })
}

pub fn encode_uint_nn(n: usize, data: *const u8, out: &mut [u8; ABI_U256_LEN]) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_encode_uint_nn(n, data, out.as_mut_ptr()) })
}

pub fn decode_uint(data: &[u8; ABI_U256_LEN], n: usize, out: *mut u8) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_decode_uint(data.as_ptr(), n, out) })
}

pub fn decode_uint_nr(data: &[u8; ABI_U256_LEN], n: usize, out: *mut u8) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_decode_uint_nr(data.as_ptr(), n, out) })
}

pub fn decode_uint_nn(data: &[u8; ABI_U256_LEN], n: usize, out: *mut u8) -> io::Result<()> {
    to_io_result(unsafe { cmt_abi_decode_uint_nn(data.as_ptr(), n, out) })
}
