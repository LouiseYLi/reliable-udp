mod args_helper;
mod globals;
mod io_helper;

use args_helper::*;
use io_helper::*;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello from client!");

    let mut seq: u32 = 0;
    let mut buf = [0u8; 1024];

    let (target, timeout, max_retries) = match validate_args() {
        Ok(values) => values,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    println!("\t Server at {}", &target);
    println!("\t Timeout {}", &timeout);
    println!("\t Retries {}", &max_retries);

    loop {
        handle_msg(&socket, &target, &mut seq, &mut buf, &timeout, &max_retries).await?;
    }
}
