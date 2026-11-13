use crate::rustpad::Rustpad;
use egui::{Button, Layout, RichText, Ui, vec2};
use std::sync::{Arc, Mutex};

pub fn menu_item(ui: &mut Ui, text: &str, shortcut: &str, enabled: bool) -> bool {
    let mut clicked = false;
    let width = ui.available_width();

    ui.allocate_ui_with_layout(
        vec2(width, 0.0),
        Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.horizontal(|ui| {
                if ui.add_enabled(enabled, Button::new(text)).clicked() {
                    clicked = true;
                }

                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new(shortcut).weak());
                });
            });
        },
    );

    clicked
}

pub fn file(app: Arc<Mutex<Rustpad>>, ui: &mut Ui) {
    let mut app = app.lock().unwrap();
    let is_active_buffer = app.buffers().len() > 0;

    let spacing = ui.spacing_mut();
    spacing.item_spacing.y = 0.0;

    if menu_item(ui, "New", "Ctrl+N", true) {
        app.new_buffer();

        ui.close();
    }

    if menu_item(ui, "Open...", "Ctrl+O", true) {
        if let Err(error) = app.open_file() {
            log::error!("Failed to open file: {error:#}");
        }

        ui.close();
    }

    ui.menu_button("Open Containing Folder", |ui| {
        if ui.button("Explorer").clicked() {
            ui.close();
        }

        if ui.button("Command Prompt").clicked() {
            ui.close();
        }
    });

    if ui.button("Open Folder as Workspace").clicked() {
        // TODO: Open folder as workspace

        ui.close();
    }

    ui.separator();

    if menu_item(ui, "Reload from Disk", "Ctrl+R", is_active_buffer) {
        ui.close();
    }

    if menu_item(ui, "Save", "Ctrl+S", is_active_buffer) {
        if let Some(buffer) = app.current_buffer() {
            buffer.save();
        }

        ui.close();
    }

    if menu_item(ui, "Save As...", "Ctrl+Alt+S", is_active_buffer) {
        ui.close();
    }

    if menu_item(ui, "Save All", "Ctrl+Shift+S", is_active_buffer) {
        ui.close();
    }

    ui.separator();

    if ui.button("Rename").clicked() {
        ui.close();
    }

    if menu_item(ui, "Close", "Ctrl+W", is_active_buffer) {
        app.remove_active_buffer();

        ui.close();
    }

    if menu_item(ui, "Close All", "Ctrl+Shift+W", true) {
        ui.close();
    }

    if menu_item(ui, "Remove", "", is_active_buffer) {
        // TODO
        ui.close();
    }

    ui.separator();

    if menu_item(ui, "Print", "Ctrl+P", is_active_buffer) {
        // TODO
        ui.close();
    }

    ui.separator();

    if menu_item(ui, "Exit", "Alt+F4", true) {
        // TODO
    }
}
