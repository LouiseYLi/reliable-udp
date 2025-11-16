use clap::{Arg, ArgMatches, Command};
use std::net::IpAddr;

pub fn get_args() -> ArgMatches {
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

pub fn validate_args() -> Result<(String, u64, u16), String> {
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
