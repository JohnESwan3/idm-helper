use eframe::egui;

use crate::state::{ActiveTab, AppState};
use crate::ui;

pub struct IdmHelperApp {
    state: AppState,
}

impl IdmHelperApp {
    pub fn new(_cc:&eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for IdmHelperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top navigation
        egui::TopBottomPanel::top("top_tabs").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading("IDM Helper");
                ui.separator();

                for tab in ActiveTab::all() {
                    let selected = self.state.active_tab == tab;
                    if ui.selectable_label(selected, tab.label()).clicked() {
                        self.state.active_tab = tab;
                    }
                }
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| match self.state.active_tab {
            ActiveTab::Home => ui::panels::home_panel::show(ui, &mut self.state),
            ActiveTab::Settings => ui::panels::settings_panel::show(ui, &mut self.state),
            ActiveTab::Approvers => ui::panels::approvers_panel::show(ui, &mut self.state),
        });

        // Global Notifications
        ui::widgets::toast::show(ctx, &mut self.state.toast);
    }
}