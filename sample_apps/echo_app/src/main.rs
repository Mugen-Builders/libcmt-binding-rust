use libcmt_binding_rust::rollup::*;
use libcmt_binding_rust::cmt_rollup_finish_t;
use hex;
use ethers_core::types::{Address, Bytes, U256};

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
fn handle_parse_erc1155_batch_deposit(
    input: String,
) -> Result<Erc1155BatchDeposit, String> {
    let bytes = hex::decode(input.trim_start_matches("0x")).unwrap();

    if bytes.len() < 20 + 20 + 32 + 32 + 32 {
        return Err(String::from(
            "Invalid payload length".to_string(),
        ));
    }

    let u256_from = |b: &[u8]| U256::from_big_endian(b);
    let _as_addr = |b: &[u8]| Address::from_slice(&b[12..32]);

    let token = hex::encode(&bytes[0..20]);
    let sender = hex::encode(&bytes[20..40]);

    let token_ids_offset = u256_from(&bytes[64..96]).as_usize();
    let values_offset = u256_from(&bytes[96..128]).as_usize();
    let base_offset = u256_from(&bytes[128..160]).as_usize();
    let exec_offset = u256_from(&bytes[160..192]).as_usize();

    let token_ids_len = u256_from(&bytes[token_ids_offset..token_ids_offset + 32]).as_usize();
    let mut token_ids = Vec::with_capacity(token_ids_len);

    let mut cursor = token_ids_offset + 32;
    for _ in 0..token_ids_len {
        token_ids.push(u256_from(&bytes[cursor..cursor + 32]));
        cursor += 32;
    }

    let values_len = u256_from(&bytes[values_offset..values_offset + 32]).as_usize();
    let mut values = Vec::with_capacity(values_len);

    let mut cursor2 = values_offset + 32;
    for _ in 0..values_len {
        values.push(u256_from(&bytes[cursor2..cursor2 + 32]));
        cursor2 += 32;
    }

    let base_len = u256_from(&bytes[base_offset..base_offset + 32]).as_usize();
    let base_start = base_offset + 32;
    let base_end = base_start + base_len;
    let base_layer_data = hex::encode(bytes[base_start..base_end].to_vec());

    let exec_len = u256_from(&bytes[exec_offset..exec_offset + 32]).as_usize();
    let exec_start = exec_offset + 32;
    let exec_end = exec_start + exec_len;
    let exec_layer_data = hex::encode(bytes[exec_start..exec_end].to_vec());

    if token_ids_len == 0 || values_len == 0 || token_ids_len != values_len {
        return Err(String::from(
            "Invalid payload data".to_string(),
        ));
    }

    Ok(Erc1155BatchDeposit {
            sender,
            token,
            count: token_ids_len,
            token_ids,
            amounts: values,
            base_layer_data,
            exec_layer_data,
        }
    )
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
            let deposit = handle_parse_erc1155_batch_deposit(payload)?;
            println!(" ERC1155BatchPortal Deposit: {:?}", deposit);
        }
        Portals::ERC1155SinglePortal => {
            let deposit = handle_parse_erc1155_single_deposit(payload)?;
            println!(" ERC1155SinglePortal Deposit: {:?}", deposit);
        }
        Portals::ERC20Portal => {
            let deposit = handle_parse_erc20_and_erc721_deposit(payload, TokenType::Erc20)?;
            println!(" ERC20Portal Deposit: {:?}", deposit);
        }
        Portals::ERC721Portal => {
            let deposit = handle_parse_erc20_and_erc721_deposit(payload, TokenType::Erc721)?;
            println!(" ERC721Portal Deposit: {:?}", deposit);
        }
        Portals::EtherPortal => {
            let deposit = handle_parse_ether_deposit(payload)?;
            println!(" EtherPortal Deposit: {:?}", deposit);
            rollup.emit_voucher(&deposit.sender, Some(&deposit.amount.to_string()), &deposit.exec_layer_data)?;
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
        
        let next_request_type = match finish.next_request_type {
            0 => "advance_state",
            1 => "inspect_state",
            _ => {
                eprintln!("Unknown request type: {}", finish.next_request_type);
                "unknown"
            }
        };
        println!("Received next input of type: {:?}", next_request_type);
        accept_previous_request = match next_request_type {
            "advance_state" => handle_advance(&mut rollup).await?,
            "inspect_state" => handle_inspect(&mut rollup).await?,
            _ => {
                eprintln!("Unknown request type");
                false
            }
        }
    }
}

