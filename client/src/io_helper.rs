use crate::globals::*;
use std::io::{Write, stdin, stdout};
use tokio::net::UdpSocket;

pub async fn handle_msg(
    socket: &UdpSocket,
    target: &str,
    seq: &mut u32,
    buf: &mut [u8],
) -> Result<(), std::io::Error> {
    let msg = wait_user_input();
    println!("Seq: {}", seq);
    println!("Message: {}", msg);

    let packet: Vec<u8> = generate_msg(seq, 0, msg.as_bytes());
    println!("Gen Packet bytes: {:?}", packet);

    socket.send_to(&packet, target).await?;

    let (total_len, _) = socket.recv_from(buf).await?;
    println!("Received {} bytes from server {}", total_len, target);

    match verify_ack(buf, seq) {
        Ok(ack) => {
            *seq += 1;
            process_ack(&ack);
        }
        Err(_e) => {
            eprintln!("Error: {}", _e);
        }
    }

    Ok(())
}

fn verify_ack(buf: &mut [u8], expected_ack: &mut u32) -> Result<u32, String> {
    // get ack
    let ack_bytes: [u8; 4] = buf[ACK_START_INDEX..MSG_START_INDEX].try_into().unwrap();
    let ack = u32::from_be_bytes(ack_bytes);

    println!("Expected Ack: {}", expected_ack);
    if *expected_ack != ack {
        return Err(format!(
            "ack {} does not match expected ack {}",
            ack, expected_ack
        ));
    }

    Ok(ack)
}

fn generate_msg(seq: &u32, ack: u32, msg: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.extend_from_slice(&ack.to_be_bytes());
    buf.extend_from_slice(msg);
    buf
}

fn wait_user_input() -> String {
    let mut msg = String::new();
    print!("Please enter some text: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut msg)
        .expect("Did not enter a correct string");
    if let Some('\n') = msg.chars().next_back() {
        msg.pop();
    }
    if let Some('\r') = msg.chars().next_back() {
        msg.pop();
    }
    msg
}

fn process_ack(ack: &u32) {
    println!("ACK received: {}", ack);
}
