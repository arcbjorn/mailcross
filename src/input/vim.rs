
#[derive(Debug, Clone, PartialEq)]
pub enum VimMode {
    Normal,
    #[allow(dead_code)] // Will be used for multi-select operations
    Visual,
    Command,
}

#[derive(Debug)]
pub struct VimState {
    pub mode: VimMode,
    pub command_buffer: String,
    #[allow(dead_code)] // Will store last executed command for repeat operations
    pub last_command: String,
    pub repeat_count: Option<usize>,
    pub pending_action: Option<char>,
}

impl VimState {
    pub fn new() -> Self {
        Self {
            mode: VimMode::Normal,
            command_buffer: String::new(),
            last_command: String::new(),
            repeat_count: None,
            pending_action: None,
        }
    }

    pub fn reset(&mut self) {
        self.mode = VimMode::Normal;
        self.command_buffer.clear();
        self.repeat_count = None;
        self.pending_action = None;
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = VimMode::Command;
        self.command_buffer.clear();
        self.command_buffer.push(':');
    }

    #[allow(dead_code)] // Will be used for visual selection mode
    pub fn enter_visual_mode(&mut self) {
        self.mode = VimMode::Visual;
    }

    pub fn process_command(&mut self, command: &str) -> Option<VimCommand> {
        match command {
            ":q" | ":quit" => Some(VimCommand::Quit),
            ":w" | ":write" => Some(VimCommand::Save),
            ":wq" => Some(VimCommand::SaveQuit),
            ":set vim" => Some(VimCommand::EnableVimMode),
            ":set novim" => Some(VimCommand::DisableVimMode),
            ":help" => Some(VimCommand::Help),
            cmd if cmd.starts_with(":set ") => {
                // Handle :set commands
                let setting = &cmd[5..];
                Some(VimCommand::Set(setting.to_string()))
            }
            _ => None,
        }
    }

    #[allow(dead_code)] // Will be used for status display
    pub fn get_mode_indicator(&self) -> &'static str {
        match self.mode {
            VimMode::Normal => "NORMAL",
            VimMode::Visual => "VISUAL", 
            VimMode::Command => "COMMAND",
        }
    }
}

#[derive(Debug, Clone)]
pub enum VimCommand {
    Quit,
    Save,
    SaveQuit,
    EnableVimMode,
    DisableVimMode,
    Help,
    Set(String),
}

impl Default for VimState {
    fn default() -> Self {
        Self::new()
    }
}

// Vim key mappings reference
pub struct VimKeymap;

impl VimKeymap {
    pub fn get_help_text() -> Vec<(&'static str, &'static str)> {
        vec![
            // Navigation
            ("j/k", "Down/Up"),
            ("h/l", "Left panel/Right panel"),
            ("gg", "First email"),
            ("G", "Last email"),
            
            // Email operations
            ("c", "Compose"),
            ("r", "Reply"),
            ("f", "Forward"),
            ("dd", "Delete"),
            
            // Search
            ("/", "Search"),
            ("n", "Next search result"),
            ("N", "Previous search result"),
            
            // Selection
            ("Space", "Select"),
            ("v", "Visual mode (multi-select)"),
            
            // Accounts
            ("1/2/3", "Switch accounts"),
            
            // Commands
            (":", "Command mode"),
            (":q", "Quit"),
            (":set novim", "Disable vim mode"),
            
            // Special
            ("Ctrl+U", "Refresh folder"),
            ("Esc", "Cancel/Back"),
        ]
    }

    pub fn get_traditional_help_text() -> Vec<(&'static str, &'static str)> {
        vec![
            // Navigation
            ("↑↓", "Navigate emails/menus"),
            ("←→", "Navigate between panels"),
            ("Tab/Shift+Tab", "Navigate between panels"),
            ("Home/End", "First/Last email"),
            ("Page Up/Down", "First/Last email"),
            
            // Email operations
            ("Ctrl+N", "New email (compose)"),
            ("Ctrl+R", "Reply"),
            ("Ctrl+L", "Forward"),
            ("Ctrl+D/Delete", "Delete email"),
            
            // Search
            ("Ctrl+F", "Search current email"),
            ("Ctrl+Shift+F", "Search all emails"),
            ("Ctrl+G/Ctrl+Shift+G", "Next/Previous result"),
            
            // Selection
            ("Space", "Select/Multi-select"),
            ("Ctrl+A", "Select all"),
            ("Enter", "Open email/Confirm"),
            
            // Accounts
            ("Ctrl+1/2/3", "Switch accounts"),
            
            // View
            ("Ctrl+Shift+R", "Refresh"),
            ("Alt+S", "Settings"),
            ("Ctrl+H", "Help"),
            ("Esc/Ctrl+W", "Back/Cancel"),
        ]
    }
}