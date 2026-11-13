#![windows_subsystem = "windows"]

use rustpad::AppWindow;

fn main() {
    env_logger::init();

    AppWindow::run();
}
