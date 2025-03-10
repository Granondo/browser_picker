use crate::app::BrowserPicker;
use eframe::egui;
use std::process::Command;

impl BrowserPicker {
    pub fn show_browser_picker_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading(egui::RichText::new("Choose Browser")
                .size(24.0)
                .color(if self.dark_mode { self.theme.foreground } else { self.theme.foreground }));
            
            ui.add_space(16.0);
            
            // URL display
            ui.label(egui::RichText::new(&self.url)
                .size(14.0)
                .color(self.theme.secondary));
            
            ui.add_space(16.0);
            
            // Browser buttons
            for (name, path, _) in &self.browsers {
                let button = egui::Button::new(
                    egui::RichText::new(name)
                        .size(16.0)
                        .color(if self.dark_mode { self.theme.foreground } else { self.theme.foreground })
                )
                .min_size(egui::vec2(200.0, 40.0))
                .rounding(8.0)
                .fill(if self.dark_mode { self.theme.background } else { self.theme.background });
                
                let mut response = ui.add(button);
                
                if response.hovered() {
                    response.mark_changed();
                    ui.painter().rect_filled(
                        response.rect,
                        8.0,
                        if self.dark_mode {
                            egui::Color32::from_white_alpha(10)
                        } else {
                            egui::Color32::from_black_alpha(10)
                        },
                    );
                }
                
                if response.clicked() {
                    // Save preference if enabled
                    if self.config.remember_choice_for_domain {
                        if let Some(domain) = crate::browser::extract_domain(&self.url) {
                            self.config.domain_preferences.insert(domain.clone(), name.clone());
                            self.config.save().ok();
                        }
                    }
                    
                    Command::new(path)
                        .arg(&self.url)
                        .spawn()
                        .ok();
                    frame.close();
                }
                
                ui.add_space(4.0);
            }
            
            ui.add_space(16.0);
            
            // Settings button
            if ui.button(
                egui::RichText::new("⚙ Settings")
                    .size(14.0)
                    .color(self.theme.secondary)
            ).clicked() {
                self.show_settings = true;
            }
        });
    }
} 