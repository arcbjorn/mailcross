use eframe::egui;

pub struct FoldersPanel;

impl FoldersPanel {
    const FOLDERS: &'static [(&'static str, &'static str)] = &[
        ("📥", "INBOX"),
        ("📤", "Sent"), 
        ("📝", "Drafts"),
        ("🗑️", "Spam"),
    ];

    // All layouts now horizontal and ultra-compact
    pub fn render(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            
            for (i, (icon, name)) in Self::FOLDERS.iter().enumerate() {
                let selected = *selected_folder == i;
                let label = if i == 0 { format!("{} 42", icon) } else { icon.to_string() };
                
                if ui.small_button(label).clicked() && !selected {
                    *selected_folder = i;
                }
                
                // Show selected folder name next to buttons
                if selected {
                    ui.weak(*name);
                }
            }
        });
    }
    
    pub fn render_compact(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 1.0;
            
            for (i, (icon, _name)) in Self::FOLDERS.iter().enumerate() {
                let selected = *selected_folder == i;
                let label = if i == 0 && selected { "📥42" } else { *icon };
                
                if ui.small_button(label).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }

    pub fn render_mobile(ui: &mut egui::Ui, selected_folder: &mut usize) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.5;
            
            for (i, (icon, _name)) in Self::FOLDERS.iter().enumerate() {
                let selected = *selected_folder == i;
                let mut button = egui::Button::new(*icon).small();
                
                if selected {
                    button = button.fill(ui.visuals().selection.bg_fill);
                }
                
                if ui.add(button).clicked() {
                    *selected_folder = i;
                }
            }
        });
    }
}