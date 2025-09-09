use eframe::egui;
use crate::types::Email;

#[derive(Debug, Clone)]
pub struct SearchState {
    pub active: bool,
    pub query: String,
    pub results: Vec<Email>,
    pub selected_result: usize,
    pub search_mode: SearchMode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchMode {
    Subject,
    Sender,
    Body,
    All,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            active: false,
            query: String::new(),
            results: Vec::new(),
            selected_result: 0,
            search_mode: SearchMode::All,
        }
    }

    pub fn start_search(&mut self) {
        self.active = true;
        self.query.clear();
        self.results.clear();
        self.selected_result = 0;
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
            ui.label("🔍");
            
            // Search mode selector
            egui::ComboBox::from_id_salt("search_mode")
                .selected_text(search_state.get_mode_display())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut search_state.search_mode, SearchMode::All, "All");
                    ui.selectable_value(&mut search_state.search_mode, SearchMode::Subject, "Subject");
                    ui.selectable_value(&mut search_state.search_mode, SearchMode::Sender, "Sender");
                    ui.selectable_value(&mut search_state.search_mode, SearchMode::Body, "Body");
                });
            
            // Search input
            let response = ui.text_edit_singleline(&mut search_state.query);
            
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                search_completed = true;
            }
            
            // Results info
            if search_state.has_results() {
                ui.label(format!("{}/{} results", 
                    search_state.selected_result + 1, 
                    search_state.result_count()));
            } else if !search_state.query.is_empty() {
                ui.weak("No results");
            }
            
            if ui.button("✕").clicked() {
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