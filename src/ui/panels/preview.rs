use eframe::egui;

pub struct PreviewPanel;

impl PreviewPanel {
    pub fn render(ui: &mut egui::Ui, _selected_email: usize) {
        ui.spacing_mut().item_spacing.y = 2.0;
        
        // Clean email headers
        ui.vertical(|ui| {
            ui.weak("alice@example.com");
            ui.strong("Project Update"); 
            ui.weak("Jan 15, 2024 at 2:30 PM");
        });
        
        ui.add_space(8.0);
        
        // Email content
        let available_height = ui.available_height();
        egui::ScrollArea::vertical()
            .max_height(available_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.label("Hi team,\n\nJust wanted to update you on the project progress. We've made significant strides in the past week:\n\n1. Completed the authentication module\n2. Implemented the new dashboard design\n3. Fixed critical bugs in the email client\n4. Added responsive layout support\n\nNext week we'll focus on:\n- Performance optimizations\n- Mobile responsiveness\n- User testing\n\nPlease let me know if you have any questions or concerns.\n\nBest regards,\nAlice");
            });
    }

    pub fn render_mobile(ui: &mut egui::Ui, _selected_email: usize) {
        ui.spacing_mut().item_spacing.y = 1.0;
        
        // Minimal mobile header
        ui.vertical(|ui| {
            ui.weak("alice@example.com");
            ui.strong("Project Update");
        });
        
        ui.add_space(6.0);
        
        // Condensed content
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.label("Hi team,\n\nProject update - we've made good progress this week. Key achievements:\n\n• Authentication module done\n• New dashboard design\n• Email client bug fixes\n• Responsive layout\n\nNext: performance & mobile work.\n\nLet me know if questions.\n\nBest,\nAlice");
            });
    }
}