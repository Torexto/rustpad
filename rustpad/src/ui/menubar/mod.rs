pub mod edit;
pub mod file;
pub mod search;
pub mod view;
mod encoding;
mod language;
mod settings;
mod tool;
mod run;
mod help;

use crate::rustpad::Rustpad;
use egui::style::StyleModifier;
use egui::{Ui, Vec2};
use std::sync::{Arc, Mutex};

pub struct MenuBar;
pub fn menubar(app: Arc<Mutex<Rustpad>>, ui: &mut Ui) {
    let style_modifier = StyleModifier::new(|style| {
        style.spacing.button_padding = Vec2::new(8.0, 5.0);
    });

    egui::MenuBar::new().style(style_modifier).ui(ui, |ui| {
        ui.menu_button("File", |ui| file::file(app.clone(), ui));
        ui.menu_button("Edit", |ui| edit::edit(app.clone(), ui));
        ui.menu_button("Search", |ui| search::search(app.clone(), ui));
        ui.menu_button("View", |ui| view::view(app.clone(), ui));
        ui.menu_button("Encoding", |ui| encoding::encoding(app.clone(), ui));
        ui.menu_button("Language", |ui| language::language(app.clone(), ui));
        ui.menu_button("Settings", |ui| settings::settings(app.clone(), ui));
        ui.menu_button("Tools", |ui| tool::tool(app.clone(), ui));
        ui.menu_button("Run", |ui| run::run(app.clone(), ui));
        ui.menu_button("Help", |ui| help::help(app.clone(), ui));
    });
}
