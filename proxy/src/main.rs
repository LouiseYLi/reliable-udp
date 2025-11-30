mod args_helper;
mod config;
mod io_helper;

use args_helper::*;
use config::ProxyConfig;
use io_helper::*;

use std::fs::OpenOptions;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> io::Result<()> {

    let file_path = "proxy/log.txt";
    let log = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?,
    ));
    clear_log(&log)?;

    let mut buf = [0u8; 1024];
    let mut rng = rand::thread_rng();

    let mut proxy_config: ProxyConfig = match validate_args() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    println!("\tProxy at {}", proxy_config.proxy_addr);
    println!("\tServer at {}", proxy_config.server_addr);

    println!("\tClient drop chance: {}%", proxy_config.client_drop);
    println!("\tServer drop chance: {}%", proxy_config.server_drop);

    println!("\tClient delay chance: {}%", proxy_config.client_delay);
    println!("\tServer delay chance: {}%", proxy_config.server_delay);

    println!(
        "\tClient delay range: {}–{} ms",
        proxy_config.client_delay_min, proxy_config.client_delay_max
    );

    println!(
        "\tServer delay range: {}–{} ms",
        proxy_config.server_delay_min, proxy_config.server_delay_max
    );

    let socket = Arc::new(UdpSocket::bind(&proxy_config.proxy_addr).await?);
    loop {
        handle_dg(
            socket.clone(),
            &mut proxy_config,
            &mut buf,
            &mut rng,
            Arc::clone(&log),
        )
        .await?;
    }
}
