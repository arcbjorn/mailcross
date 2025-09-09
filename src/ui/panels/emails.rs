use eframe::egui;
use crate::ui::{SearchState, ResponsiveLayout};

pub struct EmailsPanel;

impl EmailsPanel {
    #[allow(dead_code)] // Used in some layout modes
    pub fn render(ui: &mut egui::Ui, selected_email: &mut usize) {
        ui.spacing_mut().item_spacing.y = ResponsiveLayout::PANEL_SPACING;
        
        // Minimal header
        ui.horizontal(|ui| {
            ui.weak("📧");
            ui.weak("Emails");
        });
        
        ui.add_space(ResponsiveLayout::INNER_PADDING);
        
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Self::render_email_list(ui, selected_email);
            });
    }

    pub fn render_with_search(ui: &mut egui::Ui, selected_email: &mut usize, search_state: &SearchState) {
        ui.spacing_mut().item_spacing.y = ResponsiveLayout::PANEL_SPACING;
        
        if search_state.active {
            // Minimal search header
            ui.horizontal(|ui| {
                ui.weak("🔍");
                if search_state.has_results() {
                    ui.weak(format!("{} in {}", search_state.result_count(), search_state.get_scope_display()));
                } else if !search_state.query.is_empty() {
                    ui.weak("No results");
                } else {
                    ui.weak(format!("Search {}", search_state.get_scope_display()));
                }
            });
        } else {
            // Standard email header
            ui.horizontal(|ui| {
                ui.weak("📧");
                ui.weak("Emails");
            });
        }
        
        ui.add_space(ResponsiveLayout::INNER_PADDING);
        
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
        let emails = vec![
            ("Alice Johnson", "Project Update", "2024-01-15"),
            ("Bob Smith", "Meeting Tomorrow", "2024-01-14"),
            ("Newsletter", "Weekly Tech News", "2024-01-14"),
            ("Charlie Brown", "Re: Budget Review", "2024-01-13"),
            ("Diana Prince", "Quarterly Report", "2024-01-12"),
            ("Evan Davis", "Lunch Plans", "2024-01-11"),
        ];
        
        for (i, (sender, subject, date)) in emails.iter().enumerate() {
            let selected = *selected_email == i;
            
            // Clean email item layout
            ui.vertical(|ui| {
                let sender_response = ui.selectable_label(selected, *sender);
                ui.weak(*subject);
                ui.weak(*date);
                
                if sender_response.clicked() {
                    *selected_email = i;
                }
            });
            
            ui.add_space(ResponsiveLayout::PANEL_SPACING);
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

    pub fn render_mobile(ui: &mut egui::Ui, selected_email: &mut usize, search_state: &SearchState) {
        // Mobile layout with search awareness
        if search_state.active && search_state.has_results() {
            ui.label(format!("🔍 {} results in {}", 
                search_state.result_count(), 
                search_state.get_scope_display()));
            ui.separator();
            Self::render_mobile_email_list(ui, selected_email, true);
        } else {
            ui.label("📧 Emails");
            ui.separator();
            Self::render_mobile_email_list(ui, selected_email, false);
        }
    }

    pub fn render_mobile_full(ui: &mut egui::Ui, selected_email: &mut usize) {
        // Full-height mobile email list
        ui.label("📧 Emails");
        ui.separator();
        Self::render_mobile_email_list(ui, selected_email, false);
    }

    fn render_mobile_email_list(ui: &mut egui::Ui, selected_email: &mut usize, _compact: bool) {
        // Ultra-compact email list for mobile
        let emails = [
            ("Alice Smith", "Meeting Tomorrow", "2024-01-15"),
            ("Bob Johnson", "Project Update", "2024-01-14"),
            ("Carol Williams", "Code Review", "2024-01-13"),
            ("David Brown", "Status Report", "2024-01-12"),
            ("Evan Davis", "Lunch Plans", "2024-01-11"),
        ];

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, (sender, subject, date)) in emails.iter().enumerate() {
                let selected = *selected_email == i;
                
                ui.horizontal(|ui| {
                    if ui.selectable_label(selected, format!("📧 {}", sender)).clicked() {
                        *selected_email = i;
                    }
                });
                
                if selected {
                    ui.weak(format!("   {}", subject));
                    ui.weak(format!("   {}", date));
                }
                
                ui.add_space(1.0);
            }
        });
    }
}