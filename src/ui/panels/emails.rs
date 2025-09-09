use eframe::egui;
use crate::ui::SearchState;

pub struct EmailsPanel;

impl EmailsPanel {
    #[allow(dead_code)] // Used in some layout modes
    pub fn render(ui: &mut egui::Ui, selected_email: &mut usize) {
        ui.heading("📨 Emails");
        ui.separator();
        
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Self::render_email_list(ui, selected_email);
            });
    }

    pub fn render_with_search(ui: &mut egui::Ui, selected_email: &mut usize, search_state: &SearchState) {
        if search_state.active {
            ui.heading("🔍 Search Results");
            if search_state.has_results() {
                ui.label(format!("{} results found", search_state.result_count()));
            } else if !search_state.query.is_empty() {
                ui.weak("No results found");
            }
        } else {
            ui.heading("📨 Emails");
        }
        
        ui.separator();
        
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if search_state.active && search_state.has_results() {
                    Self::render_search_results(ui, search_state);
                } else {
                    Self::render_email_list(ui, selected_email);
                }
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

    fn render_search_results(ui: &mut egui::Ui, search_state: &SearchState) {
        for (i, email) in search_state.results.iter().enumerate() {
            let selected = i == search_state.selected_result;
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    let email_text = format!("{} - {}", email.sender, email.subject);
                    let response = ui.selectable_label(selected, email_text);
                    
                    if selected {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                });
                
                // Show search context/preview
                if !email.body.is_empty() {
                    ui.weak(format!("  {}", email.body.chars().take(60).collect::<String>()));
                }
            });
            
            ui.add_space(2.0);
        }
    }
}