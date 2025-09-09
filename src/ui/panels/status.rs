use eframe::egui;
use crate::ui::{LayoutMode, ResponsiveLayout};

pub struct StatusPanel;

impl StatusPanel {
    pub fn render(ui: &mut egui::Ui, layout_mode: LayoutMode, status_message: &str) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = ResponsiveLayout::PANEL_SPACING * 3.0;
            
            // Status message
            ui.colored_label(ui.visuals().weak_text_color(), status_message);
            
            // Essential shortcuts only
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let shortcuts = match layout_mode {
                    LayoutMode::ThreePane => "Ctrl+F Search • Alt+S Settings",
                    LayoutMode::TwoPane => "Ctrl+F Search • Alt+S Settings", 
                    LayoutMode::CompactPane => "Ctrl+F Search",
                    LayoutMode::MobilePane => "",
                };
                
                if !shortcuts.is_empty() {
                    ui.weak(shortcuts);
                }
            });
        });
    }
    
    pub fn render_minimal(ui: &mut egui::Ui, status_message: &str) {
        // Ultra-minimal status for mobile/compact modes
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = ResponsiveLayout::PANEL_SPACING;
            ui.colored_label(ui.visuals().weak_text_color(), status_message);
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.weak("Ctrl+H Help");
            });
        });
    }
}