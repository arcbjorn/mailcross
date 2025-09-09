use eframe::egui;

pub struct PreviewPanel;

impl PreviewPanel {
    pub fn render(ui: &mut egui::Ui, _selected_email: usize) {
        ui.heading("📧 Email Preview");
        ui.separator();
        
        // Email headers - fixed section
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.label("From: alice@example.com");
                ui.label("Subject: Project Update");
                ui.label("Date: 2024-01-15 14:30");
            });
        });
        
        ui.separator();
        
        // Email content - scrollable section
        let available_height = ui.available_height();
        egui::ScrollArea::vertical()
            .max_height(available_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.label("Hi team,\n\nJust wanted to update you on the project progress. We've made significant strides in the past week:\n\n1. Completed the authentication module\n2. Implemented the new dashboard design\n3. Fixed critical bugs in the email client\n4. Added responsive layout support\n\nNext week we'll focus on:\n- Performance optimizations\n- Mobile responsiveness\n- User testing\n\nPlease let me know if you have any questions or concerns.\n\nBest regards,\nAlice");
            });
    }
}