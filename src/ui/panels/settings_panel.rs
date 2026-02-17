use eframe::egui;

use crate::state::{AppState, TextScale, ThemeChoice, UiSettings};

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    let settings = &mut state.persisted.settings;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.heading("Settings");
            ui.label(
                egui::RichText::new("Customize the Application.")
                    .weak(),
            );
            ui.add_space(12.0);

            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.strong("Appearance");
                ui.label("Theme and Text Scaling");
                ui.add_space(8.0);

                let narrow = ui.available_width() < 520.0;
                let combo_width = (ui.available_width() - 24.0).clamp(140.0, 220.0);

                if narrow {
                    ui.label("Theme");
                    egui::ComboBox::from_id_salt("settings_theme_combo")
                        .width(combo_width)
                        .selected_text(settings.theme.label())
                        .show_ui(ui, |ui| {
                            for choice in ThemeChoice::all() {
                                ui.selectable_value(&mut settings.theme, choice, choice.label());
                            }
                        });

                    ui.add_space(10.0);

                    ui.label("Text Scale");
                    egui::ComboBox::from_id_salt("settings_text_scale_combo")
                        .width(combo_width)
                        .selected_text(settings.text_scale.label())
                        .show_ui(ui, |ui| {
                            for scale in TextScale::all() {
                                ui.selectable_value(&mut settings.text_scale, scale, scale.label());
                            }
                        });
                } else {
                    egui::Grid::new("settings_appearance_grid")
                        .num_columns(2)
                        .min_col_width(160.0)
                        .spacing([16.0, 12.0])
                        .show(ui, |ui| {
                            ui.label("Theme");
                            egui::ComboBox::from_id_salt("settings_theme_combo")
                                .width(220.0)
                                .selected_text(settings.theme.label())
                                .show_ui(ui, |ui| {
                                    for choice in ThemeChoice::all() {
                                        ui.selectable_value(&mut settings.theme, choice, choice.label());
                                    }
                                });
                            ui.end_row();

                            ui.label("Text Scale");
                            egui::ComboBox::from_id_salt("settings_text_scale_combo")
                                .width(220.0)
                                .selected_text(settings.text_scale.label())
                                .show_ui(ui, |ui| {
                                    for scale in TextScale::all() {
                                        ui.selectable_value(
                                            &mut settings.text_scale,
                                            scale,
                                            scale.label(),
                                        );
                                    }
                                });
                            ui.end_row();
                        });
                }
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(12.0);
            });
            ui.group(|ui| {
                ui.set_width(ui.available_width());
                ui.strong("Danger Zone");
                ui.label(egui::RichText::new("Reset to Default Settings").weak());
                ui.add_space(8.0);

                if ui.button("Reset settings").clicked() {
                    *settings = UiSettings::default();
                    state.toast.message = Some("Settings reset to defaults.".to_owned());
                }
            });

            ui.add_space(10.0);
            ui.label(egui::RichText::new("Settings are saved automatically.").weak());
        });
}