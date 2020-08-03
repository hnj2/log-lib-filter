use liba;

fn main() {
    simple_logger::init().unwrap();
    log::info!("Running liba::run!");
    liba::run();
    log::info!("Running libb::run!");
    libb::run();
}
