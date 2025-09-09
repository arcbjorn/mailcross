use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyAction {
    // Navigation
    NextItem,
    PrevItem,
    FirstItem,
    LastItem,
    NextPanel,
    PrevPanel,
    
    // Account switching
    SwitchAccount(usize),
    
    // Email operations
    Compose,
    Reply,
    Forward,
    Delete,
    #[allow(dead_code)] // Will be used for email state management
    MarkRead,
    #[allow(dead_code)] // Will be used for email state management
    MarkUnread,
    
    // Search and selection
    SearchCurrentEmail,
    SearchAllEmails,
    SearchNext,
    SearchPrev,
    Select,
    SelectAll,
    
    // View operations
    #[allow(dead_code)] // Will be used for email operations
    ToggleRead,
    RefreshFolder,
    
    // Settings and help
    Settings,
    Help,
    #[allow(dead_code)] // Will be used for mode switching
    ToggleVimMode,
    
    // Special
    Cancel,
    Confirm,
    EnterCommandMode,
}

pub struct KeyboardHandler {
    pub vim_mode: bool,
    pub search_mode: bool,
    last_key_time: std::time::Instant,
}

impl KeyboardHandler {
    pub fn new() -> Self {
        Self {
            vim_mode: false,
            search_mode: false,
            last_key_time: std::time::Instant::now(),
        }
    }

