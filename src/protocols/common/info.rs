pub fn build_protocol_info_packet(protocol_version: u8) -> Vec<u8> {
    const SERVER_NAME: &str = "Gashishnik Server";
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let mut packet = Vec::with_capacity(1 + SERVER_NAME.len() + VERSION.len() + 1);
    packet.push(protocol_version);
    packet.extend_from_slice(format!("{SERVER_NAME} v{VERSION}").as_bytes());
    packet
}

pub fn protocol_info_string() -> String {
    format!(
        "{} v{} (protocol 0x03, RAC/WRAC v2.0)",
        "Gashishnik Server",
        env!("CARGO_PKG_VERSION")
    )
}