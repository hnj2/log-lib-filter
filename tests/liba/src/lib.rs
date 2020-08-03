
#[macro_use]
extern crate log_lib_filter;

pub fn run() {
    trace!("liba trace");
    debug!("liba debug");
    info!("liba info");
    warn!("liba warning");
    error!("liba error");
}
