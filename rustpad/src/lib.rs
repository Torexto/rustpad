pub mod buffer;
pub mod rustpad;
pub mod ui;

use eframe::Frame;
use std::sync::{Arc, Mutex};
use ui::buffer_list::buffers_list;
use ui::editor::editor;
use ui::menubar::menubar;

pub struct App {
    rustpad: Arc<Mutex<rustpad::Rustpad>>,
}

impl eframe::App for App {
    #[allow(unused_variables)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut Frame) {
        let app = self.rustpad.clone();

        egui::CentralPanel::default().show(ui, |ui| {
            menubar(app.clone(), ui);
            buffers_list(app.clone(), ui);
            editor(app.clone(), ui);
        });
    }
}

impl App {
    #[allow(unused_variables)]
    pub fn new(cc: &eframe::CreationContext) -> Self {
        Self {
            rustpad: Arc::new(Mutex::new(rustpad::Rustpad::default())),
        }
    }

    pub fn run(title: &str, options: eframe::NativeOptions) -> eframe::Result {
        eframe::run_native(title, options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
    }
}
