use eframe::egui;
use crate::ui::LayoutMode;

pub struct StatusPanel;

impl StatusPanel {
    pub fn render(ui: &mut egui::Ui, layout_mode: LayoutMode, status_message: &str) {
        ui.add_space(2.0);
        ui.horizontal(|ui| {
            ui.label(status_message);
            ui.separator();
            ui.weak(format!("Layout: {}", layout_mode.display_name()));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.weak("Ctrl+1/2/3: Switch accounts | Ctrl+, Settings | F1 Help");
            });
        });
        ui.add_space(2.0);
    }
}