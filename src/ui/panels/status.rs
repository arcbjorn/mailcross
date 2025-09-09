use eframe::egui;
use crate::ui::{LayoutMode, ResponsiveLayout};

pub struct StatusPanel;

impl StatusPanel {
    pub fn render(ui: &mut egui::Ui, layout_mode: LayoutMode, status_message: &str) {
        // Minimal status bar with responsive shortcuts
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = ResponsiveLayout::PANEL_SPACING * 2.0;
            
            // Status message with subtle styling
            ui.colored_label(ui.visuals().weak_text_color(), status_message);
            
            // Layout indicator (subtle)
            ui.weak("•");
            ui.weak(layout_mode.display_name());
            
            // Responsive shortcut hints
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let shortcuts = match layout_mode {
                    LayoutMode::ThreePane => "Ctrl+F Search • Alt+S Settings • Ctrl+1/2/3 Accounts",
                    LayoutMode::TwoPane => "Ctrl+F Search • Alt+S Settings",
                    LayoutMode::CompactPane => "Ctrl+F Search • Ctrl+H Help",
                    LayoutMode::MobilePane => "Ctrl+F Search",
                };
                ui.weak(shortcuts);
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