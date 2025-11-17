use crate::globals::*;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub async fn handle_msg(
    socket: &UdpSocket,
    expected_seq: &mut u32,
    buf: &mut [u8],
    _current_target: &mut SocketAddr,
) -> Result<(), std::io::Error> {
    let (total_len, target) = socket.recv_from(buf).await?;
    println!("Received {} bytes from client {}", total_len, target);

    // TODO: remove current_target if unused
    // if target != *current_target {
    //     *expected_seq = 0;
    //     *current_target = target;
    // }

    match verify_msg(buf, expected_seq) {
        Ok(seq) => {
            process_msg(buf, total_len);

            let packet: Vec<u8> = generate_ack(&0, seq, &[]);
            println!("Gen Packet bytes: {:?}", packet);

            socket.send_to(&packet, target).await?;
        }
        Err(_e) => {
            eprintln!("Error: {}", _e);
        }
    }

    Ok(())
    // socket.send_to(buf[..len], &addr).await?;
}

fn convert_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn generate_ack(seq: &u32, ack: u32, msg: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.extend_from_slice(&ack.to_be_bytes());
    buf.extend_from_slice(msg);
    buf
}

fn verify_msg(buf: &mut [u8], expected_seq: &mut u32) -> Result<u32, String> {
    // get first 4B (sequence number)
    let seq_bytes: [u8; 4] = buf[0..ACK_START_INDEX].try_into().unwrap();
    let seq = u32::from_be_bytes(seq_bytes);

    if seq == 0 {
        *expected_seq = 0;
    }

    println!("Expected Seq: {}", expected_seq);
    println!("Seq: {}", seq);
    if seq > *expected_seq {
        Err(format!(
            "expected seq {} does not match seq {}",
            expected_seq, seq
        ))
    } else if seq < *expected_seq {
        Ok(*expected_seq - 1)
    } else {
        *expected_seq += 1;
        Ok(seq)
    }
}

fn process_msg(buf: &mut [u8], total_len: usize) {
    // get payload
    let msg = convert_to_string(&buf[MSG_START_INDEX..(total_len)]);
    println!("Message: {}", msg);
}
