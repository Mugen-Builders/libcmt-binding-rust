use std::{ffi::OsStr, io, mem::MaybeUninit, slice, ptr};
use hex;
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
    pub app_contract: String,       
    pub msg_sender: String,
    pub prev_randao: String,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct Inspect {
    pub payload: String,
}

fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2 + 2);
    s.push_str("0x");
    for b in bytes {
        use std::fmt::Write;
        write!(s, "{:02x}", b).unwrap();
    }
    s
}

fn convert_advance(c_adv: &cmt_rollup_advance_t) -> Advance {
    let metadata = Metadata {
        chain_id: c_adv.chain_id as u64,
        block_number: c_adv.block_number as u64,
        block_timestamp: c_adv.block_timestamp as u64,
        index: c_adv.index as u64,
    };

    // Addresses (20 bytes)
    let app_contract =
        to_hex(&c_adv.app_contract.data[..20]);

    let msg_sender =
        to_hex(&c_adv.msg_sender.data[..20]);

    // prev_randao (32 bytes)
    let prev_randao =
        to_hex(&c_adv.prev_randao.data[..32]);

    // payload (variable length)
    let payload = unsafe {
        let len = c_adv.payload.length as usize;
        let ptr = c_adv.payload.data as *const u8;

        if len > 0 && !ptr.is_null() {
            let bytes = slice::from_raw_parts(ptr, len);
            to_hex(bytes)
        } else {
            "0x".to_string()
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

fn parse_address_20(s: &str) -> io::Result<cmt_abi_address_t> {
    let s2 = s.strip_prefix("0x").unwrap_or(s);
    if s2.len() != 40 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("address must be 40 hex chars (20 bytes), got {}", s2.len()),
        ));
    }
    let bytes = parse_hex_bytes(s)?;
    let mut data = [0u8; 20];
    data.copy_from_slice(&bytes);
    Ok(cmt_abi_address_t { data })
}

fn parse_u256_32(s: Option<&str>) -> io::Result<cmt_abi_u256_t> {
    let mut data = [0u8; 32];

    if let Some(s) = s {
        let raw = parse_hex_bytes(s)?;
        if raw.len() > 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "u256 hex too long (>32 bytes)",
            ));
        }
        data[32 - raw.len()..].copy_from_slice(&raw);
    }

    Ok(cmt_abi_u256_t { data })
}

fn parse_hex_bytes(s: &str) -> io::Result<Vec<u8>> {
    let mut s = s.strip_prefix("0x").unwrap_or(s).to_string();

    if s.is_empty() {
        return Ok(vec![]);
    }

    if s.len() % 2 != 0 {
        s.insert(0, '0');
    }

    hex::decode(&s).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, format!("invalid hex: {e}"))
    })
}

fn parse_hex_fixed(s: &str, n: usize) -> io::Result<Vec<u8>> {
    let v = parse_hex_bytes(s)?;
    if v.len() != n {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("expected {n} bytes, got {}", v.len()),
        ));
    }
    Ok(v)
}

impl Rollup {
    pub fn new() -> io::Result<Self> {
        let mut state = std::mem::MaybeUninit::<cmt_rollup_t>::zeroed();
        let rc = unsafe { cmt_rollup_init(state.as_mut_ptr()) };
        to_io_result(rc)?;
        let inner = unsafe { state.assume_init() };
        Ok(Self { inner })
    }

    pub fn emit_voucher(
        &mut self,
        address_hex: &str, 
        value_hex: Option<&str>,
        payload_hex: &str, 
    ) -> io::Result<u64> {
        let mut index: u64 = 0;

        let address = parse_address_20(address_hex)?;
        let value = parse_u256_32(value_hex)?;
        let payload_bytes = parse_hex_bytes(payload_hex)?;

        let c_payload = cmt_abi_bytes_t {
            data: if payload_bytes.is_empty() {
                ptr::null_mut()
            } else {
                payload_bytes.as_ptr() as *mut ::std::os::raw::c_void
            },
            length: payload_bytes.len() as usize,
        };
    
        to_io_result(unsafe {
            cmt_rollup_emit_voucher(
                &mut self.inner,
                &address as *const cmt_abi_address_t,
                &value as *const cmt_abi_u256_t,
                &c_payload as *const cmt_abi_bytes_t,
                &mut index as *mut u64,
            )
        })?;
    
        Ok(index)
    }

    pub fn emit_delegate_call_voucher(
        &mut self,
        address: &String,
        payload: &String,
    ) -> io::Result<u64> {
        let mut index = 0u64;
        let address = parse_address_20(address)?;
        let payload = cmt_abi_bytes_t {
            data: payload.as_bytes().as_ptr() as *mut ::std::os::raw::c_void,
            length: payload.len() as usize,
        };
        to_io_result(unsafe {
            cmt_rollup_emit_delegate_call_voucher(&mut self.inner, &address as *const cmt_abi_address_t, &payload as *const cmt_abi_bytes_t, &mut index as *mut u64)
        })?;
        Ok(index)
    }

    pub fn emit_notice(&mut self, payload: &String) -> io::Result<u64> {
        let payload = cmt_abi_bytes_t {
            data: payload.as_bytes().as_ptr() as *mut ::std::os::raw::c_void,
            length: payload.len() as usize,
        };
        let mut index = 0u64;
        to_io_result(unsafe { cmt_rollup_emit_notice(&mut self.inner, &payload, &mut index as *mut u64) })?;
        Ok(index)
    }

    pub fn emit_report(&mut self, payload: &String) -> io::Result<()> {
        let payload = cmt_abi_bytes_t {
            data: payload.as_bytes().as_ptr() as *mut ::std::os::raw::c_void,
            length: payload.len() as usize,
        };
        to_io_result(unsafe { cmt_rollup_emit_report(&mut self.inner, &payload) })
    }

    pub fn emit_exception(&mut self, payload: &String) -> io::Result<()> {
        let payload = cmt_abi_bytes_t {
            data: payload.as_bytes().as_ptr() as *mut ::std::os::raw::c_void,
            length: payload.len() as usize,
        };
        to_io_result(unsafe { cmt_rollup_emit_exception(&mut self.inner, &payload) })
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
        Ok(convert_advance(&c_adv))
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

        Ok(Inspect { payload: to_hex(&payload) })
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
