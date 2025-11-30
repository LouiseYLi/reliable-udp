use crate::ProxyConfig;
use rand::Rng;
use rand::rngs::ThreadRng;
// use rand::thread_rng;
use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::UdpSocket;
use tokio::time::Duration;
use tokio::time::sleep;

pub async fn handle_dg(
    socket: Arc<UdpSocket>,
    proxy_config: &mut ProxyConfig,
    buf: &mut [u8],
    rng: &mut ThreadRng,
    log: Arc<Mutex<File>>,
) -> Result<(), std::io::Error> {
    let receive_str = "[RECEIVE]\n".as_bytes();

    let (total_len, target) = socket.recv_from(buf).await?;
    println!("[RECEIVE] Received from {}...", target);
    // log.write_all(receive_str)?;
    log_write(Arc::clone(&log), receive_str).await?;

    // println!("\tRead {} bytes...", total_len);

    // determine target, c or s ?
    if is_target_server(proxy_config, &target) {
        handle_incoming_dg(
            socket.clone(),
            // proxy_config,
            proxy_config.server_drop,
            proxy_config.server_delay,
            proxy_config.client_addr.clone(),
            proxy_config.server_delay_min,
            proxy_config.server_delay_max,
            buf,
            total_len,
            rng,
            Arc::clone(&log),
        )
        .await?;
    } else {
        verify_client(proxy_config, &target);
        // handle_cp_dg();
        handle_incoming_dg(
            socket.clone(),
            // proxy_config,
            proxy_config.client_drop,
            proxy_config.client_delay,
            proxy_config.server_addr.clone(),
            proxy_config.client_delay_min,
            proxy_config.client_delay_max,
            buf,
            total_len,
            rng,
            Arc::clone(&log),
        )
        .await?;
    }

    // redirect logic
    Ok(())
}

fn verify_client(proxy_config: &mut ProxyConfig, target: &SocketAddr) {
    let client_addr = target.to_string();
    if proxy_config.client_addr != client_addr {
        proxy_config.client_addr = client_addr;
    }
}

fn is_target_server(proxy_config: &mut ProxyConfig, target: &SocketAddr) -> bool {
    proxy_config.server_addr == *target.to_string()
}

#[allow(clippy::too_many_arguments)]
async fn handle_incoming_dg(
    socket: Arc<UdpSocket>,
    drop: u8,
    delay: u8,
    target_addr: String,
    delay_min: u16,
    delay_max: u16,
    buf: &mut [u8],
    total_len: usize,
    rng: &mut ThreadRng,
    log: Arc<Mutex<File>>,
) -> Result<(), std::io::Error> {
    let send_str = "[SEND]\n".as_bytes();
    let drop_str = "[DROP]\n".as_bytes();
    let delay_str = "[DELAY]\n".as_bytes();

    let mut rand_num = rng.gen_range(0..100 + 1);
    // drop if number is less
    if rand_num < drop {
        println!("[DROP] before sending to {}...", target_addr);
        log_write(Arc::clone(&log), drop_str).await?;
        return Ok(());
    }

    // delay if number is less
    rand_num = rng.gen_range(0..100 + 1);
    if rand_num < delay {
        log_write(Arc::clone(&log), delay_str).await?;

        let log_clone = Arc::clone(&log);
        let socket_clone = Arc::clone(&socket);
        let buf_clone = buf[..total_len].to_vec();
        let addr_clone = target_addr.clone();

        let delay_ms: u64 = rng.gen_range(delay_min..delay_max) as u64;

        tokio::spawn(async move {
            let send_str = "[SEND]\n".as_bytes();
            println!(
                "[DELAY] for {}ms before sending to {}...",
                delay_ms, addr_clone
            );
            sleep(Duration::from_millis(delay_ms)).await;
            let _ = socket_clone.send_to(&buf_clone, &addr_clone).await;
            let _ = log_write(log_clone, send_str).await;
        });
        Ok(())
    } else {
        println!("[SEND] Sending to {}...", target_addr);
        socket.send_to(buf, target_addr).await?;
        log_write(log, send_str).await?;
        Ok(())
    }
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
