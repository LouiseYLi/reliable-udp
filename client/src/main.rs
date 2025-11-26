mod args_helper;
mod globals;
mod io_helper;

use args_helper::*;
use io_helper::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
// use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // println!("Hello from client!");

    let file_path = "client/log.txt";
    // let initial_content = "hello";
    // let content0 = "cat\n";
    // let content1 = "dog\n";

    // let mut file = File::create(file_path)?;
    // clear file contents
    let log = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?,
    ));
    // let mut log = OpenOptions::new().append(true).open(file_path)?;
    // log.set_len(0)?;
    clear_log(&log)?;
    // file.write_all(initial_content.as_bytes())?;

    // file.write_all(content0.as_bytes())?;
    // file.write_all(content1.as_bytes())?;

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
