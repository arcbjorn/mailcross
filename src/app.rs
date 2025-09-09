use eframe::egui;
use crate::types::*;
use crate::ui::*;

#[derive(Default)]
pub struct MailCrossApp {
    // State
    pub current_account: usize,
    pub selected_folder: usize,
    pub selected_email: usize,
    pub vim_mode: bool,
    
    // Data (will be replaced with real data later)
    #[allow(dead_code)] // Will be used when IMAP is implemented
    pub accounts: Vec<Account>,
}

impl MailCrossApp {
    pub fn new() -> Self {
        Self {
            current_account: 0,
            selected_folder: 0,
            selected_email: 0,
            vim_mode: false,
            accounts: vec![
                Account::new("Gmail", "user@gmail.com"),
                Account::new("Work", "user@work.com"),
                Account::new("Personal", "user@personal.com"),
            ],
        }
    }
    
    pub fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key { key, pressed: true, modifiers, .. } = event {
                    match (key, modifiers.ctrl) {
                        (egui::Key::Num1, true) => self.current_account = 0,
                        (egui::Key::Num2, true) => self.current_account = 1,
                        (egui::Key::Num3, true) => self.current_account = 2,
                        // Add more keyboard shortcuts here
                        _ => {}
                    }
                }
            }
        });
    }
}

impl eframe::App for MailCrossApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        self.handle_keyboard_input(ctx);
        
        // Top panel for account switching
        egui::TopBottomPanel::top("accounts")
            .resizable(false)
            .min_height(32.0)
            .show(ctx, |ui| {
                AccountsPanel::render(ui, &mut self.current_account, self.vim_mode);
            });

        // Bottom status bar
        let layout_mode = LayoutMode::from_width(ctx.screen_rect().width());
        egui::TopBottomPanel::bottom("status")
            .resizable(false)
            .min_height(24.0)
            .show(ctx, |ui| {
                StatusPanel::render(ui, layout_mode.clone());
            });

        // Main content area with responsive layout
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let layout_mode = LayoutMode::from_width(available_size.x);
            
            match layout_mode {
                LayoutMode::ThreePane => self.render_three_pane(ui),
                LayoutMode::TwoPane => self.render_two_pane(ui),
                LayoutMode::SinglePane => self.render_single_pane(ui),
            }
        });
    }
}

impl MailCrossApp {
    fn render_three_pane(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Calculate dynamic widths based on available space
            let available_width = ui.available_width();
            let folder_width = ResponsiveLayout::calculate_folder_width(available_width);
            let email_width = ResponsiveLayout::calculate_email_width(available_width);
            
            // Left panel - Folders
            ui.vertical(|ui| {
                ui.set_width(folder_width);
                FoldersPanel::render(ui, &mut self.selected_folder);
            });

            ui.separator();

            // Middle panel - Email list
            ui.vertical(|ui| {
                ui.set_width(email_width);
                EmailsPanel::render(ui, &mut self.selected_email);
            });

            ui.separator();

            // Right panel - Email content (takes remaining space)
            ui.vertical(|ui| {
                PreviewPanel::render(ui, self.selected_email);
            });
        });
    }

    fn render_two_pane(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let available_width = ui.available_width();
            let left_width = ResponsiveLayout::calculate_left_pane_width(available_width);
            
            // Left side - Folders + Emails combined
            ui.vertical(|ui| {
                ui.set_width(left_width);
                
                // Folders section (compact)
                ui.horizontal(|ui| {
                    ui.set_height(80.0);
                    FoldersPanel::render_compact(ui, &mut self.selected_folder);
                });
                
                ui.separator();
                
                // Emails section
                EmailsPanel::render(ui, &mut self.selected_email);
            });

            ui.separator();

            // Right side - Email preview
            ui.vertical(|ui| {
                PreviewPanel::render(ui, self.selected_email);
            });
        });
    }

    fn render_single_pane(&mut self, ui: &mut egui::Ui) {
        // Stack everything vertically for narrow screens
        ui.vertical(|ui| {
            // Top: Folders (horizontal strip)
            ui.horizontal(|ui| {
                ui.set_height(60.0);
                FoldersPanel::render_compact(ui, &mut self.selected_folder);
            });
            
            ui.separator();
            
            // Middle: Email list (limited height)
            ui.vertical(|ui| {
                ui.set_height(200.0);
                EmailsPanel::render_compact(ui, &mut self.selected_email);
            });
            
            ui.separator();
            
            // Bottom: Email preview (remaining space)
            PreviewPanel::render(ui, self.selected_email);
        });
    }
}