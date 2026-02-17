use eframe::egui;

use crate::state::{ActiveTab, AppState, PersistedState, TextScale, ThemeChoice};
use crate::ui;

pub struct IdmHelperApp {
    state: AppState,
}

impl IdmHelperApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let persisted = cc
            .storage
            .and_then(|storage| eframe::get_value::<PersistedState>(storage, eframe::APP_KEY))
            .unwrap_or_default();
        
        let mut state = AppState::default();
        state.persisted = persisted;

        Self { state }
    }

    fn apply_theme_and_scale(&self, ctx: &egui::Context) {
        match self.state.persisted.settings.theme {
            ThemeChoice::System => {
                ctx.set_theme(egui::ThemePreference::System);
            }
            ThemeChoice::Dark => {
                ctx.set_theme(egui::Theme::Dark);
            }
            ThemeChoice::LightGray => {
                ctx.set_theme(egui::Theme::Light);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(238, 240, 243),
                    window_fill: egui::Color32::from_rgb(246, 247, 249),
                    extreme_bg_color: egui::Color32::from_rgb(224, 227, 231),
                    ..egui::Visuals::light()
                });
            }
            ThemeChoice::Nord => {
                ctx.set_theme(egui::Theme::Dark);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(46, 52, 64),
                    window_fill: egui::Color32::from_rgb(59, 66, 82),
                    extreme_bg_color: egui::Color32::from_rgb(67, 76, 94),
                    hyperlink_color: egui::Color32::from_rgb(136, 192, 208),
                    ..egui::Visuals::dark()
                });
            }
        }
    })
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