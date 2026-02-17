use eframe::egui;
use crate::state::ToastState;

pub fn show(ctx: &egui::Context, toast: &mut ToastState) {
    if let Some(msg) = toast.message.clone() {
        let mut dismiss = false;

        egui::TopBottomPanel::bottom("toast_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(msg);
                if ui.button("Dismiss").clicked() {
                    dismiss = true;
                }
            });
        });

        if dismiss {
            toast.message = None;
        }
    }
}