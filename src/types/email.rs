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
#[allow(dead_code)] // Will be used when IMAP is implemented
pub struct Account {
    pub name: String,
    pub email: String,
    pub folders: Vec<Folder>,
    pub emails: Vec<Email>,
}

impl Account {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            folders: vec![
                Folder::new("INBOX", "📥", 42),
                Folder::new("Sent", "📤", 0),
                Folder::new("Drafts", "📝", 0),
                Folder::new("Spam", "🗑️", 0),
            ],
            emails: Vec::new(),
        }
    }
}