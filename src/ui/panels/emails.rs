use eframe::egui;
use crate::ui::SearchState;

pub struct EmailsPanel;

impl EmailsPanel {
    #[allow(dead_code)] // Used in some layout modes  
    pub fn render(ui: &mut egui::Ui, selected_email: &mut usize) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Self::render_email_list(ui, selected_email);
            });
    }

    pub fn render_with_search(ui: &mut egui::Ui, selected_email: &mut usize, search_state: &SearchState) {
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
    
    #[allow(dead_code)] // Used in some layout modes
    pub fn render_compact(ui: &mut egui::Ui, selected_email: &mut usize) {
        Self::render_email_list(ui, selected_email);
    }
    
    fn render_email_list(ui: &mut egui::Ui, selected_email: &mut usize) {
        ui.spacing_mut().item_spacing.y = 1.0;
        
        let emails = vec![
            ("Alice Johnson", "Project Update", "Jan 15"),
            ("Bob Smith", "Meeting Tomorrow", "Jan 14"), 
            ("Newsletter", "Weekly Tech News", "Jan 14"),
            ("Charlie Brown", "Re: Budget Review", "Jan 13"),
            ("Diana Prince", "Quarterly Report", "Jan 12"),
            ("Evan Davis", "Lunch Plans", "Jan 11"),
        ];
        
        for (i, (sender, subject, date)) in emails.iter().enumerate() {
            let selected = *selected_email == i;
            
            // Compact table-like email row
            ui.horizontal(|ui| {
                let total_width = ui.available_width();
                
                // Sender column (30%)
                ui.allocate_ui_with_layout(
                    [total_width * 0.3, 20.0].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        if ui.selectable_label(selected, *sender).clicked() {
                            *selected_email = i;
                        }
                    }
                );
                
                // Subject column (50%) 
                ui.allocate_ui_with_layout(
                    [total_width * 0.5, 20.0].into(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.weak(*subject);
                    }
                );
                
                // Date column (20%)
                ui.allocate_ui_with_layout(
                    [total_width * 0.2, 20.0].into(),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.weak(*date);
                    }
                );
            });
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

    pub fn render_mobile(ui: &mut egui::Ui, selected_email: &mut usize, _search_state: &SearchState) {
        Self::render_mobile_email_list(ui, selected_email, true);
    }

    pub fn render_mobile_full(ui: &mut egui::Ui, selected_email: &mut usize) {
        Self::render_mobile_email_list(ui, selected_email, false);
    }

    fn render_mobile_email_list(ui: &mut egui::Ui, selected_email: &mut usize, _compact: bool) {
        ui.spacing_mut().item_spacing.y = 1.0;
        
        let emails = [
            ("Alice", "Meeting Tomorrow", "Jan 15"),
            ("Bob", "Project Update", "Jan 14"),
            ("Carol", "Code Review", "Jan 13"), 
            ("David", "Status Report", "Jan 12"),
            ("Evan", "Lunch Plans", "Jan 11"),
        ];

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, (sender, subject, _date)) in emails.iter().enumerate() {
                let selected = *selected_email == i;
                
                if ui.selectable_label(selected, format!("{} · {}", sender, subject)).clicked() {
                    *selected_email = i;
                }
            }
        });
    }
}