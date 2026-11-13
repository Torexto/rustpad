use crate::rustpad::Rustpad;
use egui::Ui;
use std::sync::{Arc, Mutex};

pub fn buffers_list(app: Arc<Mutex<Rustpad>>, ui: &mut Ui) {
    let mut app = app.lock().unwrap();

    let length = app.buffers().len();

    ui.horizontal(|ui| {
        let mut selected_buffer = None;
        let mut removed_buffer = None;

        for buffer_index in 0..length {
            ui.horizontal(|ui| {
                let title = app.buffers()[buffer_index].title().to_owned();

                if ui
                    .selectable_label(app.active_buffer_index() == Some(buffer_index), title)
                    .clicked()
                {
                    selected_buffer = Some(buffer_index);
                }

                if ui.button("X").clicked() {
                    removed_buffer = Some(buffer_index);
                }
            });
        }

        if let Some(buffer_index) = removed_buffer {
            app.remove_buffer(buffer_index);
        } else if let Some(buffer_index) = selected_buffer {
            app.set_buffer(buffer_index);
        }
    });
}
