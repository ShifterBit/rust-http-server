use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: Option<u32>,
    pub routes: Vec<RouteConfig>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RouteConfig {
    pub location: String,
    pub source: String,
}

pub fn read_route_config(path: &str) -> Config {
    let mut f = File::open(path).unwrap();
    let mut buffer = String::new();

    f.read_to_string(&mut buffer).unwrap();

    let mut config: Config = toml::from_str(&buffer).unwrap();

    config
        .routes
        .sort_by(|a, b| b.location.len().cmp(&a.location.len()));
    config
}

pub fn get_port(config: &Config) -> Option<u32> {
    match config.port {
        Some(port) => {
            println!("Running on Port {port}");
            Some(port)
        }
        None => match env::var("PORT") {
            Ok(port) => {
                println!("Running on Port {port}");
                Some(port.parse::<u32>().unwrap())
            }
            Err(_) => {
                println!("No Port Configured");
                println!("Please set PORT or configure ports in the configuration file");
                None
            }
        },
    }
}
