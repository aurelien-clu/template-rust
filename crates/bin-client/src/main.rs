extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
