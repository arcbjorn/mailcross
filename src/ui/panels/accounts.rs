use eframe::egui;
use crate::types::Account;

pub struct AccountsPanel;

impl AccountsPanel {
    pub fn render(
        ui: &mut egui::Ui, 
        current_account: &mut usize, 
        vim_mode: bool,
        accounts: &[&Account]
    ) {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.heading("✉️ MailCross");
            ui.separator();
            
            // Render actual accounts
            for (i, account) in accounts.iter().enumerate() {
                let selected = *current_account == i;
                let label = format!("[{}] {} {}", i + 1, account.name, account.connection_status());
                if ui.selectable_label(selected, label).clicked() {
                    *current_account = i;
                }
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if vim_mode {
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "🔥 VIM");
                } else {
                    ui.label("⌨️ NORMAL");
                }
            });
        });
        ui.add_space(4.0);
    }
}