use eframe::egui;

pub struct AccountsPanel;

impl AccountsPanel {
    pub fn render(
        ui: &mut egui::Ui, 
        current_account: &mut usize, 
        vim_mode: bool
    ) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.heading("✉️ MailCross");
            ui.separator();
            
            // Mock accounts for now
            let mock_accounts = vec!["Gmail", "Work", "Personal"];
            for (i, account) in mock_accounts.iter().enumerate() {
                let selected = *current_account == i;
                if ui.selectable_label(selected, format!("[{}] {}", i + 1, account)).clicked() {
                    *current_account = i;
                }
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(if vim_mode { "🔥 VIM" } else { "⌨️ NORMAL" });
            });
        });
        ui.add_space(4.0);
    }
}