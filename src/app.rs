use eframe::egui;
use crate::types::*;
use crate::ui::*;
use crate::backend::{AccountManager, AccountEvent};
use crate::input::{KeyboardHandler, KeyAction, VimState, VimCommand};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct DeleteConfirmation {
    #[allow(dead_code)] // Will be used for actual email deletion
    pub email_id: usize,
    pub email_subject: String,
}

pub struct MailCrossApp {
    // State
    pub current_account: usize,
    pub selected_folder: usize,
    pub selected_email: usize,
    
    // Input handling
    pub keyboard_handler: KeyboardHandler,
    pub vim_state: VimState,
    
    // Backend
    pub account_manager: AccountManager,
    pub event_receiver: Option<mpsc::UnboundedReceiver<AccountEvent>>,
    
    // UI State
    pub status_message: String,
    pub show_help: bool,
    pub help_vim_mode: bool,
    pub composer: ComposerWindow,
    pub delete_confirmation: Option<DeleteConfirmation>,
    pub search_state: SearchState,
    pub settings: SettingsWindow,
}

impl MailCrossApp {
    pub fn new() -> Self {
        let mut account_manager = AccountManager::new();
        
        // Create communication channels for events
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        account_manager.set_event_sender(event_sender);
        
        // Add default accounts
        account_manager.add_account(Account::new("Gmail", "user@gmail.com"));
        account_manager.add_account(Account::new("Work", "user@work.com"));
        account_manager.add_account(Account::new("Personal", "user@personal.com"));
        
        Self {
            current_account: 0,
            selected_folder: 0,
            selected_email: 0,
            keyboard_handler: KeyboardHandler::new(),
            vim_state: VimState::new(),
            account_manager,
            event_receiver: Some(event_receiver),
            status_message: "Ready".to_string(),
            show_help: false,
            help_vim_mode: false,
            composer: ComposerWindow::new(),
            delete_confirmation: None,
            search_state: SearchState::new(),
            settings: SettingsWindow::new(),
        }
    }
    
    pub fn get_accounts(&self) -> Vec<&Account> {
        self.account_manager.get_accounts()
    }
    
    #[allow(dead_code)] // Will be used when email data is available
    fn get_current_email(&self) -> Option<Email> {
        // Mock email for testing compose functionality
        Some(Email {
            id: 1,
            sender: "sender@example.com".to_string(),
            recipient: "recipient@example.com".to_string(),
            subject: "Test Email".to_string(),
            body: "This is a test email body.".to_string(),
            date: "2024-01-01".to_string(),
            is_read: false,
            is_selected: false,
        })
    }

    fn get_current_emails(&self) -> Vec<Email> {
        // Mock emails for testing search functionality
        vec![
            Email {
                id: 1,
                sender: "alice@example.com".to_string(),
                recipient: "me@example.com".to_string(),
                subject: "Meeting tomorrow".to_string(),
                body: "Don't forget about our meeting tomorrow at 10am.".to_string(),
                date: "2024-01-15".to_string(),
                is_read: false,
                is_selected: false,
            },
            Email {
                id: 2,
                sender: "bob@company.com".to_string(),
                recipient: "me@example.com".to_string(),
                subject: "Project update".to_string(),
                body: "The project is progressing well. Here's the latest update.".to_string(),
                date: "2024-01-14".to_string(),
                is_read: true,
                is_selected: false,
            },
            Email {
                id: 3,
                sender: "newsletter@tech.com".to_string(),
                recipient: "me@example.com".to_string(),
                subject: "Weekly Tech News".to_string(),
                body: "This week in technology: AI advances, new frameworks, and more.".to_string(),
                date: "2024-01-13".to_string(),
                is_read: false,
                is_selected: false,
            },
        ]
    }
    
