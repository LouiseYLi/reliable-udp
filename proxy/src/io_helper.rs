use crate::ProxyConfig;
use rand::Rng;
use rand::rngs::ThreadRng;
// use rand::thread_rng;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::time::Duration;
use tokio::time::sleep;
pub async fn handle_dg(
    socket: Arc<UdpSocket>,
    proxy_config: &mut ProxyConfig,
    buf: &mut [u8],
    rng: &mut ThreadRng,
) -> Result<(), std::io::Error> {
    let (total_len, target) = socket.recv_from(buf).await?;
    println!("[OK PROXY] Received from {}...", target);
    println!("\tRead {} bytes...", total_len);

    // determine target, c or s ?
    if is_target_server(proxy_config, &target) {
        handle_ps_dg(
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
        )
        .await?;
    } else {
        verify_client(proxy_config, &target);
        // handle_cp_dg();
        handle_ps_dg(
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
async fn handle_ps_dg(
    socket: Arc<UdpSocket>,
    drop: u8,
    delay: u8,
    target_addr: String,
    delay_min: u16,
    delay_max: u16,
    buf: &mut [u8],
    total_len: usize,
    rng: &mut ThreadRng,
) -> Result<(), std::io::Error> {
    let mut rand_num = rng.gen_range(0..100 + 1);
    // drop if number is less
    // if rand_num < proxy_config.server_drop {
    if rand_num < drop {
        println!("[DROP] Sending to {}...", target_addr);
        return Ok(());
    }

    // delay if number is less
    rand_num = rng.gen_range(0..100 + 1);
    // if rand_num < proxy_config.server_delay {
    if rand_num < delay {
        let socket_clone = Arc::clone(&socket);
        let buf_clone = buf[..total_len].to_vec();
        let addr_clone = target_addr.clone();
        // let client_addr_clone = proxy_config.client_addr.clone();

        // let delay_ms: u64 =
        //     rng.gen_range(proxy_config.server_delay_min..proxy_config.server_delay_max) as u64;
        let delay_ms: u64 = rng.gen_range(delay_min..delay_max) as u64;
        println!("\tDelaying packet to {} for {}ms...", target_addr, delay_ms);

        tokio::spawn(async move {
            sleep(Duration::from_millis(delay_ms)).await;
            println!("[DELAY] Sending to {}...", addr_clone);
            let _ = socket_clone.send_to(&buf_clone, &addr_clone).await;
        });
        Ok(())
    } else {
        println!("[OK PROXY] Sending to {}...", target_addr);
        socket.send_to(buf, target_addr).await?;
        Ok(())
    }
}
