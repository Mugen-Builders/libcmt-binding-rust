use libcmt_binding_rust::rollup::*;
use libcmt_binding_rust::cmt_rollup_finish_t;


pub async fn handle_advance(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let advance = rollup.read_advance_state()?;
    println!("Received advance request data {:?}", &advance);
    // let _payload = advance["data"]["payload"]
    //     .as_str()
    //     .ok_or("Missing payload")?;
    // TODO: add application logic here
    Ok(true)
}

pub async fn handle_inspect(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let inspect = rollup.read_inspect_state()?;
    println!("Received inspect request data {:?}", &inspect);
    // let _payload = inspect["data"]["payload"]
    //     .as_str()
    //     .ok_or("Missing payload")?;
    // TODO: add application logic here
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


    // loop {
    //     println!("Sending finish");
    //     let response = object! {"status" => status};
    //     let request = hyper::Request::builder()
    //         .method(hyper::Method::POST)
    //         .header(hyper::header::CONTENT_TYPE, "application/json")
    //         .uri(format!("{}/finish", &server_addr))
    //         .body(hyper::Body::from(response.dump()))?;
    //     let response = client.request(request).await?;
    //     println!("Received finish status {}", response.status());

    //     if response.status() == hyper::StatusCode::ACCEPTED {
    //         println!("No pending rollup request, trying again");
    //     } else {
    //         let body = hyper::body::to_bytes(response).await?;
    //         let utf = std::str::from_utf8(&body)?;
    //         let req = json::parse(utf)?;

    //         let request_type = req["request_type"]
    //             .as_str()
    //             .ok_or("request_type is not a string")?;
    //         status = match request_type {
    //             "advance_state" => handle_advance(&client, &server_addr[..], req).await?,
    //             "inspect_state" => handle_inspect(&client, &server_addr[..], req).await?,
    //             &_ => {
    //                 eprintln!("Unknown request type");
    //                 "reject"
    //             }
    //         };
    //     }
    // }


    //     rollup = Rollup()
    // accept_previous_request = True

    // # Main loop
    // while True:
    //     logger.info("[app] Sending finish")

    //     next_request_type = rollup.finish(accept_previous_request)

    //     logger.info(f"[app] Received input of type {next_request_type}")

    //     accept_previous_request = handlers[next_request_type](rollup)