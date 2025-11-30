mod args_helper;
mod globals;
mod io_helper;

use args_helper::*;
use io_helper::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let file_path = "client/log.txt";

    let log = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?,
    ));
    clear_log(&log)?;

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
        handle_msg(
            &socket,
            &target,
            &mut seq,
            &mut buf,
            &timeout,
            &max_retries,
            Arc::clone(&log),
        )
        .await?;
    }
}
