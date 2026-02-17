// Most of this is placeholder code. The user will not be asked to paste text in this pane.
use eframe::egui;
use crate::state::AppState;
pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Users");
    ui.label("Paste IDM role export text below");
    ui.add_sized(
        ui.available_size(),
        egui::TextEdit::multiline(&mut state.input_users_raw)
            .hint_text("Paste copied IDM rows here..."),
    );
}