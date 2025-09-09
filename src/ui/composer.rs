use eframe::egui;
use crate::types::{Account, Email};
use std::sync::mpsc::Sender;

#[derive(Debug, Clone)]
pub enum ComposerAction {
    Send,
    Save,
    Cancel,
}

#[derive(Debug, Clone)]
pub enum ComposerMessage {
    SendEmail {
        from: String,
        to: String,
        subject: String,
        body: String,
    },
    SaveDraft {
        from: String,
        to: String,
        subject: String,
        body: String,
    },
}

pub struct ComposerWindow {
    pub visible: bool,
    pub to: String,
    pub cc: String,
    pub bcc: String,
    pub subject: String,
    pub body: String,
    pub from_account: usize,
    pub mode: ComposerMode,
    #[allow(dead_code)] // Will be used for message composition
    sender: Option<Sender<ComposerMessage>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComposerMode {
    Compose,
    Reply(Box<Email>),
    Forward(Box<Email>),
}

impl ComposerWindow {
    pub fn new() -> Self {
        Self {
            visible: false,
            to: String::new(),
            cc: String::new(),
            bcc: String::new(),
            subject: String::new(),
            body: String::new(),
            from_account: 0,
            mode: ComposerMode::Compose,
            sender: None,
        }
    }

    pub fn show_compose(&mut self, account_index: usize) {
        self.visible = true;
        self.mode = ComposerMode::Compose;
        self.from_account = account_index;
        self.clear_fields();
    }

    pub fn show_reply(&mut self, email: &Email, account_index: usize) {
        self.visible = true;
        self.mode = ComposerMode::Reply(Box::new(email.clone()));
        self.from_account = account_index;
        self.to = email.sender.clone();
        self.subject = if email.subject.starts_with("Re: ") {
            email.subject.clone()
        } else {
            format!("Re: {}", email.subject)
        };
        self.body = format!("\n\nOn {}, {} wrote:\n> {}", 
            email.date, 
            email.sender,
            email.body.lines().collect::<Vec<_>>().join("\n> ")
        );
        self.cc.clear();
        self.bcc.clear();
    }

    pub fn show_forward(&mut self, email: &Email, account_index: usize) {
        self.visible = true;
        self.mode = ComposerMode::Forward(Box::new(email.clone()));
        self.from_account = account_index;
        self.to.clear();
        self.cc.clear();
        self.bcc.clear();
        self.subject = if email.subject.starts_with("Fwd: ") {
            email.subject.clone()
        } else {
            format!("Fwd: {}", email.subject)
        };
        self.body = format!("\n\n---------- Forwarded message ----------\nFrom: {}\nDate: {}\nSubject: {}\nTo: {}\n\n{}", 
            email.sender, 
            email.date, 
            email.subject,
            email.recipient,
            email.body
        );
    }

    pub fn render(&mut self, ctx: &egui::Context, accounts: &[&Account]) -> Option<ComposerAction> {
        if !self.visible {
            return None;
        }

        let mut action = None;
        let mut should_close = false;

        egui::Window::new("Compose Email")
            .resizable(true)
            .default_size([600.0, 400.0])
            .open(&mut should_close)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("From:");
                    egui::ComboBox::from_id_salt("from_account")
                        .selected_text(&accounts[self.from_account].email)
                        .show_ui(ui, |ui| {
                            for (i, account) in accounts.iter().enumerate() {
                                ui.selectable_value(&mut self.from_account, i, &account.email);
                            }
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("To:");
                    ui.text_edit_singleline(&mut self.to);
                });

                ui.horizontal(|ui| {
                    ui.label("CC:");
                    ui.text_edit_singleline(&mut self.cc);
                });

                ui.horizontal(|ui| {
                    ui.label("BCC:");
                    ui.text_edit_singleline(&mut self.bcc);
                });

                ui.horizontal(|ui| {
                    ui.label("Subject:");
                    ui.text_edit_singleline(&mut self.subject);
                });

                ui.separator();

                ui.label("Body:");
                ui.text_edit_multiline(&mut self.body);

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Send").clicked() {
                        action = Some(ComposerAction::Send);
                    }
                    if ui.button("Save Draft").clicked() {
                        action = Some(ComposerAction::Save);
                    }
                    if ui.button("Cancel").clicked() {
                        action = Some(ComposerAction::Cancel);
                    }
                });

                // Show compose mode in status
                let mode_text = match &self.mode {
                    ComposerMode::Compose => "New Message",
                    ComposerMode::Reply(_) => "Reply",
                    ComposerMode::Forward(_) => "Forward",
                };
                ui.separator();
                ui.weak(format!("Mode: {}", mode_text));
            });

        // Handle window close button
        if should_close {
            self.visible = false;
            self.clear_fields();
        }

        action
    }

    fn clear_fields(&mut self) {
        self.to.clear();
        self.cc.clear();
        self.bcc.clear();
        self.subject.clear();
        self.body.clear();
    }

    pub fn set_sender(&mut self, sender: Sender<ComposerMessage>) {
        self.sender = Some(sender);
    }

    #[allow(dead_code)] // Will be used for draft functionality
    pub fn is_empty(&self) -> bool {
        self.to.is_empty() && 
        self.cc.is_empty() && 
        self.bcc.is_empty() && 
        self.subject.is_empty() && 
        self.body.is_empty()
    }

    #[allow(dead_code)] // Will be used for validation
    pub fn is_valid(&self) -> bool {
        !self.to.is_empty() && !self.subject.is_empty()
    }
}

impl Default for ComposerWindow {
    fn default() -> Self {
        Self::new()
    }
}