use crate::File;
use crate::globals::*;
use std::io::Write;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::UdpSocket;

pub async fn handle_msg(
    socket: &UdpSocket,
    expected_seq: &mut u32,
    buf: &mut [u8],
    current_target: &mut SocketAddr,
    log: Arc<Mutex<File>>,
) -> Result<(), std::io::Error> {
    let receive_str = "[RECEIVE]\n".as_bytes();
    let send_str = "[SEND]\n".as_bytes();
    let ignore_str = "[IGNORE]\n".as_bytes();

    buf.fill(0);
    let (_total_len, target) = socket.recv_from(buf).await?;
    println!("[RECEIVE] Received from {}...", target);

    log_write(Arc::clone(&log), receive_str).await?;

    let src_target = match parse_src_target(&buf[SRC_TARGET_START_INDEX..MSG_LEN_START_INDEX]) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("\tFailed to parse source target: {}", e);
            return Ok(()); // skip this packet
        }
    };

    if src_target != *current_target {
        *expected_seq = 0;
        *current_target = src_target;
    }

    match verify_msg(buf, expected_seq) {
        Ok((seq, do_print)) => {
            if do_print {
                process_msg(buf);
            }

            let packet: Vec<u8> = generate_ack(&0, seq, &[]);

            println!("[SEND] Sending to {}...", target);
            socket.send_to(&packet, target).await?;

            log_write(Arc::clone(&log), send_str).await?;
        }
        Err(_e) => {
            println!(
                "[IGNORE] Out of order or old packet ignored from {}...",
                target
            );
            log_write(Arc::clone(&log), ignore_str).await?;
        }
    }

    Ok(())
}

fn generate_ack(seq: &u32, ack: u32, msg: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.extend_from_slice(&ack.to_be_bytes());
    buf.resize(buf.len() + 18, 0);
    buf.extend_from_slice(msg);
    buf
}

fn verify_msg(buf: &mut [u8], expected_seq: &mut u32) -> Result<(u32, bool), String> {
    // get first 4B (sequence number)
    let seq_bytes: [u8; 4] = buf[0..ACK_START_INDEX].try_into().unwrap();
    let seq = u32::from_be_bytes(seq_bytes);

    if seq == *expected_seq {
        println!("\t[VALID] Valid SEQ received: {}", seq);
        *expected_seq += 1;
        Ok((seq, true))
    } else if *expected_seq > 0 && seq == *expected_seq - 1 {
        println!("\t[INVALID] Duplicate SEQ received: {}", seq);
        Ok((seq, false))
    } else {
        Err(format!(
            "Out of order packet. expected seq {}, actual seq {}",
            expected_seq, seq
        ))
    }
}

fn process_msg(buf: &mut [u8]) {
    // get payload
    let msg_len = u32::from_be_bytes(buf[26..30].try_into().unwrap()) as usize;

    let msg_bytes = &buf[30..30 + msg_len];

    let msg = String::from_utf8_lossy(msg_bytes);

    println!("\tMessage: {}", msg);
}

pub fn parse_src_target(src_bytes: &[u8]) -> Result<SocketAddr, String> {
    if src_bytes.len() != 18 {
        return Err(format!(
            "Expected 18 bytes for src_target, got {} bytes",
            src_bytes.len()
        ));
    }
    let mut ip_bytes = [0u8; 16];
    ip_bytes.copy_from_slice(&src_bytes[0..16]);

    let ipv6 = Ipv6Addr::from(ip_bytes);

    let port = u16::from_be_bytes([src_bytes[16], src_bytes[17]]);

    let ip = IpAddr::V6(ipv6);

    Ok(SocketAddr::new(ip, port))
}

pub async fn log_write(log: Arc<Mutex<File>>, data: &[u8]) -> std::io::Result<()> {
    let data_vec = data.to_vec();
    let log_clone = Arc::clone(&log);

    tokio::task::spawn_blocking(move || -> std::io::Result<()> {
        let mut file = log_clone.lock().unwrap();
        file.write_all(&data_vec)?;
        file.flush()?;
        Ok(())
    })
    .await
    .unwrap()
}

pub fn clear_log(log: &Arc<Mutex<File>>) -> std::io::Result<()> {
    let mut file = log.lock().unwrap(); // lock the mutex
    file.set_len(0)?; // truncate file to zero
    file.flush()?; // ensure changes are written immediately
    Ok(())
}
