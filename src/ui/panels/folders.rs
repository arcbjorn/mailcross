use eframe::egui;
use crate::types::Folder;
use crate::ui::ResponsiveLayout;

pub struct FoldersPanel;

impl FoldersPanel {
    pub fn render(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.y = 2.0; // Tighter spacing
        
        let folders = vec![
            Folder::new("INBOX", "📥", 42),
            Folder::new("Sent", "📤", 0),
            Folder::new("Drafts", "📝", 0),
            Folder::new("Spam", "🗑️", 0),
        ];
        
        for (i, folder) in folders.iter().enumerate() {
            let selected = *selected_folder == i;
            
            ui.horizontal(|ui| {
                let response = ui.selectable_label(selected, folder.display_name());
                if response.clicked() {
                    *selected_folder = i;
                }
            });
        }
    }
    
    pub fn render_compact(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.x = 6.0;
        
        ui.horizontal(|ui| {
            let folders = [
                ("📥 INBOX", 42),
                ("📤 Sent", 0), 
                ("📝 Drafts", 0),
                ("🗑️ Spam", 0),
            ];
            
            for (i, (folder_name, _count)) in folders.iter().enumerate() {
                let selected = *selected_folder == i;
                
                let style = if selected {
                    ui.style().visuals.widgets.active
                } else {
                    ui.style().visuals.widgets.inactive
                };
                
                let response = ui.add(
                    egui::Button::new(*folder_name)
                        .small()
                        .fill(if selected { style.bg_fill } else { egui::Color32::TRANSPARENT })
                );
                
                if response.clicked() {
                    *selected_folder = i;
                }
            }
        });
    }

    pub fn render_mobile(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.x = 4.0;
        
        ui.horizontal(|ui| {
            let folders = ["📥", "📤", "📝", "🗑️"];
            
            for (i, folder_icon) in folders.iter().enumerate() {
                let selected = *selected_folder == i;
                
                let response = ui.add(
                    egui::Button::new(*folder_icon)
                        .small()
                        .fill(if selected { 
                            ui.visuals().widgets.active.bg_fill 
                        } else { 
                            egui::Color32::TRANSPARENT 
                        })
                );
                
                if response.clicked() {
                    *selected_folder = i;
                }
            }
        });
    }
}