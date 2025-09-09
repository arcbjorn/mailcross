use eframe::egui;

pub struct EmailsPanel;

impl EmailsPanel {
    pub fn render(ui: &mut egui::Ui, selected_email: &mut usize) {
        ui.heading("📨 Emails");
        ui.separator();
        
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Self::render_email_list(ui, selected_email);
            });
    }
    
    pub fn render_compact(ui: &mut egui::Ui, selected_email: &mut usize) {
        Self::render_email_list(ui, selected_email);
    }
    
    fn render_email_list(ui: &mut egui::Ui, selected_email: &mut usize) {
        let emails = vec![
            ("Alice Johnson", "Project Update", "2024-01-15"),
            ("Bob Smith", "Meeting Tomorrow", "2024-01-14"),
            ("Newsletter", "Weekly Tech News", "2024-01-14"),
            ("Charlie Brown", "Re: Budget Review", "2024-01-13"),
            ("Diana Prince", "Quarterly Report", "2024-01-12"),
            ("Evan Davis", "Lunch Plans", "2024-01-11"),
        ];
        
        for (i, (sender, subject, _date)) in emails.iter().enumerate() {
            let selected = *selected_email == i;
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let email_text = format!("{} - {}", sender, subject);
                    if ui.selectable_label(selected, email_text).clicked() {
                        *selected_email = i;
                    }
                });
            });
            
            ui.add_space(2.0);
        }
    }
}