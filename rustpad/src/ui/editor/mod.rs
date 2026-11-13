use crate::rustpad::Rustpad;
use egui::Ui;
use std::sync::{Arc, Mutex};

pub fn editor(app: Arc<Mutex<Rustpad>>, ui: &mut Ui) {
    let mut app = app.lock().unwrap();

    egui::ScrollArea::vertical().show(ui, |ui| {
        if let Some(buffer) = app.current_buffer_mut() {
            ui.add(egui::TextEdit::multiline(&mut buffer.content));
        }
    });
}
