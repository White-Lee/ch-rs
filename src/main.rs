#![windows_subsystem = "windows"]

use log4rs;
use log::{error, info, warn};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    init_logging();
    info!("开始");
    warn!("booting up");
    let ui = AppWindow::new()?;
    ui.run()
}

fn init_logging() {
    log4rs::init_file("conf/log4rs.yaml", Default::default()).unwrap();
    error!("错误");
}