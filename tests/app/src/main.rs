
fn main() {
    simple_logger::init().unwrap();
    println!("### Running app::main!");
    log::error!("app info!");
    log::warn!("app info!");
    log::info!("app info!");
    log::debug!("app info!");
    log::trace!("app info!");
    println!("### Running liba::run!");
    liba::run();
    println!("### Running libb::run!");
    libb::run();
}
