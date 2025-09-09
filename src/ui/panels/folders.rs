use eframe::egui;

pub struct FoldersPanel;

impl FoldersPanel {
    const FOLDERS: &'static [(&'static str, &'static str, u32)] = &[
        ("📥", "INBOX", 42),
        ("📤", "Sent", 0), 
        ("📝", "Drafts", 0),
        ("🗑", "Spam", 0),
    ];

    // Vertical layout with text - properly sized
    pub fn render(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.spacing_mut().item_spacing.y = 0.5;
        
        for (i, (icon, name, count)) in Self::FOLDERS.iter().enumerate() {
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
    
    // Horizontal compact layout with text
    pub fn render_compact(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            
            for (i, (icon, name, count)) in Self::FOLDERS.iter().enumerate() {
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

    // Mobile with short text
    pub fn render_mobile(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            
            for (i, (icon, name, count)) in Self::FOLDERS.iter().enumerate() {
                let selected = *selected_folder == i;
                let label = if *count > 0 {
                    format!("{} {} {}", icon, name, count)
                } else {
                    format!("{} {}", icon, name)
                };
                
                if ui.selectable_label(selected, label).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }
}