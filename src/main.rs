#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod core;
mod state;
mod ui;

use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("IDM Helper")
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([340.0, 340.0]),
        ..Default::default()
    };

    eframe::run_native(
        "IDM Helper",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::ThemePreference::System);
            Ok(Box::new(app::IdmHelperApp::new(cc)))
        }),
    )
}