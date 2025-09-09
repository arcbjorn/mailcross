use eframe::egui;
use crate::types::Folder;
use crate::ui::ResponsiveLayout;

pub struct FoldersPanel;

impl FoldersPanel {
    pub fn render(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.y = ResponsiveLayout::PANEL_SPACING;
        
        // Minimal header
        ui.horizontal(|ui| {
            ui.weak("📁");
            ui.weak("Folders");
        });
        
        ui.add_space(ResponsiveLayout::INNER_PADDING);
        
        let folders = vec![
            Folder::new("INBOX", "📥", 42),
            Folder::new("Sent", "📤", 0),
            Folder::new("Drafts", "📝", 0),
            Folder::new("Spam", "🗑️", 0),
        ];
        
        for (i, folder) in folders.iter().enumerate() {
            let selected = *selected_folder == i;
            let response = ui.selectable_label(selected, folder.display_name());
            
            if response.clicked() {
                *selected_folder = i;
            }
        }
    }
    
    pub fn render_compact(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.x = ResponsiveLayout::PANEL_SPACING;
        
        ui.horizontal(|ui| {
            ui.weak("📁");
            
            let folders = ["INBOX", "Sent", "Drafts", "Spam"];
            for (i, folder) in folders.iter().enumerate() {
                let selected = *selected_folder == i;
                let label = if selected { format!("● {}", folder) } else { folder.to_string() };
                
                if ui.small_button(label).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }

    pub fn render_mobile(ui: &mut egui::Ui, selected_folder: &mut usize) {
        // Ultra-compact horizontal strip for mobile
        ui.horizontal(|ui| {
            let folders = ["INBOX", "Sent", "Drafts", "Spam"];
            for (i, folder) in folders.iter().enumerate() {
                let selected = *selected_folder == i;
                if ui.small_button(if selected { format!("● {}", folder) } else { folder.to_string() }).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }
}