#[derive(Debug, Clone)]
#[allow(dead_code)] // Will be used when IMAP is implemented
pub struct Email {
    pub id: usize,
    pub sender: String,
    pub subject: String,
    pub date: String,
    pub body: String,
    pub is_read: bool,
    pub is_selected: bool,
}

impl Email {
    #[allow(dead_code)] // Will be used when IMAP is implemented
    pub fn new(id: usize, sender: &str, subject: &str, date: &str, body: &str) -> Self {
        Self {
            id,
            sender: sender.to_string(),
            subject: subject.to_string(),
            date: date.to_string(),
            body: body.to_string(),
            is_read: false,
            is_selected: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub icon: String,
    pub count: usize,
}

impl Folder {
    pub fn new(name: &str, icon: &str, count: usize) -> Self {
        Self {
            name: name.to_string(),
            icon: icon.to_string(),
            count,
        }
    }
    
    pub fn display_name(&self) -> String {
        if self.count > 0 {
            format!("{} {} ({})", self.icon, self.name, self.count)
        } else {
            format!("{} {}", self.icon, self.name)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub server: String,
    #[allow(dead_code)] // Server configuration for IMAP
    pub port: u16,
    #[allow(dead_code)] // TLS configuration
    pub use_tls: bool,
    #[allow(dead_code)] // Will store synced folders
    pub folders: Vec<Folder>,
    #[allow(dead_code)] // Will store fetched emails
    pub emails: Vec<Email>,
    pub is_connected: bool,
}

impl Account {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            server: String::new(),
            port: 993,
            use_tls: true,
            folders: vec![
                Folder::new("INBOX", "📥", 0),
                Folder::new("Sent", "📤", 0),
                Folder::new("Drafts", "📝", 0),
                Folder::new("Spam", "🗑️", 0),
            ],
            emails: Vec::new(),
            is_connected: false,
        }
    }

    #[allow(dead_code)] // Will be used for custom server configurations
    pub fn with_server(name: &str, email: &str, server: &str, port: u16, use_tls: bool) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            server: server.to_string(),
            port,
            use_tls,
            folders: Vec::new(),
            emails: Vec::new(),
            is_connected: false,
        }
    }

    pub fn connection_status(&self) -> &str {
        if self.is_connected {
            "🟢 Connected"
        } else if self.server.is_empty() {
            "⚪ Not Configured"
        } else {
            "🔴 Disconnected"
        }
    }
}