    fn process_events(&mut self) {
        if let Some(receiver) = &mut self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                match event {
                    AccountEvent::Connected(email) => {
                        self.status_message = format!("Connected to {}", email);
                    }
                    AccountEvent::Disconnected(email) => {
                        self.status_message = format!("Disconnected from {}", email);
                    }
                    AccountEvent::ConnectionFailed(email, error) => {
                        self.status_message = format!("Failed to connect to {}: {}", email, error);
                    }
                    AccountEvent::FoldersUpdated(email, _folders) => {
                        self.status_message = format!("Folders updated for {}", email);
                    }
                    AccountEvent::EmailsUpdated(email, folder, _emails) => {
                        self.status_message = format!("Emails updated for {}/{}", email, folder);
                    }
                    AccountEvent::EmailDeleted(email, email_id) => {
                        self.status_message = format!("Deleted email {} from {}", email_id, email);
                    }
                }
            }
        }
    }

    pub fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        // Update keyboard handler with current vim mode state
        self.keyboard_handler.vim_mode = self.vim_state.mode != crate::input::vim::VimMode::Normal || self.keyboard_handler.vim_mode;
        
        if let Some(action) = self.keyboard_handler.handle_input(ctx) {
            self.handle_key_action(action);
        }
        
        // Handle vim command mode input separately
        self.handle_vim_command_input(ctx);
    }

    fn handle_key_action(&mut self, action: KeyAction) {
        let accounts_len = self.get_accounts().len();
        
        match action {
            // Account switching
            KeyAction::SwitchAccount(idx) if idx < accounts_len => {
                self.current_account = idx;
                self.status_message = format!("Switched to account {}", idx + 1);
            }
            
            // Navigation
            KeyAction::NextItem => {
                // Navigate to next email
                self.selected_email = (self.selected_email + 1).min(10); // Mock limit
                self.status_message = format!("Email {}", self.selected_email + 1);
            }
            KeyAction::PrevItem => {
                self.selected_email = self.selected_email.saturating_sub(1);
                self.status_message = format!("Email {}", self.selected_email + 1);
            }
            KeyAction::NextPanel => {
                // Cycle through panels (folders -> emails -> preview)
                self.status_message = "Next panel".to_string();
            }
            KeyAction::PrevPanel => {
                self.status_message = "Previous panel".to_string();
            }
            KeyAction::FirstItem => {
                self.selected_email = 0;
                self.status_message = "First email".to_string();
            }
            KeyAction::LastItem => {
                self.selected_email = 10; // Mock limit
                self.status_message = "Last email".to_string();
            }
            
            // Email operations  
            KeyAction::Compose => {
                self.composer.show_compose(self.current_account);
                self.status_message = "Compose new email".to_string();
            }
            KeyAction::Reply => {
                // Get current email (mock for now)
                if let Some(email) = self.get_current_email() {
                    self.composer.show_reply(&email, self.current_account);
                    self.status_message = "Reply to email".to_string();
                }
            }
            KeyAction::Forward => {
                // Get current email (mock for now)
                if let Some(email) = self.get_current_email() {
                    self.composer.show_forward(&email, self.current_account);
                    self.status_message = "Forward email".to_string();
                }
            }
            KeyAction::Delete => {
                if let Some(email) = self.get_current_email() {
                    self.show_delete_confirmation(email.id);
                } else {
                    self.status_message = "No email selected to delete".to_string();
                }
            }
            
            // Search
            KeyAction::SearchCurrentEmail => {
                if !self.search_state.active {
                    self.search_state.start_search_current_email();
                    self.keyboard_handler.set_search_mode(true);
                    self.status_message = "Search current email (Ctrl+F)".to_string();
                } else {
                    self.search_state.cancel_search();
                    self.keyboard_handler.set_search_mode(false);
                    self.status_message = "Search cancelled".to_string();
                }
            }
            KeyAction::SearchAllEmails => {
                if !self.search_state.active {
                    self.search_state.start_search_all_emails();
                    self.keyboard_handler.set_search_mode(true);
                    self.status_message = "Search all emails (Ctrl+Shift+F)".to_string();
                } else {
                    self.search_state.cancel_search();
                    self.keyboard_handler.set_search_mode(false);
                    self.status_message = "Search cancelled".to_string();
                }
            }
            KeyAction::SearchNext => {
                if self.search_state.active && self.search_state.has_results() {
                    self.search_state.next_result();
                    self.status_message = format!("Search result {}/{}", 
                        self.search_state.selected_result + 1, 
                        self.search_state.result_count());
                } else {
                    self.status_message = "No search results".to_string();
                }
            }
            KeyAction::SearchPrev => {
                if self.search_state.active && self.search_state.has_results() {
                    self.search_state.prev_result();
                    self.status_message = format!("Search result {}/{}", 
                        self.search_state.selected_result + 1, 
                        self.search_state.result_count());
                } else {
                    self.status_message = "No search results".to_string();
                }
            }
            
            // View operations
            KeyAction::RefreshFolder => {
                self.status_message = "Refreshing folder...".to_string();
            }
            
            // Settings and help
            KeyAction::Settings => {
                self.settings.update_vim_mode(self.keyboard_handler.vim_mode);
                self.settings.show();
                self.status_message = "Opening settings".to_string();
            }
            KeyAction::Help => {
                self.show_help = true;
                self.help_vim_mode = self.keyboard_handler.vim_mode;
            }
            KeyAction::ToggleVimMode => {
                self.keyboard_handler.toggle_vim_mode();
                self.vim_state.reset();
                let mode_name = if self.keyboard_handler.vim_mode { "Vim" } else { "Normal" };
                self.status_message = format!("Switched to {} mode", mode_name);
            }
            
            // Special actions
            KeyAction::Cancel => {
                self.show_help = false;
                if self.search_state.active {
                    self.search_state.cancel_search();
                }
                self.keyboard_handler.set_search_mode(false);
                self.vim_state.reset();
                self.status_message = "Cancelled".to_string();
            }
            KeyAction::Confirm => {
                self.status_message = "Confirmed".to_string();
            }
            KeyAction::EnterCommandMode => {
                if self.keyboard_handler.vim_mode {
                    self.vim_state.enter_command_mode();
                    self.status_message = "Command mode".to_string();
                }
            }
            
            _ => {
                // Log unhandled actions for debugging
                self.status_message = format!("Action: {:?}", action);
            }
        }
    }

    fn handle_vim_command_input(&mut self, ctx: &egui::Context) {
        if self.vim_state.mode != crate::input::vim::VimMode::Command {
            return;
        }

        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key { key: egui::Key::Enter, pressed: true, .. } => {
                        // Execute command
                        let command = self.vim_state.command_buffer.clone();
                        if let Some(vim_cmd) = self.vim_state.process_command(&command) {
                            self.handle_vim_command(vim_cmd);
                        } else {
                            self.status_message = format!("Unknown command: {}", command);
                        }
                        self.vim_state.reset();
                    }
                    egui::Event::Key { key: egui::Key::Escape, pressed: true, .. } => {
                        self.vim_state.reset();
                        self.status_message = "Command cancelled".to_string();
                    }
                    egui::Event::Key { key: egui::Key::Backspace, pressed: true, .. } => {
                        if self.vim_state.command_buffer.len() > 1 { // Keep the ':'
                            self.vim_state.command_buffer.pop();
                        }
                    }
                    egui::Event::Text(text) => {
                        self.vim_state.command_buffer.push_str(text);
                    }
                    _ => {}
                }
            }
        });
    }

    fn handle_vim_command(&mut self, command: VimCommand) {
        match command {
            VimCommand::Quit => {
                self.status_message = "Quit command (would close app)".to_string();
            }
            VimCommand::Save => {
                self.status_message = "Save command".to_string();
            }
            VimCommand::SaveQuit => {
                self.status_message = "Save and quit".to_string();
            }
            VimCommand::EnableVimMode => {
                self.keyboard_handler.vim_mode = true;
                self.status_message = "Vim mode enabled".to_string();
            }
            VimCommand::DisableVimMode => {
                self.keyboard_handler.vim_mode = false;
                self.vim_state.reset();
                self.status_message = "Vim mode disabled".to_string();
            }
            VimCommand::Help => {
                self.show_help = true;
                self.help_vim_mode = true;
            }
            VimCommand::Set(setting) => {
                self.status_message = format!("Set: {}", setting);
            }
        }
    }

    fn handle_composer_action(&mut self, action: ComposerAction) {
        match action {
            ComposerAction::Send => {
                let account_email = self.get_accounts()[self.composer.from_account].email.clone();
                self.status_message = format!("Sending email from {}", account_email);
                self.composer.visible = false;
            }
            ComposerAction::Save => {
                self.status_message = "Email saved as draft".to_string();
                self.composer.visible = false;
            }
            ComposerAction::Cancel => {
                self.status_message = "Email composition cancelled".to_string();
                self.composer.visible = false;
            }
        }
    }

    fn show_delete_confirmation(&mut self, email_id: usize) {
        if let Some(email) = self.get_current_email() {
            self.delete_confirmation = Some(DeleteConfirmation {
                email_id,
                email_subject: email.subject.clone(),
            });
            self.status_message = "Confirm deletion".to_string();
        }
    }

    fn handle_delete_confirmation(&mut self, confirmed: bool) {
        if let Some(confirmation) = &self.delete_confirmation {
            if confirmed {
                // Delete email through backend
                self.account_manager.delete_email_by_index(self.current_account, confirmation.email_id);
                self.status_message = format!("Deleting email: {}", confirmation.email_subject);
            } else {
                self.status_message = "Deletion cancelled".to_string();
            }
        }
        self.delete_confirmation = None;
    }

    fn handle_settings_action(&mut self, action: SettingsAction) {
        match action {
            SettingsAction::Apply => {
                // Apply settings changes
                self.keyboard_handler.vim_mode = self.settings.get_vim_mode();
                if !self.settings.get_vim_mode() {
                    self.vim_state.reset();
                }
                self.settings.hide();
                self.status_message = "Settings applied".to_string();
            }
            SettingsAction::Cancel => {
                // Restore previous settings
                self.settings.update_vim_mode(self.keyboard_handler.vim_mode);
                self.settings.hide();
                self.status_message = "Settings cancelled".to_string();
            }
            SettingsAction::Reset => {
                self.settings.reset_to_defaults();
                self.status_message = "Settings reset to defaults".to_string();
            }
            SettingsAction::VimModeToggled(enabled) => {
                // Immediate feedback for vim mode toggle
                self.keyboard_handler.vim_mode = enabled;
                if !enabled {
                    self.vim_state.reset();
                }
                let mode_name = if enabled { "Vim" } else { "Traditional" };
                self.status_message = format!("Switched to {} mode", mode_name);
            }
            SettingsAction::ThemeChanged(theme) => {
                self.status_message = format!("Theme changed to {}", theme.display_name());
                // TODO: Apply theme changes to egui context
            }
        }
    }

}

