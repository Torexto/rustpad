#![windows_subsystem = "windows"]

use rustpad::App;

fn main() -> eframe::Result {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    
    App::run("Rustpad", options)
}
    
