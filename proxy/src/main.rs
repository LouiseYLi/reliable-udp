mod args_helper;
mod config;

use args_helper::*;
use config::ProxyConfig;

fn main() {
    println!("Hello from proxy!");

    let proxy_config: ProxyConfig = match validate_args() {
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
}
