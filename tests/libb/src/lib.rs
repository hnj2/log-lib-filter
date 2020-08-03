
#[macro_use]
extern crate log_lib_filter;

pub fn run() {
    trace!("libb trace");
    debug!("libb debug");
    info!("libb info");
    warn!("libb warning");
    error!("libb error");
}
