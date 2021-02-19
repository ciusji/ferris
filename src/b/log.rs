extern crate log;
extern crate env_logger;

use self::log::{debug, error, info};

pub fn info() {
    env_logger::init();
    debug!("debug");
    info!("info");
    error!("error");
}