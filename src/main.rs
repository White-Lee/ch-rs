#![windows_subsystem = "windows"]

use log4rs;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    init_logging();
    let ui = AppWindow::new()?;
    ui.run()
}

fn init_logging() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
}