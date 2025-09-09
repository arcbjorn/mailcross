use keyring::{Entry, Result as KeyringResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)] // Will be used for IMAP connections
pub struct AccountCredentials {
    pub email: String,
    pub server: String,
    pub port: u16,
    pub use_tls: bool,
}

#[allow(dead_code)] // Backend credential management
pub struct CredentialsManager {
    service_name: String,
    cache: HashMap<String, String>,
}

impl CredentialsManager {
    pub fn new() -> Self {
        Self {
            service_name: "mailcross".to_string(),
            cache: HashMap::new(),
        }
    }

    #[allow(dead_code)] // Will be used for storing credentials
    pub fn store_password(&mut self, email: &str, password: &str) -> KeyringResult<()> {
        let entry = Entry::new(&self.service_name, email)?;
        entry.set_password(password)?;
        self.cache.insert(email.to_string(), password.to_string());
        Ok(())
    }

    pub fn get_password(&mut self, email: &str) -> KeyringResult<String> {
        // Check cache first
        if let Some(password) = self.cache.get(email) {
            return Ok(password.clone());
        }

        // Retrieve from keyring
        let entry = Entry::new(&self.service_name, email)?;
        let password = entry.get_password()?;
        self.cache.insert(email.to_string(), password.clone());
        Ok(password)
    }

    #[allow(dead_code)] // Will be used for credential management
    pub fn delete_password(&mut self, email: &str) -> KeyringResult<()> {
        let entry = Entry::new(&self.service_name, email)?;
        entry.delete_credential()?;
        self.cache.remove(email);
        Ok(())
    }

    #[allow(dead_code)] // Will be used for credential checking
    pub fn has_credentials(&self, email: &str) -> bool {
        if self.cache.contains_key(email) {
            return true;
        }
        
        let entry = Entry::new(&self.service_name, email);
        if let Ok(entry) = entry {
            entry.get_password().is_ok()
        } else {
            false
        }
    }
}

impl Default for CredentialsManager {
    fn default() -> Self {
        Self::new()
    }
}