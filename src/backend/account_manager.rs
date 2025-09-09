use crate::backend::{ImapClient, EmailCache, AccountCredentials};
use crate::types::{Account, Email, Folder};
use std::collections::HashMap;
use tokio::sync::mpsc;

#[allow(dead_code)] // Will be used for async account operations
pub enum AccountCommand {
    Connect(String), // email
    Disconnect(String), // email  
    RefreshFolders(String), // email
    FetchEmails { account_email: String, folder: String, limit: usize },
    StoreCredentials { email: String, password: String },
}

#[allow(dead_code)] // Will be used for UI updates
pub enum AccountEvent {
    Connected(String),
    Disconnected(String),
    ConnectionFailed(String, String), // email, error
    FoldersUpdated(String, Vec<Folder>), // email, folders
    EmailsUpdated(String, String, Vec<Email>), // email, folder, emails
}

#[allow(dead_code)] // Backend account management infrastructure
pub struct AccountManager {
    accounts: HashMap<String, Account>,
    imap_client: ImapClient,
    email_cache: EmailCache,
    event_sender: Option<mpsc::UnboundedSender<AccountEvent>>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            imap_client: ImapClient::new(),
            email_cache: EmailCache::new(),
            event_sender: None,
        }
    }

    pub fn set_event_sender(&mut self, sender: mpsc::UnboundedSender<AccountEvent>) {
        self.event_sender = Some(sender);
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.email.clone(), account);
    }

    #[allow(dead_code)] // Will be used for account operations
    pub fn get_account(&self, email: &str) -> Option<&Account> {
        self.accounts.get(email)
    }

    #[allow(dead_code)] // Will be used for account modifications
    pub fn get_account_mut(&mut self, email: &str) -> Option<&mut Account> {
        self.accounts.get_mut(email)
    }

    pub fn get_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    #[allow(dead_code)] // Will be used for account removal
    pub fn remove_account(&mut self, email: &str) -> Option<Account> {
        self.imap_client.disconnect(email);
        self.email_cache.clear_account(email);
        self.accounts.remove(email)
    }

    #[allow(dead_code)] // Will be used for command processing
    pub async fn handle_command(&mut self, command: AccountCommand) {
        match command {
            AccountCommand::Connect(email) => {
                self.connect_account(&email).await;
            }
            AccountCommand::Disconnect(email) => {
                self.disconnect_account(&email);
            }
            AccountCommand::RefreshFolders(email) => {
                self.refresh_folders(&email).await;
            }
            AccountCommand::FetchEmails { account_email, folder, limit } => {
                self.fetch_emails(&account_email, &folder, limit).await;
            }
            AccountCommand::StoreCredentials { email, password } => {
                if let Err(e) = self.imap_client.store_credentials(&email, &password) {
                    self.send_event(AccountEvent::ConnectionFailed(email, format!("Failed to store credentials: {:?}", e)));
                }
            }
        }
    }

    #[allow(dead_code)] // Will be used for account connections
    async fn connect_account(&mut self, email: &str) {
        if let Some(account) = self.accounts.get(email).cloned() {
            if account.server.is_empty() {
                self.send_event(AccountEvent::ConnectionFailed(email.to_string(), "Server not configured".to_string()));
                return;
            }

            let credentials = AccountCredentials {
                email: account.email.clone(),
                server: account.server.clone(),
                port: account.port,
                use_tls: account.use_tls,
            };

            match self.imap_client.connect(&credentials).await {
                Ok(_) => {
                    if let Some(account) = self.accounts.get_mut(email) {
                        account.is_connected = true;
                    }
                    self.send_event(AccountEvent::Connected(email.to_string()));
                    // Auto-refresh folders after successful connection
                    self.refresh_folders(email).await;
                }
                Err(e) => {
                    self.send_event(AccountEvent::ConnectionFailed(email.to_string(), format!("{:?}", e)));
                }
            }
        }
    }

    #[allow(dead_code)] // Will be used for disconnections
    fn disconnect_account(&mut self, email: &str) {
        self.imap_client.disconnect(email);
        if let Some(account) = self.accounts.get_mut(email) {
            account.is_connected = false;
        }
        self.send_event(AccountEvent::Disconnected(email.to_string()));
    }

    #[allow(dead_code)] // Will be used for folder syncing
    async fn refresh_folders(&mut self, email: &str) {
        match self.imap_client.get_folders(email) {
            Ok(folders) => {
                // Update account folders
                if let Some(account) = self.accounts.get_mut(email) {
                    account.folders = folders.clone();
                }
                
                // Store in cache
                for folder in &folders {
                    self.email_cache.store_folder(email, folder.clone(), Vec::new());
                }

                self.send_event(AccountEvent::FoldersUpdated(email.to_string(), folders));
            }
            Err(e) => {
                self.send_event(AccountEvent::ConnectionFailed(email.to_string(), format!("Failed to refresh folders: {:?}", e)));
            }
        }
    }

    #[allow(dead_code)] // Will be used for email fetching
    async fn fetch_emails(&mut self, email: &str, folder: &str, limit: usize) {
        // Check cache first
        if let Some(cached_emails) = self.email_cache.get_emails(email, folder) {
            self.send_event(AccountEvent::EmailsUpdated(email.to_string(), folder.to_string(), cached_emails));
            return;
        }

        // Fetch from IMAP
        match self.imap_client.get_emails(email, folder, limit) {
            Ok(emails) => {
                // Update account emails
                if let Some(account) = self.accounts.get_mut(email) {
                    account.emails = emails.clone();
                }

                // Store in cache
                if let Some(folder_obj) = self.accounts.get(email)
                    .and_then(|acc| acc.folders.iter().find(|f| f.name == folder)) {
                    self.email_cache.store_folder(email, folder_obj.clone(), emails.clone());
                }

                self.send_event(AccountEvent::EmailsUpdated(email.to_string(), folder.to_string(), emails));
            }
            Err(e) => {
                self.send_event(AccountEvent::ConnectionFailed(email.to_string(), format!("Failed to fetch emails: {:?}", e)));
            }
        }
    }

    #[allow(dead_code)] // Helper for event dispatching
    fn send_event(&self, event: AccountEvent) {
        if let Some(sender) = &self.event_sender {
            let _ = sender.send(event);
        }
    }

    #[allow(dead_code)] // Will be used for credential checking
    pub fn has_stored_credentials(&self, email: &str) -> bool {
        self.imap_client.has_credentials(email)
    }

    #[allow(dead_code)] // Will be used for maintenance
    pub fn cleanup_cache(&mut self) {
        self.email_cache.clear_expired();
    }
}

impl Default for AccountManager {
    fn default() -> Self {
        Self::new()
    }
}