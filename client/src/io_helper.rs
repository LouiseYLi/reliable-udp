use crate::File;
use crate::globals::*;
use std::io::{Write, stdin, stdout};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
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
    log: Arc<Mutex<File>>,
) -> Result<(), std::io::Error> {
    let send_str = "[SEND]\n".as_bytes();
    let receive_str = "[RECEIVE]\n".as_bytes();
    let ignore_str = "[IGNORE]\n".as_bytes();

    let msg = wait_user_input();
    // println!("\tSeq: {}", seq);
    // println!("\tMessage: {}", msg);

    let packet: Vec<u8> = generate_msg(seq, 0, &socket.local_addr()?, msg.as_bytes());
    // println!("\tGen Packet bytes: {:?}", packet);

    let mut retry: u16 = 0;
    while retry < *max_retries {
        socket.send_to(&packet, target).await?;
        println!("[SEND] Sending to {}...", target);
        // println!("outerloop");
        // log.write_all(send_str)?;
        log_write(Arc::clone(&log), send_str).await?;

        buf.fill(0);

        loop {
            let ack_result =
                timeout(Duration::from_secs(*timeout_secs), socket.recv_from(buf)).await;

            match ack_result {
                Ok(Ok((_total_len, _target))) => {
                    let ack_slice = &buf[.._total_len];

                    match verify_ack(ack_slice, seq) {
                        Ok(ack) => {
                            // println!("[OK CLIENT] Received from {}...", _target);
                            // log.write_all(receive_str)?;
                            log_write(Arc::clone(&log), receive_str).await?;

                            // println!("\tRead {} bytes...", _total_len);

                            *seq += 1;
                            process_ack(&ack);
                            break;
                        }
                        Err(_e) => {
                            println!("[IGNORE] Ignored {}...", _e);
                            // log.write_all(ignore_str)?;
                            log_write(Arc::clone(&log), ignore_str).await?;

                            // eprintln!("\tverify_ack error: {}", _e);
                            continue;
                        }
                    }
                }
                Ok(Err(_e)) => {
                    eprintln!("\trecv_from error: {}", _e);
                    retry += 1;
                }
                Err(_e) => {
                    println!("\tTimeout expired, retransmit... {}", _e);
                    retry += 1;
                    break;
                }
            }
        }
    }
    if retry >= *max_retries {
        *seq += 1;
        println!("\tMax retries exceeded.")
    }
    Ok(())
}

fn verify_ack(buf: &[u8], expected_ack: &mut u32) -> Result<u32, String> {
    // get ack
    let ack_bytes: [u8; 4] = buf[ACK_START_INDEX..SRC_TARGET_START_INDEX]
        .try_into()
        .unwrap();
    let ack = u32::from_be_bytes(ack_bytes);

    if *expected_ack != ack {
        Err(format!(
            "duplicate ack. expected ack {}, actual ack {}",
            expected_ack, ack
        ))
    } else {
        Ok(ack)
    }
}

fn generate_msg(seq: &u32, ack: u32, src_target: &SocketAddr, msg: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&seq.to_be_bytes());
    buf.extend_from_slice(&ack.to_be_bytes());
    buf.extend_from_slice(&encode_socket_addr(src_target));

    let msg_len = msg.len() as u32;
    buf.extend_from_slice(&msg_len.to_be_bytes());

    buf.extend_from_slice(msg);
    // println!("packet.len = {}", buf.len());
    // println!("{:?}", buf);
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
    println!("\t[RECEIVE] Valid ACK received: {}", ack);
}

fn encode_socket_addr(addr: &SocketAddr) -> [u8; 18] {
    let mut buf = [0u8; 18];

    match addr {
        SocketAddr::V4(v4) => {
            // convert IPv4 into IPv6-mapped
            let ipv4 = v4.ip().octets();
            buf[10] = 0xff;
            buf[11] = 0xff;
            buf[12..16].copy_from_slice(&ipv4);
            let port = v4.port().to_be_bytes();
            buf[16..18].copy_from_slice(&port);
        }
        SocketAddr::V6(v6) => {
            buf[0..16].copy_from_slice(&v6.ip().octets());
            buf[16..18].copy_from_slice(&v6.port().to_be_bytes());
        }
    }

    buf
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
