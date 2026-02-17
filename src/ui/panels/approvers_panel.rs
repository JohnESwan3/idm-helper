use eframe::egui;
use crate::state::AppState;

pub fn show(ui: &mut egui::Ui, _state: &mut AppState) {
    ui.heading("Script");
    ui.label("Script instructions coming later");
}
