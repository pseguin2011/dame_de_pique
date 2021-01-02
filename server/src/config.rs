use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr};
use std::env;

const DEFAULT_NETWORK_CONFIG_PATH: &str = "../common/network.json";

/// Server Configuration of ip and port
#[derive(Debug)]
pub struct Config {
    pub websocket_host: String,
    pub host: IpAddr,
    pub port: u16,
}

/// JSON representation of the loaded json data
#[derive(Debug, Clone, Deserialize)]
pub struct ConfigJson {
    websocket_host: String,
    server_host: String,
    server_port: u16,
}

impl Into<Config> for ConfigJson {
    fn into(self) -> Config {
        // Host must be format XXX.XXX.XXX.XXX - EX: 127.0.0.1
        // Port must be u16 integer
        let mut host_string = self.server_host.split('.');
        if let (Some(a), Some(b), Some(c), Some(d)) = (
            host_string.next(),
            host_string.next(),
            host_string.next(),
            host_string.next(),
        ) {
            return Config {
                websocket_host: self.websocket_host,
                host: IpAddr::V4(Ipv4Addr::new(
                    a.parse::<u8>().unwrap(),
                    b.parse::<u8>().unwrap(),
                    c.parse::<u8>().unwrap(),
                    d.parse::<u8>().unwrap(),
                )),
                port: self.server_port,
            };
        }
        panic!();
    }
}

/// Loads the network json file and maps it to the config json struct
/// Then converts the json struct to a Config object
///
/// # Returns
/// A config object with the host and port of the server
pub fn load_config() -> Config {
    let path = match env::var("NETWORK_CONFIG_PATH") {
        Ok(path) => path,
        Err(_) => DEFAULT_NETWORK_CONFIG_PATH.to_string(),
    };

    println!("Path: {}", path);
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let config: ConfigJson = serde_json::from_str(&data).unwrap();
    config.into()
}
