pub struct ProxyConfig {
    pub proxy_addr: String,
    pub server_addr: String,
    pub client_addr: String,

    pub client_drop: u8,
    pub server_drop: u8,

    pub client_delay: u8,
    pub server_delay: u8,

    pub client_delay_min: u16,
    pub client_delay_max: u16,

    pub server_delay_min: u16,
    pub server_delay_max: u16,
}
