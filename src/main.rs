mod cluster;
mod config;
mod log;
mod serve;
mod utils;

pub const PROTOCOL_VERSION: &str = "1.7.3";

fn main() {
    log::init_log_with_cli();
}
