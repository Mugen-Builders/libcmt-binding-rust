use libcmt_binding_rust::rollup::*;
use libcmt_binding_rust::cmt_rollup_finish_t;
use hex;
use ethers_core::types::{Address, Bytes, U256};
use ethers_core::abi::{Token, encode};
use ethers_core::utils::{id};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Portals {
    ERC1155BatchPortal,
    ERC1155SinglePortal,
    ERC20Portal,
    ERC721Portal,
    EtherPortal,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenType {
    Erc20,
    Erc721,
}

#[derive(Debug, Clone)]
pub struct Erc1155SingleDeposit {
    pub sender: String,
    pub token: String,
    pub token_id: U256,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone)]
pub struct EtherDeposit {
    pub sender: String,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc20Deposit {
    pub sender: String,
    pub token: String,
    pub amount: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc721Deposit {
    pub sender: String,
    pub token: String,
    pub token_id: U256,
    pub exec_layer_data: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Erc20OrErc721Deposit {
    Erc20(Erc20Deposit),
    Erc721(Erc721Deposit),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Erc1155BatchDeposit {
    pub sender: String,
    pub token: String,
    pub count: usize,
    pub token_ids: Vec<U256>,
    pub amounts: Vec<U256>,
    pub base_layer_data: String,
    pub exec_layer_data: String,
}

pub fn match_portal(address: &str) -> Portals {
    if address.eq_ignore_ascii_case("0xc700A2e5531E720a2434433b6ccf4c0eA2400051") {
        Portals::ERC1155BatchPortal
    } else if address.eq_ignore_ascii_case("0xc700A261279aFC6F755A3a67D86ae43E2eBD0051") {
        Portals::ERC1155SinglePortal
    } else if address.eq_ignore_ascii_case("0xc700D6aDd016eECd59d989C028214Eaa0fCC0051") {
        Portals::ERC20Portal
    } else if address.eq_ignore_ascii_case("0xc700d52F5290e978e9CAe7D1E092935263b60051") {
        Portals::ERC721Portal
    } else if address.eq_ignore_ascii_case("0xc70076a466789B595b50959cdc261227F0D70051") {
        Portals::EtherPortal
    } else {
        Portals::None
    }
}

fn handle_parse_erc1155_single_deposit(
    input: String,
) -> Result<Erc1155SingleDeposit, String> {
    let bytes = hex::decode(input.trim_start_matches("0x")).unwrap();

    if bytes.len() < 20 + 20 + 32 + 32 {
        return Err(String::from(
            "Invalid payload length".to_string(),
        ));
    }

    let token = &bytes[0..20];
    let sender = &bytes[20..40];
    let token_id_bytes = &bytes[40..72];
    let token_id = U256::from_big_endian(token_id_bytes);
    let amount_bytes = &bytes[72..104];
    let amount = U256::from_big_endian(amount_bytes);
    let _base_layer_data = Bytes::from(bytes[104..136].to_vec());
    let exec_layer_data = Bytes::from(bytes[136..].to_vec());

    Ok(Erc1155SingleDeposit {
        sender: hex::encode(sender),
        token: hex::encode(token),
        token_id,
        amount,
        exec_layer_data: hex::encode(exec_layer_data),
    })
}

fn handle_parse_ether_deposit(input: String) -> Result<EtherDeposit, String> {
    let bytes = hex::decode(input.trim_start_matches("0x")).unwrap();

    if bytes.len() < 20 + 32 {
        return Err(String::from(
            "Invalid payload length".to_string(),
        ));
    }

    let sender_bytes = &bytes[0..20];
    let sender = hex::encode(sender_bytes);

    let value_bytes = &bytes[20..52];
    let value = U256::from_big_endian(value_bytes);

    let exec_layer_data = hex::encode(bytes[52..].to_vec());

    println!(":::: LOGS:::: EtherDeposit: {:?}", EtherDeposit {
        sender: sender.clone(),
        amount: value,
        exec_layer_data: exec_layer_data.clone(),
    });

    Ok(EtherDeposit {
        sender,
        amount: value,
        exec_layer_data,
    })
}

fn handle_parse_erc20_and_erc721_deposit(
    input: String,
    t_type: TokenType,
) -> Result<Erc20OrErc721Deposit, String> {
    let bytes = hex::decode(input.trim_start_matches("0x")).unwrap();

    if bytes.len() < 20 + 20 + 32 {
        return Err(String::from(
            "Invalid payload length".to_string(),
        ));
    }

    let token = &bytes[0..20];
    let sender = &bytes[20..40];
    let amount_bytes = &bytes[40..72];
    let amount = U256::from_big_endian(amount_bytes);
    let exec_layer_data = hex::encode(bytes[72..].to_vec());

    match t_type {
        TokenType::Erc20 => Ok(Erc20OrErc721Deposit::Erc20(Erc20Deposit {
            sender: hex::encode(sender),
            token: hex::encode(token),
            amount,
            exec_layer_data,
        })),
        TokenType::Erc721 => Ok(Erc20OrErc721Deposit::Erc721(Erc721Deposit {
            sender: hex::encode(sender),
            token: hex::encode(token),
            token_id: amount,
            exec_layer_data,
        })),
    }
}


fn u256_from_word(word: &[u8]) -> Result<U256, String> {
    if word.len() != 32 {
        return Err("expected 32-byte ABI word".to_string());
    }
    Ok(U256::from_big_endian(word))
}

fn u256_to_usize_checked(x: U256) -> Result<usize, String> {
    if x > U256::from(usize::MAX) {
        return Err("Integer overflow when casting to usize".to_string());
    }
    Ok(x.as_usize())
}

fn read_word(buf: &[u8], off: usize) -> Result<&[u8], String> {
    let end = off.checked_add(32).ok_or("offset overflow")?;
    if end > buf.len() {
        return Err("out of bounds".to_string());
    }
    Ok(&buf[off..end])
}

fn read_dyn_u256_array(buf: &[u8], base: usize) -> Result<Vec<U256>, String> {
    let len = u256_to_usize_checked(u256_from_word(read_word(buf, base)?)?)?;
    let start = base + 32;
    let end = start
        .checked_add(len.checked_mul(32).ok_or("array len overflow")?)
        .ok_or("array span overflow")?;
    if end > buf.len() {
        return Err("array out of bounds".to_string());
    }

    let mut out = Vec::with_capacity(len);
    let mut cur = start;
    for _ in 0..len {
        out.push(u256_from_word(&buf[cur..cur + 32])?);
        cur += 32;
    }
    Ok(out)
}

fn read_dyn_bytes(buf: &[u8], base: usize) -> Result<String, String> {
    let len = u256_to_usize_checked(u256_from_word(read_word(buf, base)?)?)?;
    let start = base + 32;
    let end = start.checked_add(len).ok_or("bytes span overflow")?;
    if end > buf.len() {
        return Err("bytes out of bounds".to_string());
    }
    Ok(hex::encode(&buf[start..end]))
}

fn handle_parse_erc1155_batch_deposit(input: String) -> Result<Erc1155BatchDeposit, String> {
    let bytes = hex::decode(input.trim_start_matches("0x"))
        .map_err(|e| format!("invalid hex: {e}"))?;

    if bytes.len() < 40 + 32 * 4 {
        return Err("Invalid payload length".to_string());
    }
    let token = hex::encode(&bytes[0..20]);
    let sender = hex::encode(&bytes[20..40]);
    let abi = &bytes[40..];

    let token_ids_off = u256_to_usize_checked(u256_from_word(read_word(abi, 0)?)?)?;
    let values_off    = u256_to_usize_checked(u256_from_word(read_word(abi, 32)?)?)?;
    let base_off      = u256_to_usize_checked(u256_from_word(read_word(abi, 64)?)?)?;
    let exec_off      = u256_to_usize_checked(u256_from_word(read_word(abi, 96)?)?)?;

    for (name, off) in [
        ("tokenIds", token_ids_off),
        ("values", values_off),
        ("base", base_off),
        ("exec", exec_off),
    ] {
        if off % 32 != 0 {
            return Err(format!("{name} offset not 32-byte aligned"));
        }
        if off + 32 > abi.len() {
            return Err(format!("{name} offset out of bounds"));
        }
    }

    let token_ids = read_dyn_u256_array(abi, token_ids_off)?;
    let values    = read_dyn_u256_array(abi, values_off)?;
    let base_layer_data = read_dyn_bytes(abi, base_off)?;
    let exec_layer_data = read_dyn_bytes(abi, exec_off)?;

    if token_ids.is_empty() || values.is_empty() || token_ids.len() != values.len() {
        return Err("Invalid payload data".to_string());
    }

    Ok(Erc1155BatchDeposit {
        sender,
        token,
        count: token_ids.len(),
        token_ids,
        amounts: values,
        base_layer_data,
        exec_layer_data,
    })
}

pub fn build_and_emit_erc20_voucher(rollup: &mut Rollup, deposit: Erc20Deposit) -> Result<bool, Box<dyn std::error::Error>> {
    let sender_hex = deposit.sender.strip_prefix("0x").unwrap_or(&deposit.sender);
    let sender_bytes = hex::decode(sender_hex)?;
    if sender_bytes.len() != 20 {
        return Err(format!("invalid sender address length: {}", sender_bytes.len()).into());
    }
    let sender = Address::from_slice(&sender_bytes);
    let args: Vec<Token> = vec![
        Token::Address(sender),
        Token::Uint(deposit.amount.into()),
    ];
    let function_sig = "transfer(address,uint256)";
    let selector = &id(function_sig)[..4];
    let encoded_args = encode(&args);
    let mut payload_bytes = Vec::new();
    payload_bytes.extend_from_slice(selector);
    payload_bytes.extend_from_slice(&encoded_args);
    let payload = format!("0x{}", hex::encode(payload_bytes));
    rollup.emit_voucher(&deposit.token, None, &payload)?;
    println!("Emitted ERC20 voucher");
    Ok(true)
}

pub fn build_and_emit_erc721_voucher(rollup: &mut Rollup, deposit: Erc721Deposit, app_contract: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let token = deposit.token;
    let app_s = if app_contract.starts_with("0x") {
        app_contract.to_string()
    } else {
        format!("0x{}", app_contract)
    };
    let app_addr = Address::from_str(&app_s)?;
    let sender_s = if deposit.sender.starts_with("0x") {
        deposit.sender.clone()
    } else {
        format!("0x{}", deposit.sender)
    };
    let sender_addr = Address::from_str(&sender_s)?;
    let args: Vec<Token> = vec![
        Token::Address(app_addr),
        Token::Address(sender_addr),
        Token::Uint(deposit.token_id.into()),
    ];
    let function_sig = "transferFrom(address,address,uint256)";
    let selector = &id(function_sig)[..4];
    let encoded_args = encode(&args);
    let mut payload_bytes = Vec::new();
    payload_bytes.extend_from_slice(selector);
    payload_bytes.extend_from_slice(&encoded_args);
    let payload = format!("0x{}", hex::encode(payload_bytes));
    rollup.emit_voucher(&token, None, &payload)?;
    println!("Emitted ERC721 voucher");
    Ok(true)
}

pub fn build_and_emit_erc1155_single_voucher(rollup: &mut Rollup, deposit: Erc1155SingleDeposit, app_contract: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let app_s = if app_contract.starts_with("0x") {
        app_contract.to_string()
    } else {
        format!("0x{}", app_contract)
    };
    let app_addr = Address::from_str(&app_s)?;
    let sender_s = if deposit.sender.starts_with("0x") {
        deposit.sender.clone()
    } else {
        format!("0x{}", deposit.sender)
    };
    let sender_addr = Address::from_str(&sender_s)?;

    let args: Vec<Token> = vec![
        Token::Address(app_addr),
        Token::Address(sender_addr),
        Token::Uint(deposit.token_id.into()),
        Token::Uint(deposit.amount.into()),
        Token::Bytes(deposit.exec_layer_data.as_bytes().to_vec()),
    ];
    let function_sig = "safeTransferFrom(address from, address to, uint256 id, uint256 value, bytes calldata data)";
    let selector = &id(function_sig)[..4];
    let encoded_args = encode(&args);
    let mut payload_bytes = Vec::new();
    payload_bytes.extend_from_slice(selector);
    payload_bytes.extend_from_slice(&encoded_args);
    let payload = format!("0x{}", hex::encode(payload_bytes));

    rollup.emit_voucher(&deposit.token, None, &payload)?;
    println!("Emitted ERC1155Single voucher");
    Ok(true)
}

pub fn build_and_emit_erc1155_batch_voucher(rollup: &mut Rollup, deposit: Erc1155BatchDeposit, app_contract: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let app_s = if app_contract.starts_with("0x") {
        app_contract.to_string()
    } else {
        format!("0x{}", app_contract)
    };
    let app_addr = Address::from_str(&app_s)?;
    let sender_s = if deposit.sender.starts_with("0x") {
        deposit.sender.clone()
    } else {
        format!("0x{}", deposit.sender)
    };
    let sender_addr = Address::from_str(&sender_s)?;

    let args: Vec<Token> = vec![
        Token::Address(app_addr),
        Token::Address(sender_addr),
        Token::Array(deposit.token_ids.iter().cloned().map(|id| Token::Uint(id.into())).collect()),
        Token::Array(deposit.amounts.iter().cloned().map(|amount| Token::Uint(amount.into())).collect()),
        Token::Bytes(deposit.exec_layer_data.as_bytes().to_vec()),
    ];
    let function_sig = "safeBatchTransferFrom(address from, address to, uint256[] calldata ids, uint256[] calldata values, bytes calldata data)";
    let selector = &id(function_sig)[..4];
    let encoded_args = encode(&args);
    let mut payload_bytes = Vec::new();
    payload_bytes.extend_from_slice(selector);
    payload_bytes.extend_from_slice(&encoded_args);
    let payload = format!("0x{}", hex::encode(payload_bytes));
    rollup.emit_voucher(&deposit.token, None, &payload)?;
    println!("Emitted ERC1155Batch voucher");
    Ok(true)
}


pub async fn handle_advance(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let advance = rollup.read_advance_state()?;
    println!("Received advance request data {:?}", &advance);
    let payload = advance.payload;
    let msg_sender = advance.msg_sender;

    println!("Payload: {}", payload);
    println!("Msg sender: {}", msg_sender);

    match match_portal(&msg_sender) {
        Portals::ERC1155BatchPortal => {
            let deposit: Erc1155BatchDeposit = handle_parse_erc1155_batch_deposit(payload)?;
            println!(" ERC1155BatchPortal Deposit: {:?}", deposit);
            build_and_emit_erc1155_batch_voucher(rollup, deposit, &advance.app_contract)?;
        }
        Portals::ERC1155SinglePortal => {
            let deposit = handle_parse_erc1155_single_deposit(payload)?;
            println!(" ERC1155SinglePortal Deposit: {:?}", deposit);
            build_and_emit_erc1155_single_voucher(rollup, deposit, &advance.app_contract)?;
        }
        Portals::ERC20Portal => {
            let deposit: Erc20OrErc721Deposit = handle_parse_erc20_and_erc721_deposit(payload, TokenType::Erc20)?;
            println!(" ERC20Portal Deposit: {:?}", deposit);
            match deposit {
                Erc20OrErc721Deposit::Erc20(deposit) => {
                    build_and_emit_erc20_voucher(rollup, deposit)?;
                }
                _ => {}
            }
        }
        Portals::ERC721Portal => {
            let deposit: Erc20OrErc721Deposit = handle_parse_erc20_and_erc721_deposit(payload, TokenType::Erc721)?;
            println!(" ERC721Portal Deposit: {:?}", deposit);
            match deposit {
                Erc20OrErc721Deposit::Erc721(deposit) => {
                    build_and_emit_erc721_voucher(rollup, deposit, &advance.app_contract)?;
                }
                _ => {}
            }
        }
        Portals::EtherPortal => {
            let deposit = handle_parse_ether_deposit(payload)?;
            println!(" EtherPortal Deposit: {:?}", deposit);
            let amount_hex = format!("0x{:x}", deposit.amount);
            rollup.emit_voucher(&deposit.sender, Some(&amount_hex), &deposit.exec_layer_data)?;
            println!(":::: LOGS:::: Emitted voucher: {:?}", &amount_hex);
            println!("Emitted voucher");
        }
        Portals::None => {
            eprintln!("Unknown portal. User Input detected from: {}", msg_sender);
        }
    }
    Ok(true)
}

pub async fn handle_inspect(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let inspect = rollup.read_inspect_state()?;
    println!("Received inspect request data {:?}", &inspect);
    let payload = inspect.payload;
    println!("Received, but ignoring inspect request. Payload: {}", payload);
    Ok(true)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut accept_previous_request = true;
    let mut rollup: Rollup = Rollup::new().expect("Failed to create Rollup instance");
    
    loop {
        println!("Sending finish");
        let mut finish = cmt_rollup_finish_t {
            accept_previous_request,
            next_request_type: 0,
            next_request_payload_length: 0,
        };
        rollup.finish(&mut finish)?;
        
        accept_previous_request = match finish.next_request_type {
            0 => {
                println!("Received next input of type: advance_state");
                handle_advance(&mut rollup).await?
            },
            1 => {
                println!("Received next input of type: inspect_state");
                handle_inspect(&mut rollup).await?
            },
            _ =>  {
                eprintln!("Unknown request type: {}", finish.next_request_type);
                false
            }
        };
    }
}
