use crate::backend::credentials::{AccountCredentials, CredentialsManager};
use crate::types::{Email, Folder};
use imap::Client;
use std::net::TcpStream;

pub type ImapSession = imap::Session<TcpStream>;

#[derive(Debug)]
#[allow(dead_code)] // Will be used when implementing email operations
pub enum ImapError {
    Connection(String),
    Authentication(String),
    Operation(String),
    Credentials(String),
}

impl From<imap::Error> for ImapError {
    fn from(error: imap::Error) -> Self {
        ImapError::Operation(error.to_string())
    }
}

#[allow(dead_code)] // Backend infrastructure for IMAP operations
pub struct ImapClient {
    credentials_manager: CredentialsManager,
    active_sessions: std::collections::HashMap<String, ImapSession>,
}

impl ImapClient {
    pub fn new() -> Self {
        Self {
            credentials_manager: CredentialsManager::new(),
            active_sessions: std::collections::HashMap::new(),
        }
    }

    #[allow(dead_code)] // Will be used for email operations
    pub async fn connect(&mut self, account_creds: &AccountCredentials) -> Result<(), ImapError> {
        let password = self.credentials_manager
            .get_password(&account_creds.email)
            .map_err(|e| ImapError::Credentials(e.to_string()))?;

        let address = format!("{}:{}", account_creds.server, account_creds.port);
        let tcp_stream = TcpStream::connect(&address)
            .map_err(|e| ImapError::Connection(format!("Failed to connect to {}: {}", address, e)))?;

        // For now, create a basic client - TLS support will be added later
        let client = Client::new(tcp_stream);

        let session = client
            .login(&account_creds.email, &password)
            .map_err(|(e, _)| ImapError::Authentication(format!("Login failed: {}", e)))?;

        self.active_sessions.insert(account_creds.email.clone(), session);
        Ok(())
    }

    #[allow(dead_code)] // Will be used for account management
    pub fn disconnect(&mut self, email: &str) {
        if let Some(mut session) = self.active_sessions.remove(email) {
            let _ = session.logout();
        }
    }

    #[allow(dead_code)] // Will be used for folder syncing
    pub fn get_folders(&mut self, email: &str) -> Result<Vec<Folder>, ImapError> {
        let session = self.active_sessions
            .get_mut(email)
            .ok_or_else(|| ImapError::Operation("No active session for this account".to_string()))?;

        let folders = session.list(Some(""), Some("*"))?;
        
        let mut result = Vec::new();
        for folder in folders.iter() {
            let name = folder.name();
            let icon = match name.to_uppercase().as_str() {
                "INBOX" => "📥",
                "SENT" | "SENT ITEMS" => "📤", 
                "DRAFTS" => "📝",
                "SPAM" | "JUNK" => "🗑️",
                "TRASH" | "DELETED" => "🗑️",
                _ => "📁",
            };
            
            // Get message count for this folder  
            let mailbox = session.select(name)?;
            let count = mailbox.exists as usize;
            result.push(Folder::new(name, icon, count));
        }

        Ok(result)
    }

    #[allow(dead_code)] // Helper for folder operations
    fn get_folder_count(&mut self, session: &mut ImapSession, folder_name: &str) -> Result<usize, ImapError> {
        let mailbox = session.select(folder_name)?;
        // Get message count from mailbox exists field
        Ok(mailbox.exists as usize)
    }

    #[allow(dead_code)] // Will be used for email fetching
    pub fn get_emails(&mut self, email: &str, folder: &str, limit: usize) -> Result<Vec<Email>, ImapError> {
        let session = self.active_sessions
            .get_mut(email)
            .ok_or_else(|| ImapError::Operation("No active session for this account".to_string()))?;

        session.select(folder)?;
        
        // Get recent messages (up to limit)
        let messages = session.search("ALL")?;
        let mut message_vec: Vec<u32> = messages.into_iter().collect();
        message_vec.sort();
        message_vec.reverse(); // Get most recent first
        let recent_messages: Vec<u32> = message_vec.into_iter().take(limit).collect();

        let mut emails = Vec::new();
        
        for &msg_id in &recent_messages {
            let messages = session.fetch(msg_id.to_string(), "RFC822.HEADER")?;
            
            for message in messages.iter() {
                let header = message.header().unwrap_or_default();
                let header_str = String::from_utf8_lossy(header);
                
                let subject = extract_header_value(&header_str, "Subject").unwrap_or("(No Subject)".to_string());
                let from = extract_header_value(&header_str, "From").unwrap_or("Unknown Sender".to_string());
                let date = extract_header_value(&header_str, "Date").unwrap_or("Unknown Date".to_string());
                
                emails.push(Email::new(
                    msg_id as usize,
                    &from,
                    &subject,
                    &date,
                    "(Email content preview...)" // Will be loaded on demand
                ));
            }
        }

        Ok(emails)
    }

    #[allow(dead_code)] // Will be used for credential management
    pub fn store_credentials(&mut self, email: &str, password: &str) -> Result<(), ImapError> {
        self.credentials_manager
            .store_password(email, password)
            .map_err(|e| ImapError::Credentials(e.to_string()))
    }

    #[allow(dead_code)] // Will be used for credential checking
    pub fn has_credentials(&self, email: &str) -> bool {
        self.credentials_manager.has_credentials(email)
    }
}

impl Default for ImapClient {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)] // Helper function for email parsing
fn extract_header_value(header: &str, field: &str) -> Option<String> {
    header
        .lines()
        .find(|line| line.to_lowercase().starts_with(&format!("{}:", field.to_lowercase())))
        .map(|line| {
            line.split(':')
                .skip(1)
                .collect::<Vec<_>>()
                .join(":")
                .trim()
                .to_string()
        })
}