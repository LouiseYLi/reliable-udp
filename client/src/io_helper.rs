// use crate::globals::*;
use std::io::{Write, stdin, stdout};
use tokio::net::UdpSocket;

pub async fn handle_user_input(
    socket: &UdpSocket,
    target: &str,
    seq: &u32,
) -> Result<(), std::io::Error> {
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
    println!("Seq: {}", seq);
    println!("Message: {}", msg);

    let packet: Vec<u8> = generate_msg(seq, msg.as_bytes());
    println!("Packet bytes: {:?}", packet);

    socket.send_to(&packet, target).await?;
    // let bytes_sent = socket.send_to(&packet, target).await?;

    Ok(())
    // socket.send_to(buf[..len], &addr).await?;
}

fn generate_msg(seq: &u32, msg: &[u8]) -> Vec<u8> {
    let ack: u32 = 0;
    let mut buf = Vec::new();
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.extend_from_slice(&ack.to_be_bytes());
    buf.extend_from_slice(msg);
    buf
}
