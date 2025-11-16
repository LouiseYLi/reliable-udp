use clap::{Arg, ArgMatches, Command};
use std::net::IpAddr;

pub fn get_args() -> ArgMatches {
    Command::new("server")
        .arg(
            Arg::new("target-ip")
                .short('i')
                .long("target-ip")
                .help("Set the server IP")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("target-port")
                .short('p')
                .long("target-port")
                .help("Set the server port")
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

pub fn validate_args() -> Result<String, String> {
    let matches = get_args();
    let ip = matches.get_one::<String>("target-ip").expect("IP required");
    let port = matches
        .get_one::<String>("target-port")
        .expect("Port required");
    validate_ip(ip)?;
    validate_port(port)?;

    let formatted_ip_at_port = format_ip_port(ip, port);
    Ok(formatted_ip_at_port)
}
