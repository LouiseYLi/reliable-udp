use clap::{Arg, ArgMatches, Command};
use std::net::IpAddr;

pub fn get_args() -> ArgMatches {
    Command::new("myapp")
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
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .help("Set the timeout (in seconds) for waiting for acknowledgements")
                .num_args(1)
                .required(true),
        )
        .arg(
            Arg::new("max-retries")
                .short('r')
                .long("max-retries")
                .help("Set the maximum number of retries per message")
                .num_args(1)
                .required(true),
        )
        .get_matches()
}

// pub fn validate_args(args: &[String]) -> Result<String, String> {
// let formatted_ip_at_port = format_ip_port(args);
// #[allow(clippy::needless_return)]
// return Ok(formatted_ip_at_port);
// }

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

fn validate_timeout(timeout: &str) -> Result<u16, String> {
    let timeout = match timeout.parse::<u16>() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!("Timeout must be a valid number: {}", timeout));
        }
    };
    Ok(timeout)
}

fn validate_retries(retries: &str) -> Result<u16, String> {
    let max_retries = match retries.parse::<u16>() {
        Ok(n) => n,
        Err(_) => {
            return Err(format!("Max retries must be a valid number: {}", retries));
        }
    };
    Ok(max_retries)
}

fn format_ip_port(ip: &str, port: &str) -> String {
    if ip.contains(':') {
        return format!("[{}]:{}", ip, port);
    }
    format!("{}:{}", ip, port)
}

pub fn validate_args() -> Result<(String, u16, u16), String> {
    let matches = get_args();
    let ip = matches.get_one::<String>("target-ip").expect("IP required");
    let port = matches
        .get_one::<String>("target-port")
        .expect("Port required");
    validate_ip(ip)?;
    validate_port(port)?;

    let timeout_str = matches
        .get_one::<String>("timeout")
        .expect("Timeout required");
    let timeout = validate_timeout(timeout_str)?;

    let max_retries_str = matches
        .get_one::<String>("max-retries")
        .expect("Max retries required");
    let max_retries = validate_retries(max_retries_str)?;

    let formatted_ip_at_port = format_ip_port(ip, port);
    Ok((formatted_ip_at_port, timeout, max_retries))
}
