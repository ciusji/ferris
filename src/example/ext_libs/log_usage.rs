//! Copyright 2021 Ferris Project Authors. License user Apache License.

use log::{debug, error, info, warn};

pub fn log_usage() {
    env_logger::init();

    let _data = (42, "forty-two");
    let _private_data = "private data";

    // log!(Level::Error, "received errors: {}, {}", data.0, data.1);
    // log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}", data.0, data.1, private_data);

    debug!("this is a debug level log");
    info!("this is a info level log");
    warn!("this ia a warn level log");

    let (err_info,port) = ("No connection", 22);
    error!("error:{} on port {}", err_info, port);
}