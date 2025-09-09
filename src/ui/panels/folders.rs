use eframe::egui;
use crate::types::Folder;

pub struct FoldersPanel;

impl FoldersPanel {
    pub fn render(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.heading("📁 Folders");
        ui.separator();
        
        let folders = vec![
            Folder::new("INBOX", "📥", 42),
            Folder::new("Sent", "📤", 0),
            Folder::new("Drafts", "📝", 0),
            Folder::new("Spam", "🗑️", 0),
        ];
        
        for (i, folder) in folders.iter().enumerate() {
            let selected = *selected_folder == i;
            if ui.selectable_label(selected, folder.display_name()).clicked() {
                *selected_folder = i;
            }
        }
    }
    
    pub fn render_compact(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.label("📁");
        ui.separator();
        
        let folders = vec!["INBOX", "Sent", "Drafts", "Spam"];
        for (i, folder) in folders.iter().enumerate() {
            let selected = *selected_folder == i;
            if ui.selectable_label(selected, *folder).clicked() {
                *selected_folder = i;
            }
        }
    }
}