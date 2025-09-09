use eframe::egui;

#[derive(Debug, Clone)]
pub struct SettingsWindow {
    pub visible: bool,
    pub vim_mode: bool,
    pub theme: AppTheme,
    pub show_status_bar: bool,
    pub show_folder_icons: bool,
    pub auto_refresh: bool,
    pub refresh_interval: u32, // minutes
    pub confirm_delete: bool,
    pub compact_layout: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppTheme {
    Light,
    Dark,
    Auto,
}

impl AppTheme {
    pub fn display_name(&self) -> &'static str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark", 
            AppTheme::Auto => "Auto",
        }
    }
}

#[derive(Debug, Clone)]
pub enum SettingsAction {
    Apply,
    Cancel,
    Reset,
    VimModeToggled(bool),
    ThemeChanged(AppTheme),
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self {
            visible: false,
            vim_mode: false,
            theme: AppTheme::Auto,
            show_status_bar: true,
            show_folder_icons: true,
            auto_refresh: false,
            refresh_interval: 5,
            confirm_delete: true,
            compact_layout: false,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn update_vim_mode(&mut self, vim_mode: bool) {
        self.vim_mode = vim_mode;
    }

    pub fn render(&mut self, ctx: &egui::Context) -> Option<SettingsAction> {
        if !self.visible {
            return None;
        }

        let mut action = None;
        let mut should_close = false;

        egui::Window::new("⚙️ Settings")
            .resizable(true)
            .default_size([400.0, 500.0])
            .open(&mut should_close)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().id_salt("settings").show(ui, |ui| {
                    // Input Mode Settings
                    ui.heading("Input Mode");
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        let old_vim_mode = self.vim_mode;
                        ui.checkbox(&mut self.vim_mode, "Enable Vim mode");
                        ui.weak("(hjkl navigation, : commands)");
                        
                        if old_vim_mode != self.vim_mode {
                            action = Some(SettingsAction::VimModeToggled(self.vim_mode));
                        }
                    });

                    ui.add_space(15.0);

                    // Appearance Settings
                    ui.heading("Appearance");
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Theme:");
                        let old_theme = self.theme.clone();
                        egui::ComboBox::from_id_salt("theme")
                            .selected_text(self.theme.display_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.theme, AppTheme::Light, "Light");
                                ui.selectable_value(&mut self.theme, AppTheme::Dark, "Dark");
                                ui.selectable_value(&mut self.theme, AppTheme::Auto, "Auto");
                            });
                            
                        if old_theme != self.theme {
                            action = Some(SettingsAction::ThemeChanged(self.theme.clone()));
                        }
                    });

                    ui.checkbox(&mut self.show_status_bar, "Show status bar");
                    ui.checkbox(&mut self.show_folder_icons, "Show folder icons");
                    ui.checkbox(&mut self.compact_layout, "Use compact layout");

                    ui.add_space(15.0);

                    // Behavior Settings
                    ui.heading("Behavior");
                    ui.separator();
                    
                    ui.checkbox(&mut self.auto_refresh, "Auto-refresh emails");
                    
                    ui.horizontal(|ui| {
                        ui.add_enabled(
                            self.auto_refresh,
                            egui::Slider::new(&mut self.refresh_interval, 1..=60)
                                .text("minutes")
                                .suffix(" min")
                        );
                    });

                    ui.checkbox(&mut self.confirm_delete, "Confirm before deleting emails");

                    ui.add_space(15.0);

                    // Keyboard Shortcuts Info
                    ui.heading("Keyboard Shortcuts");
                    ui.separator();
                    
                    if self.vim_mode {
                        ui.label("Vim mode shortcuts:");
                        ui.weak("• hjkl - Navigate");
                        ui.weak("• / - Search");
                        ui.weak("• dd - Delete");
                        ui.weak("• c - Compose");
                        ui.weak("• r - Reply");
                        ui.weak("• : - Command mode");
                    } else {
                        ui.label("Traditional shortcuts:");
                        ui.weak("• ↑↓ - Navigate");
                        ui.weak("• Ctrl+F - Search current email");
                        ui.weak("• Ctrl+Shift+F - Search all emails");
                        ui.weak("• Delete - Delete");
                        ui.weak("• Ctrl+N - Compose");
                        ui.weak("• Ctrl+R - Reply");
                        ui.weak("• Ctrl+L - Forward");
                        ui.weak("• Ctrl+H - Help");
                        ui.weak("• Alt+S - Settings");
                    }

                    ui.add_space(20.0);

                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("Apply").clicked() {
                            action = Some(SettingsAction::Apply);
                        }
                        if ui.button("Cancel").clicked() {
                            action = Some(SettingsAction::Cancel);
                        }
                        if ui.button("Reset to Defaults").clicked() {
                            action = Some(SettingsAction::Reset);
                        }
                    });
                });
            });

        // Handle window close button
        if should_close {
            self.visible = false;
        }

        action
    }

    pub fn reset_to_defaults(&mut self) {
        *self = Self::new();
        self.visible = true; // Keep settings window open
    }

    #[allow(dead_code)] // Will be used for settings persistence
    pub fn load_from_config(&mut self) {
        // TODO: Load settings from config file
        // For now, use defaults
    }

    #[allow(dead_code)] // Will be used for settings persistence
    pub fn save_to_config(&self) {
        // TODO: Save settings to config file
        // For now, this is a no-op
    }

    // Getters for settings values
    pub fn get_vim_mode(&self) -> bool {
        self.vim_mode
    }

    #[allow(dead_code)] // Will be used for theme application
    pub fn get_theme(&self) -> &AppTheme {
        &self.theme
    }

    #[allow(dead_code)] // Will be used for UI layout
    pub fn get_show_status_bar(&self) -> bool {
        self.show_status_bar
    }

    #[allow(dead_code)] // Will be used for folder display
    pub fn get_show_folder_icons(&self) -> bool {
        self.show_folder_icons
    }

    #[allow(dead_code)] // Will be used for email refresh
    pub fn get_auto_refresh(&self) -> bool {
        self.auto_refresh
    }

    #[allow(dead_code)] // Will be used for refresh timing
    pub fn get_refresh_interval(&self) -> u32 {
        self.refresh_interval
    }

    #[allow(dead_code)] // Will be used for delete behavior
    pub fn get_confirm_delete(&self) -> bool {
        self.confirm_delete
    }

    #[allow(dead_code)] // Will be used for layout modes
    pub fn get_compact_layout(&self) -> bool {
        self.compact_layout
    }
}

impl Default for SettingsWindow {
    fn default() -> Self {
        Self::new()
    }
}