impl eframe::App for MailCrossApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Process backend events
        self.process_events();
        
        // Handle keyboard input
        self.handle_keyboard_input(ctx);
        
        // Handle search input and updates
        if self.search_state.active {
            let emails = self.get_current_emails();
            self.search_state.perform_search(&emails);
        }
        
        // Handle composer window
        let accounts: Vec<&Account> = self.account_manager.get_accounts();
        if let Some(action) = self.composer.render(ctx, &accounts) {
            self.handle_composer_action(action);
        }

        // Handle settings window
        if let Some(action) = self.settings.render(ctx) {
            self.handle_settings_action(action);
        }
        
        // Top panel for account switching
        egui::TopBottomPanel::top("accounts")
            .resizable(false)
            .min_height(32.0)
            .show(ctx, |ui| {
                let accounts = self.get_accounts();
                let mut current_account = self.current_account;
                let vim_mode = self.keyboard_handler.vim_mode;
                AccountsPanel::render(ui, &mut current_account, vim_mode, &accounts);
                self.current_account = current_account;
            });

        // Search bar (if active)
        if self.search_state.active {
            egui::TopBottomPanel::bottom("search")
                .resizable(false)
                .min_height(32.0)
                .show(ctx, |ui| {
                    SearchPanel::render_search_bar(ui, &mut self.search_state);
                });
        }

        // Bottom status bar with responsive height
        let layout_mode = LayoutMode::from_width(ctx.screen_rect().width());
        let status_height = match layout_mode {
            LayoutMode::ThreePane => 28.0,
            LayoutMode::TwoPane => 24.0,
            LayoutMode::CompactPane => 20.0,
            LayoutMode::MobilePane => 18.0,
        };
        
        egui::TopBottomPanel::bottom("status")
            .resizable(false)
            .min_height(status_height)
            .show(ctx, |ui| {
                match layout_mode {
                    LayoutMode::MobilePane | LayoutMode::CompactPane => {
                        StatusPanel::render_minimal(ui, &self.status_message);
                    }
                    _ => {
                        StatusPanel::render(ui, layout_mode.clone(), &self.status_message);
                    }
                }
            });

        // Main content area with responsive layout
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            let layout_mode = LayoutMode::from_width(available_size.x);
            
            match layout_mode {
                LayoutMode::ThreePane => self.render_three_pane(ui),
                LayoutMode::TwoPane => self.render_two_pane(ui),
                LayoutMode::CompactPane => self.render_compact_pane(ui),
                LayoutMode::MobilePane => self.render_mobile_pane(ui),
            }
            
            // Show vim command buffer if in command mode
            if self.vim_state.mode == crate::input::vim::VimMode::Command {
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(
                        egui::pos2(0.0, available_size.y - 30.0), 
                        egui::vec2(available_size.x, 30.0)
                    ),
                    0.0,
                    egui::Color32::from_gray(40)
                );
                
                let _ = ui.allocate_rect(
                    egui::Rect::from_min_size(
                        egui::pos2(10.0, available_size.y - 25.0), 
                        egui::vec2(available_size.x - 20.0, 20.0)
                    ),
                    egui::Sense::hover()
                );
                ui.painter().text(
                    egui::pos2(15.0, available_size.y - 15.0),
                    egui::Align2::LEFT_CENTER,
                    format!("{} (Press Enter to execute, Esc to cancel)", &self.vim_state.command_buffer),
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE
                );
                
                /*ui.allocate_ui_at_rect(
                    egui::Rect::from_min_size(
                        egui::pos2(10.0, available_size.y - 25.0), 
                        egui::vec2(available_size.x - 20.0, 20.0)
                    ),
                    |ui| {
                        ui.colored_label(
                            egui::Color32::WHITE, 
                            format!("{} (Press Enter to execute, Esc to cancel)", &self.vim_state.command_buffer)
                        );
                    }
                );*/
            }
        });

        // Help dialog - render at top level with context
        if self.show_help {
            let mut close_help = false;
            let mut toggle_vim = false;
            
            egui::Window::new("Keyboard Shortcuts")
                .open(&mut self.show_help)
                .default_width(500.0)
                .default_height(400.0)
                .resizable(true)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.heading(if self.help_vim_mode { "Vim Mode" } else { "Traditional Mode" });
                    ui.separator();
                    
                    egui::ScrollArea::vertical().id_salt("help").show(ui, |ui| {
                        let shortcuts = if self.help_vim_mode {
                            crate::input::VimKeymap::get_help_text()
                        } else {
                            crate::input::VimKeymap::get_traditional_help_text()
                        };

                        ui.columns(2, |columns| {
                            columns[0].heading("Shortcut");
                            columns[1].heading("Action");
                        });
                        
                        ui.separator();
                        
                        for (shortcut, description) in shortcuts {
                            ui.columns(2, |columns| {
                                columns[0].monospace(shortcut);
                                columns[1].label(description);
                            });
                        }
                        
                        if self.help_vim_mode {
                            ui.add_space(20.0);
                            ui.heading("Vim Commands");
                            ui.separator();
                            ui.columns(2, |columns| {
                                columns[0].monospace(":q");
                                columns[1].label("Quit");
                            });
                            ui.columns(2, |columns| {
                                columns[0].monospace(":set novim");
                                columns[1].label("Disable vim mode");
                            });
                            ui.columns(2, |columns| {
                                columns[0].monospace(":help");
                                columns[1].label("Show this help");
                            });
                        }
                        
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui| {
                            let toggle_text = if self.help_vim_mode {
                                "Switch to Traditional Mode"
                            } else {
                                "Switch to Vim Mode (experimental)"
                            };
                            
                            if ui.button(toggle_text).clicked() {
                                toggle_vim = true;
                            }
                            
                            if ui.button("Close").clicked() {
                                close_help = true;
                            }
                        });
                    });
                });
            
            if close_help {
                self.show_help = false;
            }
            
            if toggle_vim {
                self.keyboard_handler.toggle_vim_mode();
                self.vim_state.reset();
                self.settings.update_vim_mode(self.keyboard_handler.vim_mode);
                let mode_name = if self.keyboard_handler.vim_mode { "Vim" } else { "Normal" };
                self.status_message = format!("Switched to {} mode", mode_name);
                self.help_vim_mode = self.keyboard_handler.vim_mode;
            }
        }

        // Delete confirmation dialog
        if let Some(confirmation) = &self.delete_confirmation.clone() {
            let mut should_delete = false;
            let mut should_cancel = false;
            
            egui::Window::new("Delete Email")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("Are you sure you want to delete this email?");
                        ui.add_space(10.0);
                        ui.label(format!("Subject: {}", confirmation.email_subject));
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui| {
                            if ui.button("Delete").clicked() {
                                should_delete = true;
                            }
                            if ui.button("Cancel").clicked() {
                                should_cancel = true;
                            }
                        });
                    });
                });
                
            if should_delete {
                self.handle_delete_confirmation(true);
            } else if should_cancel {
                self.handle_delete_confirmation(false);
            }
        }
    }
}

