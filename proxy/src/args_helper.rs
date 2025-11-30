use crate::config::ProxyConfig;

use clap::{Arg, ArgMatches, Command};
use std::net::IpAddr;

fn get_args() -> ArgMatches {
    Command::new("proxy")
        .arg(
            Arg::new("listen-ip")
                .long("listen-ip")
                .help("Set the proxy server IP")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("listen-port")
                .long("listen-port")
                .help("Set the proxy server port")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("target-ip")
                .long("target-ip")
                .help("Set the server IP")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("target-port")
                .long("target-port")
                .help("Set the server port")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("client-drop")
                .long("client-drop")
                .help("Set the drop chance (%) for client packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("server-drop")
                .long("server-drop")
                .help("Set the drop chance (%) for server packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("client-delay")
                .long("client-delay")
                .help("Set the delay chance (%) for client packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("server-delay")
                .long("server-delay")
                .help("Set the delay chance (%) for server packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("client-delay-time-min")
                .long("client-delay-time-min")
                .help("Set the minimum delay (ms) for client packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("client-delay-time-max")
                .long("client-delay-time-max")
                .help("Set the maximum delay (ms) for client packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("server-delay-time-min")
                .long("server-delay-time-min")
                .help("Set the minimum delay (ms) for server packets")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("server-delay-time-max")
                .long("server-delay-time-max")
                .help("Set the maximum delay (ms) for server packets")
                .num_args(1)
                .required(true),
        )
        .get_matches()
}

fn validate_ip(ip: &str) -> Result<(), String> {
    // ::1 ipv6 loopback addr
    if ip.parse::<IpAddr>().is_err() && ip != "::1" {
        return Err(format!(
            "Invalid IP address... Neither IPv4 or IPv6: {}",
            ip
        ));
    }
    Ok(())
}

fn validate_port(port: &str) -> Result<(), String> {
    match port.parse::<u16>() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!(
                "Port must be a valid number between 0-65535: {}",
                port
            ));
        }
    };

    Ok(())
}

fn format_ip_port(ip: &str, port: &str) -> String {
    if ip.contains(':') {
        return format!("[{}]:{}", ip, port);
    }
    format!("{}:{}", ip, port)
}

fn validate_chance(chance: &str) -> Result<u8, String> {
    let chance_num = match chance.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!("Chance must be a valid number: {}", chance));
        }
    };
    if chance_num > 100 {
        Err(format!("Chance must be a valid percentage: {}", chance_num))
    } else {
        Ok(chance_num)
    }
}

fn validate_delay_range(min: &str, max: &str) -> Result<(u16, u16), String> {
    let min_delay = match min.parse::<u16>() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!("Min delay must be a valid number: {}", min));
        }
    };
    let max_delay = match max.parse::<u16>() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!("Max delay must be a valid number: {}", max));
        }
    };
    if max_delay < min_delay {
        Err(format!(
            "Range is not valid. Min: {} Max: {}",
            min_delay, max_delay
        ))
    } else {
        Ok((min_delay, max_delay))
    }
}

pub fn validate_args() -> Result<ProxyConfig, String> {
    let matches = get_args();

    // proxy server
    let listen_ip = matches.get_one::<String>("listen-ip").expect("IP required");
    let listen_port = matches
        .get_one::<String>("listen-port")
        .expect("Port required");
    validate_ip(listen_ip)?;
    validate_port(listen_port)?;

    // server
    let target_ip = matches.get_one::<String>("target-ip").expect("IP required");
    let target_port = matches
        .get_one::<String>("target-port")
        .expect("Port required");
    validate_ip(target_ip)?;
    validate_port(target_port)?;

    let client_drop_str = matches
        .get_one::<String>("client-drop")
        .expect("Client drop chance required");
    let server_drop_str = matches
        .get_one::<String>("server-drop")
        .expect("Server drop chance required");
    let client_drop = validate_chance(client_drop_str)?;
    let server_drop = validate_chance(server_drop_str)?;

    let client_delay_str = matches
        .get_one::<String>("client-delay")
        .expect("Client delay chance required");
    let server_delay_str = matches
        .get_one::<String>("server-delay")
        .expect("Server delay chance required");
    let client_delay = validate_chance(client_delay_str)?;
    let server_delay = validate_chance(server_delay_str)?;

    let client_delay_time_min_str = matches
        .get_one::<String>("client-delay-time-min")
        .expect("Client delay time min required");
    let client_delay_time_max_str = matches
        .get_one::<String>("client-delay-time-max")
        .expect("Client delay time max required");
    let (client_delay_min, client_delay_max) =
        validate_delay_range(client_delay_time_min_str, client_delay_time_max_str)?;

    let server_delay_time_min_str = matches
        .get_one::<String>("server-delay-time-min")
        .expect("Server delay time min required");
    let server_delay_time_max_str = matches
        .get_one::<String>("server-delay-time-max")
        .expect("Server delay time max required");
    let (server_delay_min, server_delay_max) =
        validate_delay_range(server_delay_time_min_str, server_delay_time_max_str)?;

    Ok(ProxyConfig {
        proxy_addr: format_ip_port(listen_ip, listen_port),
        server_addr: format_ip_port(target_ip, target_port),
        client_addr: "_".to_string(),

        client_drop,
        server_drop,

        client_delay,
        server_delay,

        client_delay_min,
        client_delay_max,

        server_delay_min,
        server_delay_max,
    })
}
