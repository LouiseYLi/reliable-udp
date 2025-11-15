mod args_helper;
// mod globals;
mod io_helper;

use args_helper::*;
use io_helper::*;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello from client!");

    let seq: u32 = 0;
    // let expected_ack: u32 = 0;
    // let buf = [0u8; 1024];

    let (target, timeout, retries) = match validate_args() {
        Ok(values) => values,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    println!("\t Server at {}", &target);
    println!("\t Timeout {}", &timeout);
    println!("\t Retries {}", &retries);

    loop {
        handle_user_input(&socket, &target, &seq).await?;
        // handle_ack(&socket, &expected_ack, &mut buf).await?;
    }
}