impl MailCrossApp {
    fn render_three_pane(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = ResponsiveLayout::PANEL_SPACING;
            
            // Calculate optimal widths for fullscreen comfort
            let available_width = ui.available_width();
            let folder_width = ResponsiveLayout::calculate_folder_width_fullscreen(available_width);
            let email_width = ResponsiveLayout::calculate_email_width_fullscreen(available_width);
            
            // Left panel - Folders
            ui.vertical(|ui| {
                ui.set_width(folder_width);
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                FoldersPanel::render(ui, &mut self.selected_folder);
            });

            self.render_minimal_separator(ui);

            // Middle panel - Email list  
            ui.vertical(|ui| {
                ui.set_width(email_width);
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                EmailsPanel::render_with_search(ui, &mut self.selected_email, &self.search_state);
            });

            self.render_minimal_separator(ui);

            // Right panel - Email content (remaining space)
            ui.vertical(|ui| {
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                PreviewPanel::render(ui, self.selected_email);
            });
        });
    }

    fn render_two_pane(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = ResponsiveLayout::PANEL_SPACING;
            
            let available_width = ui.available_width();
            let left_width = ResponsiveLayout::calculate_left_pane_width_half(available_width);
            
            // Left side - Folders + Emails combined
            ui.vertical(|ui| {
                ui.set_width(left_width);
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                
                // Folders section (compact horizontal strip)
                ui.horizontal(|ui| {
                    ui.set_height(80.0);
                    FoldersPanel::render_compact(ui, &mut self.selected_folder);
                });
                
                ui.add_space(ResponsiveLayout::PANEL_SPACING);
                self.render_horizontal_separator(ui);
                ui.add_space(ResponsiveLayout::PANEL_SPACING);
                
                // Emails section
                EmailsPanel::render_with_search(ui, &mut self.selected_email, &self.search_state);
            });

            self.render_minimal_separator(ui);

            // Right side - Email preview
            ui.vertical(|ui| {
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                PreviewPanel::render(ui, self.selected_email);
            });
        });
    }

    fn render_compact_pane(&mut self, ui: &mut egui::Ui) {
        // Half vertical layout - stack with balanced proportions
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = ResponsiveLayout::PANEL_SPACING;
            let available_height = ui.available_height();
            
            // Top: Folders (horizontal strip)
            ui.horizontal(|ui| {
                ui.set_height(ResponsiveLayout::calculate_folder_height_compact());
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                FoldersPanel::render_compact(ui, &mut self.selected_folder);
            });
            
            self.render_horizontal_separator(ui);
            
            // Middle: Email list (calculated height)
            ui.vertical(|ui| {
                let email_height = ResponsiveLayout::calculate_email_height_compact(available_height);
                ui.set_height(email_height);
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                EmailsPanel::render_with_search(ui, &mut self.selected_email, &self.search_state);
            });
            
            self.render_horizontal_separator(ui);
            
            // Bottom: Email preview (remaining space)
            ui.vertical(|ui| {
                ui.add_space(ResponsiveLayout::INNER_PADDING);
                PreviewPanel::render(ui, self.selected_email);
            });
        });
    }

    fn render_mobile_pane(&mut self, ui: &mut egui::Ui) {
        // Quarter/mobile layout - single column, minimal spacing
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = ResponsiveLayout::PANEL_SPACING;
            
            // Compact folder strip
            ui.horizontal(|ui| {
                ui.set_height(ResponsiveLayout::calculate_mobile_item_height());
                FoldersPanel::render_mobile(ui, &mut self.selected_folder);
            });
            
            // Minimal separator
            ui.add_space(2.0);
            
            // If search is active, show search, otherwise show email list
            if self.search_state.active {
                ui.vertical(|ui| {
                    ui.set_height(ui.available_height() * 0.4);
                    EmailsPanel::render_mobile(ui, &mut self.selected_email, &self.search_state);
                });
                
                ui.add_space(2.0);
                
                // Preview takes remaining space
                PreviewPanel::render_mobile(ui, self.selected_email);
            } else {
                // Focus on email list when not searching
                EmailsPanel::render_mobile_full(ui, &mut self.selected_email);
            }
        });
    }

    // Helper methods for visual consistency
    fn render_minimal_separator(&self, ui: &mut egui::Ui) {
        ui.allocate_space(egui::vec2(ResponsiveLayout::SEPARATOR_WIDTH, ui.available_height()));
        let rect = ui.min_rect();
        ui.painter().rect_filled(
            rect,
            0.0,
            ui.visuals().widgets.noninteractive.bg_stroke.color.gamma_multiply(0.3)
        );
    }

    fn render_horizontal_separator(&self, ui: &mut egui::Ui) {
        ui.allocate_space(egui::vec2(ui.available_width(), ResponsiveLayout::SEPARATOR_WIDTH));
        let rect = ui.min_rect();
        ui.painter().rect_filled(
            rect,
            0.0,
            ui.visuals().widgets.noninteractive.bg_stroke.color.gamma_multiply(0.3)
        );
    }
}