    pub fn handle_input(&mut self, ctx: &egui::Context) -> Option<KeyAction> {
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key { key, pressed: true, modifiers, .. } = event {
                    return self.process_key(*key, *modifiers);
                }
            }
            None
        })
    }

    fn process_key(&mut self, key: egui::Key, modifiers: egui::Modifiers) -> Option<KeyAction> {
        if self.vim_mode {
            self.process_vim_key(key, modifiers)
        } else {
            self.process_traditional_key(key, modifiers)
        }
    }

    // Traditional keyboard navigation (default mode)
    fn process_traditional_key(&mut self, key: egui::Key, modifiers: egui::Modifiers) -> Option<KeyAction> {
        use egui::Key;

        match (key, modifiers) {
            // Account switching
            (Key::Num1, m) if m.ctrl => Some(KeyAction::SwitchAccount(0)),
            (Key::Num2, m) if m.ctrl => Some(KeyAction::SwitchAccount(1)),
            (Key::Num3, m) if m.ctrl => Some(KeyAction::SwitchAccount(2)),
            
            // Navigation
            (Key::Tab, m) if m.shift => Some(KeyAction::PrevPanel),
            (Key::Tab, _) => Some(KeyAction::NextPanel),
            (Key::ArrowLeft, _) => Some(KeyAction::PrevPanel),
            (Key::ArrowRight, _) => Some(KeyAction::NextPanel),
            (Key::ArrowUp, _) => Some(KeyAction::PrevItem),
            (Key::ArrowDown, _) => Some(KeyAction::NextItem),
            (Key::Home, _) => Some(KeyAction::FirstItem),
            (Key::End, _) => Some(KeyAction::LastItem),
            (Key::PageUp, _) => Some(KeyAction::FirstItem),
            (Key::PageDown, _) => Some(KeyAction::LastItem),
            
            // Email operations
            (Key::N, m) if m.ctrl => Some(KeyAction::Compose),
            (Key::R, m) if m.ctrl => Some(KeyAction::Reply),
            (Key::L, m) if m.ctrl => Some(KeyAction::Forward), // L for forLward/reLay
            (Key::D, m) if m.ctrl => Some(KeyAction::Delete),
            (Key::Delete, _) => Some(KeyAction::Delete),
            
            // Search - Standard convention
            (Key::F, m) if m.ctrl && !m.shift => Some(KeyAction::SearchCurrentEmail), // Ctrl+F: search current email
            (Key::F, m) if m.ctrl && m.shift => Some(KeyAction::SearchAllEmails),     // Ctrl+Shift+F: search all emails
            (Key::G, m) if m.ctrl && m.shift => Some(KeyAction::SearchPrev),
            (Key::G, m) if m.ctrl => Some(KeyAction::SearchNext),
            
            // Selection
            (Key::Space, _) => Some(KeyAction::Select),
            (Key::A, m) if m.ctrl => Some(KeyAction::SelectAll),
            (Key::Enter, _) => Some(KeyAction::Confirm),
            (Key::Escape, _) => Some(KeyAction::Cancel),
            
            // View operations
            (Key::R, m) if m.ctrl && m.shift => Some(KeyAction::RefreshFolder),
            
            // Close/Cancel
            (Key::W, m) if m.ctrl => Some(KeyAction::Cancel),
            
            // Settings  
            (Key::S, m) if m.alt => Some(KeyAction::Settings),
            (Key::H, m) if m.ctrl => Some(KeyAction::Help),
            
            _ => None,
        }
    }

    // Vim-style keyboard navigation
    fn process_vim_key(&mut self, key: egui::Key, modifiers: egui::Modifiers) -> Option<KeyAction> {
        use egui::Key;

        match (key, modifiers) {
            // Account switching
            (Key::Num1, _) => Some(KeyAction::SwitchAccount(0)),
            (Key::Num2, _) => Some(KeyAction::SwitchAccount(1)),
            (Key::Num3, _) => Some(KeyAction::SwitchAccount(2)),
            
            // Navigation (vim-style)
            (Key::J, _) => Some(KeyAction::NextItem),
            (Key::K, _) => Some(KeyAction::PrevItem),
            (Key::H, _) => Some(KeyAction::PrevPanel),
            (Key::L, _) => Some(KeyAction::NextPanel),
            (Key::G, m) if m.shift => Some(KeyAction::LastItem), // G (shift+g)
            
            // Email operations (vim-style)
            (Key::C, _) => Some(KeyAction::Compose),
            (Key::R, _) => Some(KeyAction::Reply),
            (Key::F, _) => Some(KeyAction::Forward),
            
            // Search
            (Key::Slash, _) => Some(KeyAction::SearchAllEmails), // Vim / searches all
            (Key::N, m) if m.shift => Some(KeyAction::SearchPrev),
            (Key::N, _) => Some(KeyAction::SearchNext),
            
            // Selection and actions
            (Key::Space, _) => Some(KeyAction::Select),
            (Key::V, _) => Some(KeyAction::SelectAll), // Visual mode
            (Key::Enter, _) => Some(KeyAction::Confirm),
            (Key::Escape, _) => Some(KeyAction::Cancel),
            
            // View operations
            (Key::U, m) if m.ctrl => Some(KeyAction::RefreshFolder),
            
            // Command mode entry
            (Key::Colon, _) => Some(KeyAction::EnterCommandMode),
            
            // Special vim sequences
            _ => self.process_vim_sequence(key, modifiers),
        }
    }

    // Handle special vim key sequences like 'dd' for delete
    fn process_vim_sequence(&mut self, key: egui::Key, _modifiers: egui::Modifiers) -> Option<KeyAction> {
        use egui::Key;
        
        let now = std::time::Instant::now();
        let time_since_last = now.duration_since(self.last_key_time).as_millis();
        self.last_key_time = now;

        // Handle 'gg' for first item (must be within 500ms of each other)
        if key == Key::G && time_since_last < 500 {
            return Some(KeyAction::FirstItem);
        }

        // Handle 'dd' for delete (must be within 500ms of each other)
        if key == Key::D && time_since_last < 500 {
            return Some(KeyAction::Delete);
        }

        None
    }

    pub fn toggle_vim_mode(&mut self) {
        self.vim_mode = !self.vim_mode;
    }

    pub fn set_search_mode(&mut self, enabled: bool) {
        self.search_mode = enabled;
    }
}

impl Default for KeyboardHandler {
    fn default() -> Self {
        Self::new()
    }
}