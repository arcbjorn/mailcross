use eframe::egui;

pub struct FoldersPanel;

impl FoldersPanel {
    // Shared folder data - consistent across all layouts
    const FOLDERS: &'static [(&'static str, &'static str, u32)] = &[
        ("INBOX", "📥", 42),
        ("Sent", "📤", 0),
        ("Drafts", "📝", 0),
        ("Spam", "🗑️", 0),
    ];

    // Fullscreen/wide layout - vertical list
    pub fn render(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.y = 1.0;
        
        for (i, (name, icon, count)) in Self::FOLDERS.iter().enumerate() {
            let selected = *selected_folder == i;
            let label = if *count > 0 {
                format!("{} {} ({})", icon, name, count)
            } else {
                format!("{} {}", icon, name)
            };
            
            if ui.selectable_label(selected, label).clicked() {
                *selected_folder = i;
            }
        }
    }
    
    // Compact horizontal layout
    pub fn render_compact(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.x = 3.0;
        
        ui.horizontal(|ui| {
            for (i, (name, icon, count)) in Self::FOLDERS.iter().enumerate() {
                let selected = *selected_folder == i;
                let label = if *count > 0 {
                    format!("{} {} ({})", icon, name, count)
                } else {
                    format!("{} {}", icon, name)
                };
                
                if ui.selectable_label(selected, label).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }

    // Mobile - icons only
    pub fn render_mobile(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.x = 2.0;
        
        ui.horizontal(|ui| {
            for (i, (_name, icon, count)) in Self::FOLDERS.iter().enumerate() {
                let selected = *selected_folder == i;
                let label = if *count > 0 {
                    format!("{} {}", icon, count)
                } else {
                    icon.to_string()
                };
                
                if ui.selectable_label(selected, label).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }
}