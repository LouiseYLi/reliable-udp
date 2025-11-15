mod args_helper;
mod globals;
mod io_helper;

use args_helper::*;
// use globals::*;
use io_helper::*;
use std::io;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Hello from server!");

    let expected_seq: u32 = 0;
    let mut buf = [0u8; 1024];

    let formatted_ip_at_port = match validate_args() {
        Ok(values) => values,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let socket = UdpSocket::bind(&formatted_ip_at_port).await?;
    println!("\tServer is listening on {}", &formatted_ip_at_port);

    loop {
        handle_msg(&socket, &expected_seq, &mut buf).await?;
    }
}
