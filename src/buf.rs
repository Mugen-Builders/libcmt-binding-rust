//! Buffer helpers from `libcmt`.

use std::io;

use crate::generated::{
    cmt_buf_init, cmt_buf_length, cmt_buf_split, cmt_buf_split_by_comma, cmt_buf_xxd,
};
use crate::{cmt_buf_t, to_io_result};

pub fn init(buf: &mut cmt_buf_t, length: usize, data: *mut u8) {
    unsafe { cmt_buf_init(buf, length, data as *mut _) }
}

pub fn split(
    me: &cmt_buf_t,
    lhs_length: usize,
    lhs: &mut cmt_buf_t,
    rhs: &mut cmt_buf_t,
) -> io::Result<()> {
    to_io_result(unsafe { cmt_buf_split(me, lhs_length, lhs, rhs) })
}

pub fn length(me: &cmt_buf_t) -> usize {
    unsafe { cmt_buf_length(me) }
}

pub fn split_by_comma(x: &mut cmt_buf_t, xs: &mut cmt_buf_t) -> bool {
    unsafe { cmt_buf_split_by_comma(x, xs) }
}

pub fn xxd(begin: *const u8, end: *const u8, bytes_per_line: i32) {
    unsafe { cmt_buf_xxd(begin as *mut _, end as *mut _, bytes_per_line) }
}
