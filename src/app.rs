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
                match ctx.theme() {
                    egui::Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
                    egui::Theme::Light => ctx.set_visuals(egui::Visuals::light()),
                }
            }
            ThemeChoice::Dark => {
                ctx.set_theme(egui::Theme::Dark);
                ctx.set_visuals(egui::Visuals::dark());
            }
            ThemeChoice::DarkGray => {
                ctx.set_theme(egui::Theme::Dark);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(44, 47, 52),
                    window_fill: egui::Color32::from_rgb(54, 58, 64),
                    extreme_bg_color: egui::Color32::from_rgb(36, 39, 43),
                    ..egui::Visuals::dark()
                });
            }
            ThemeChoice::LightGray => {
                ctx.set_theme(egui::Theme::Light);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(210, 214, 220),
                    window_fill: egui::Color32::from_rgb(222, 226, 232),
                    extreme_bg_color: egui::Color32::from_rgb(192, 198, 206),
                    ..egui::Visuals::light()
                });
            }
            ThemeChoice::LightSlate => {
                ctx.set_theme(egui::Theme::Light);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(226, 232, 240),
                    window_fill: egui::Color32::from_rgb(241, 245, 249),
                    extreme_bg_color: egui::Color32::from_rgb(203, 213, 225),
                    hyperlink_color: egui::Color32::from_rgb(71, 85, 105),
                    ..egui::Visuals::light()
                });
            }
            ThemeChoice::DarkSlate => {
                ctx.set_theme(egui::Theme::Dark);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(30, 41, 59),
                    window_fill: egui::Color32::from_rgb(51, 65, 85),
                    extreme_bg_color: egui::Color32::from_rgb(15, 23, 42),
                    hyperlink_color: egui::Color32::from_rgb(148, 163, 184),
                    ..egui::Visuals::dark()
                });
            }
            ThemeChoice::SolarizedLight => {
                ctx.set_theme(egui::Theme::Light);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(253, 246, 227),
                    window_fill: egui::Color32::from_rgb(238, 232, 213),
                    extreme_bg_color: egui::Color32::from_rgb(214, 206, 183),
                    hyperlink_color: egui::Color32::from_rgb(38, 139, 210),
                    ..egui::Visuals::light()
                });
            }
            ThemeChoice::SolarizedDark => {
                ctx.set_theme(egui::Theme::Dark);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(0, 43, 54),
                    window_fill: egui::Color32::from_rgb(7, 54, 66),
                    extreme_bg_color: egui::Color32::from_rgb(88, 110, 117),
                    hyperlink_color: egui::Color32::from_rgb(42, 161, 152),
                    ..egui::Visuals::dark()
                });
            }
            ThemeChoice::WinterIsComing => {
                ctx.set_theme(egui::Theme::Dark);
                ctx.set_visuals(egui::Visuals {
                    panel_fill: egui::Color32::from_rgb(22, 30, 45),
                    window_fill: egui::Color32::from_rgb(31, 41, 55),
                    extreme_bg_color: egui::Color32::from_rgb(15, 23, 36),
                    hyperlink_color: egui::Color32::from_rgb(125, 211, 252),
                    ..egui::Visuals::dark()
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

        let scale = match self.state.persisted.settings.text_scale {
            TextScale::Pct90 => 0.90,
            TextScale::Pct100 => 1.00,
            TextScale::Pct110 => 1.10,
            TextScale::Pct125 => 1.25,
            TextScale::Pct140 => 1.40,
        };

        ctx.set_pixels_per_point(scale);
    }
}

impl eframe::App for IdmHelperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme_and_scale(ctx);

        // Application version for footer
        let version = env!("CARGO_PKG_VERSION");

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

        egui::CentralPanel::default().show(ctx, |ui| match self.state.active_tab {
            ActiveTab::Home => ui::panels::home_panel::show(ui, &mut self.state),
            ActiveTab::Settings => ui::panels::settings_panel::show(ui, &mut self.state),
            ActiveTab::Approvers => ui::panels::approvers_panel::show(ui, &mut self.state),
        });

        egui::TopBottomPanel::bottom("footer_bar").show(ctx, |ui| {
            ui.set_height(20.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("v{}", version)).weak());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Created By: John Swan | Feb 2026").weak());
                });
            });
        });
        ui::widgets::toast::show(ctx, &mut self.state.toast);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state.persisted);
    }
}