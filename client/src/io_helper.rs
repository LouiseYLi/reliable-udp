use crate::globals::*;
use std::io::{Write, stdin, stdout};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

pub async fn handle_msg(
    socket: &UdpSocket,
    target: &str,
    seq: &mut u32,
    buf: &mut [u8],
    timeout_secs: &u64,
    max_retries: &u16,
) -> Result<(), std::io::Error> {
    let msg = wait_user_input();
    println!("Seq: {}", seq);
    println!("Message: {}", msg);

    let packet: Vec<u8> = generate_msg(seq, 0, msg.as_bytes());
    println!("Gen Packet bytes: {:?}", packet);

    let mut retry: u16 = 0;
    while retry < *max_retries {
        socket.send_to(&packet, target).await?;

        let ack_result = timeout(Duration::from_secs(*timeout_secs), socket.recv_from(buf)).await;
        match ack_result {
            Ok(Ok((_total_len, _target))) => match verify_ack(buf, seq) {
                Ok(ack) => {
                    *seq += 1;
                    process_ack(&ack);
                    break;
                }
                Err(_e) => {
                    eprintln!("verify_ack error: {}", _e);
                }
            },
            Ok(Err(_e)) => {
                eprintln!("recv_from error: {}", _e);
            }
            Err(_e) => {
                println!("Timeout expired, retransmit... {}", _e)
            }
        }

        retry += 1;
    }
    if retry >= *max_retries {
        println!("Max retries exceeded.")
    }
    Ok(())
}

fn verify_ack(buf: &mut [u8], expected_ack: &mut u32) -> Result<u32, String> {
    // get ack
    let ack_bytes: [u8; 4] = buf[ACK_START_INDEX..MSG_START_INDEX].try_into().unwrap();
    let ack = u32::from_be_bytes(ack_bytes);

    println!("Expected Ack: {}", expected_ack);
    if *expected_ack != ack {
        Err(format!(
            "ack {} does not match expected ack {}",
            ack, expected_ack
        ))
    } else {
        Ok(ack)
    }
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
