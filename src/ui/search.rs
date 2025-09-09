use eframe::egui;
use crate::types::Email;

#[derive(Debug, Clone)]
pub struct SearchState {
    pub active: bool,
    pub query: String,
    pub results: Vec<Email>,
    pub selected_result: usize,
    pub search_mode: SearchMode,
    pub search_scope: SearchScope,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchMode {
    Subject,
    Sender,
    Body,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchScope {
    CurrentEmail,
    AllEmails,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            active: false,
            query: String::new(),
            results: Vec::new(),
            selected_result: 0,
            search_mode: SearchMode::All,
            search_scope: SearchScope::AllEmails,
        }
    }

    pub fn start_search(&mut self, scope: SearchScope) {
        self.active = true;
        self.query.clear();
        self.results.clear();
        self.selected_result = 0;
        self.search_scope = scope;
    }

    pub fn start_search_current_email(&mut self) {
        self.start_search(SearchScope::CurrentEmail);
    }

    pub fn start_search_all_emails(&mut self) {
        self.start_search(SearchScope::AllEmails);
    }

    pub fn cancel_search(&mut self) {
        self.active = false;
        self.query.clear();
        self.results.clear();
        self.selected_result = 0;
    }

    #[allow(dead_code)] // Will be used for real-time search updates
    pub fn update_query(&mut self, new_query: String) {
        self.query = new_query;
        // Reset selection when query changes
        self.selected_result = 0;
    }

    pub fn perform_search(&mut self, emails: &[Email]) {
        if self.query.is_empty() {
            self.results.clear();
            return;
        }

        let query_lower = self.query.to_lowercase();
        
        self.results = emails
            .iter()
            .filter(|email| self.matches_search(email, &query_lower))
            .cloned()
            .collect();
            
        // Reset selection if results changed
        if self.selected_result >= self.results.len() {
            self.selected_result = 0;
        }
    }

    fn matches_search(&self, email: &Email, query: &str) -> bool {
        match self.search_mode {
            SearchMode::Subject => email.subject.to_lowercase().contains(query),
            SearchMode::Sender => email.sender.to_lowercase().contains(query),
            SearchMode::Body => email.body.to_lowercase().contains(query),
            SearchMode::All => {
                email.subject.to_lowercase().contains(query)
                    || email.sender.to_lowercase().contains(query)
                    || email.body.to_lowercase().contains(query)
                    || email.recipient.to_lowercase().contains(query)
            }
        }
    }

    pub fn next_result(&mut self) {
        if !self.results.is_empty() {
            self.selected_result = (self.selected_result + 1) % self.results.len();
        }
    }

    pub fn prev_result(&mut self) {
        if !self.results.is_empty() {
            self.selected_result = if self.selected_result == 0 {
                self.results.len() - 1
            } else {
                self.selected_result - 1
            };
        }
    }

    #[allow(dead_code)] // Will be used for email selection
    pub fn get_selected_email(&self) -> Option<&Email> {
        self.results.get(self.selected_result)
    }

    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }

    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    #[allow(dead_code)] // Will be used for search mode cycling
    pub fn cycle_search_mode(&mut self) {
        self.search_mode = match self.search_mode {
            SearchMode::All => SearchMode::Subject,
            SearchMode::Subject => SearchMode::Sender,
            SearchMode::Sender => SearchMode::Body,
            SearchMode::Body => SearchMode::All,
        };
    }

    pub fn get_mode_display(&self) -> &'static str {
        match self.search_mode {
            SearchMode::Subject => "Subject",
            SearchMode::Sender => "Sender",
            SearchMode::Body => "Body",
            SearchMode::All => "All",
        }
    }

    pub fn get_scope_display(&self) -> &'static str {
        match self.search_scope {
            SearchScope::CurrentEmail => "Current Email",
            SearchScope::AllEmails => "All Emails",
        }
    }

    #[allow(dead_code)] // May be used for window titles in future
    pub fn get_search_title(&self) -> String {
        format!("🔍 Search {} ({})", self.get_scope_display(), self.get_mode_display())
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SearchPanel;

impl SearchPanel {
    pub fn render_search_bar(ui: &mut egui::Ui, search_state: &mut SearchState) -> bool {
        if !search_state.active {
            return false;
        }

        let mut search_completed = false;
        
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = crate::ui::ResponsiveLayout::PANEL_SPACING;
            
            // Compact search indicator
            ui.weak("🔍");
            ui.colored_label(
                ui.visuals().strong_text_color(),
                search_state.get_scope_display()
            );
            
            // Minimal mode selector (only show if not mobile)
            let available_width = ui.available_width();
            if available_width > 400.0 {
                egui::ComboBox::from_id_salt("search_mode")
                    .selected_text(search_state.get_mode_display())
                    .width(60.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut search_state.search_mode, SearchMode::All, "All");
                        ui.selectable_value(&mut search_state.search_mode, SearchMode::Subject, "Subject");
                        ui.selectable_value(&mut search_state.search_mode, SearchMode::Sender, "Sender");
                        ui.selectable_value(&mut search_state.search_mode, SearchMode::Body, "Body");
                    });
            }
            
            // Responsive search input
            let input_width = if available_width > 500.0 { 
                available_width - 300.0 
            } else { 
                available_width - 150.0 
            };
            
            let response = ui.add_sized(
                [input_width.max(100.0), 20.0],
                egui::TextEdit::singleline(&mut search_state.query)
                    .hint_text("Search...")
            );
            
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                search_completed = true;
            }
            
            // Compact results info
            if search_state.has_results() {
                ui.weak(format!("{}/{}", 
                    search_state.selected_result + 1, 
                    search_state.result_count()));
            } else if !search_state.query.is_empty() {
                ui.weak("∅");
            }
            
            // Minimal close button
            if ui.small_button("✕").clicked() {
                search_state.cancel_search();
            }
        });
        
        search_completed
    }

    #[allow(dead_code)] // Alternative search results display method
    pub fn render_results_list(ui: &mut egui::Ui, search_state: &SearchState) {
        if !search_state.active || !search_state.has_results() {
            return;
        }

        ui.separator();
        ui.heading("Search Results");
        
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for (i, email) in search_state.results.iter().enumerate() {
                    let selected = i == search_state.selected_result;
                    
                    let response = ui.selectable_label(
                        selected,
                        format!("{} - {}", email.sender, email.subject)
                    );
                    
                    if response.clicked() {
                        // This would trigger email selection - handled by parent
                    }
                    
                    if selected {
                        response.scroll_to_me(Some(egui::Align::Center));
                    }
                }
            });
    }

    #[allow(dead_code)] // Will be used for search highlighting
    pub fn highlight_search_term(text: &str, query: &str) -> String {
        if query.is_empty() {
            return text.to_string();
        }
        
        let query_lower = query.to_lowercase();
        let text_lower = text.to_lowercase();
        
        if let Some(pos) = text_lower.find(&query_lower) {
            let before = &text[..pos];
            let matched = &text[pos..pos + query.len()];
            let after = &text[pos + query.len()..];
            format!("{}**{}**{}", before, matched, after)
        } else {
            text.to_string()
        }
    }
}