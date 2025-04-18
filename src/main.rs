use crate::config::Config;

pub mod config;

fn main() {
    env_logger::init();

    println!("Hello, world!");
    let cfg = Config::load_from_file("config.yaml").unwrap();
    println!("{:?}", cfg);
}
