use crate::configs;

pub fn export_hosts(cfg: &configs::WireguardNetworkInfo) -> Result<String, String> {
    // TODO: replace with some table lib
    let mut built = String::new();

    built += format!("# Hosts for Wireguard network \"{}\"\n", cfg.name).as_str();
    built += "# Generated by wgbond\n";
    for peer in cfg.real_peers() {
        let wg_peer = cfg.map_to_interface(peer)?;
        built += format!(
            "{ip}\t{name}.{network}\n",
            name = peer.name,
            ip = wg_peer
                .address
                .iter()
                .map(|a| a.to_string())
                .next()
                .unwrap(),
            network = cfg.name
        )
        .as_str();
    }
    Ok(built)
}
