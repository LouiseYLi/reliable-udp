use crate::globals::*;
use tokio::net::UdpSocket;

pub async fn handle_msg(
    socket: &UdpSocket,
    expected_seq: &u32,
    buf: &mut [u8],
) -> Result<(), std::io::Error> {
    let (total_len, addr) = socket.recv_from(buf).await?;
    println!("Received {} bytes from {}", total_len, addr);

    // get first 4B (sequence number)
    let seq_bytes: [u8; 4] = buf[0..ACK_START_INDEX].try_into().unwrap();
    let seq = u32::from_be_bytes(seq_bytes);

    // get payload
    let msg = convert_to_string(&buf[MSG_START_INDEX..(total_len)]);

    println!("Expected Seq: {}", expected_seq);
    println!("Seq: {}", seq);
    println!("Message: {}", msg);

    Ok(())
    // socket.send_to(buf[..len], &addr).await?;
}

fn convert_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}
