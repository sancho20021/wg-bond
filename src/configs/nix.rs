// ! Wireguard conf file
// Better way of doing this is invoking builtins.fromJSON, but that's not portable.

use crate::configs::*;

pub struct NixConf {}

impl ConfigType for NixConf {

  fn write_config(net: &WireguardNetworkInfo, id: u128) -> String {

    let my_peer = net.by_id(id).unwrap();
    let other_peers: Vec<&PeerInfo> = net.peer_list(my_peer);
    let interface = net.map_to_interface(my_peer);

    fn set_assign(key: &str, value: &Option<impl core::fmt::Display>) -> String {
      match value {
        Some(val) => {
          format!("{}=\"{}\";", key, val)
        }
        _ => {
          "".into()
        }
      }
    }


    fn set_assign_raw(key: &str, value: &Option<impl core::fmt::Display>) -> String {
      match value {
        Some(val) => {
          format!("{}={};", key, val)
        }
        _ => {
          "".into()
        }
      }
    }

    let mut built = String::new();
    built += format!("networking.wg-quick.interfaces.\"{}\"={{", &net.name).as_str();
    built += format!("privateKey=\"{}\";", &my_peer.private_key).as_str();

    built += set_assign_raw("listenPort", &my_peer.endpoint.as_ref().map(get_port)).as_str();

    fn wrap_string<T>(thing: &T) -> String where T : core::fmt::Display {
      format!("\"{}\"", thing)
    }

    // Addresses
    built += format!("address=[{}];", &interface.address.iter().map(wrap_string).collect::<Vec<String>>().join(" ")).as_str();

    built += set_assign("preUp", &interface.pre_up).as_str();
    built += set_assign("preDown", &interface.pre_down).as_str();
    built += set_assign("postUp", &interface.post_up).as_str();
    built += set_assign("postDown", &interface.post_down).as_str();

    // Peers
    fn encode_peer(peer: Peer) -> String {
      let mut built = String::new();
      built += "{";
      built += set_assign("publicKey", &Some(&peer.public_key)).as_str();
      built += format!("allowedIPs=[{}];", peer.allowed_ips.iter().map(wrap_string).collect::<Vec<String>>().join(" ")).as_str();
      built += set_assign_raw("persistentKeepalive", &peer.persistent_keepalive).as_str();
      built += set_assign("presharedKey", &peer.preshared_key).as_str();
      built += set_assign("endpoint", &peer.endpoint).as_str();
      built += "}";
      built
    }

    built += format!("peers=[{}];", &other_peers.iter().map(|peer| encode_peer(net.map_to_peer(peer))).collect::<Vec<String>>().join(" ")).as_str();

    built += "};";

    built

  }
}
