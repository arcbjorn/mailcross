use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MailCross",
        options,
        Box::new(|_cc| Ok(Box::new(MailCrossApp::default()))),
    )
}

#[derive(Default)]
struct MailCrossApp {
    current_account: usize,
    accounts: Vec<String>,
    vim_mode: bool,
}

impl eframe::App for MailCrossApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel for account switching
        egui::TopBottomPanel::top("accounts").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("MailCross");
                ui.separator();
                
                // Mock accounts for now
                let mock_accounts = vec!["Gmail", "Work", "Personal"];
                for (i, account) in mock_accounts.iter().enumerate() {
                    let selected = self.current_account == i;
                    if ui.selectable_label(selected, format!("[{}] {}", i + 1, account)).clicked() {
                        self.current_account = i;
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(if self.vim_mode { "VIM" } else { "NORMAL" });
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left panel - Folders
                ui.vertical(|ui| {
                    ui.set_width(200.0);
                    ui.heading("Folders");
                    ui.separator();
                    
                    let folders = vec!["INBOX (42)", "Sent", "Drafts", "Spam"];
                    for folder in folders {
                        if ui.selectable_label(false, folder).clicked() {
                            // TODO: Select folder
                        }
                    }
                });

                ui.separator();

                // Middle panel - Email list
                ui.vertical(|ui| {
                    ui.set_width(400.0);
                    ui.heading("Emails");
                    ui.separator();
                    
                    // Mock emails
                    let emails = vec![
                        ("Alice Johnson", "Project Update", "2024-01-15"),
                        ("Bob Smith", "Meeting Tomorrow", "2024-01-14"),
                        ("Newsletter", "Weekly Tech News", "2024-01-14"),
                        ("Charlie Brown", "Re: Budget Review", "2024-01-13"),
                    ];
                    
                    for (sender, subject, date) in emails {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.strong(sender);
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.weak(date);
                                    });
                                });
                                ui.label(subject);
                            });
                        });
                    }
                });

                ui.separator();

                // Right panel - Email content
                ui.vertical(|ui| {
                    ui.heading("Email Preview");
                    ui.separator();
                    
                    ui.label("From: alice@example.com");
                    ui.label("Subject: Project Update");
                    ui.label("Date: 2024-01-15 14:30");
                    ui.separator();
                    
                    ui.label("Hi team,\n\nJust wanted to update you on the project progress...\n\nBest regards,\nAlice");
                });
            });
        });

        // Bottom status bar
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Ready");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("Ctrl+, for settings | F1 for help");
                });
            });
        });

        // Handle keyboard shortcuts
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key { key, pressed: true, modifiers, .. } = event {
                    match (key, modifiers.ctrl) {
                        (egui::Key::Num1, true) => self.current_account = 0,
                        (egui::Key::Num2, true) => self.current_account = 1,
                        (egui::Key::Num3, true) => self.current_account = 2,
                        _ => {}
                    }
                }
            }
        });
    }
}