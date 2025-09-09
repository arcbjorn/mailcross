use crate::types::{Email, Folder};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
#[allow(dead_code)] // Email caching infrastructure
pub struct CachedEmail {
    pub email: Email,
    pub cached_at: Instant,
    pub full_body_loaded: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // Email caching infrastructure
pub struct CachedFolder {
    pub folder: Folder,
    pub emails: Vec<CachedEmail>,
    pub last_sync: Instant,
}

#[allow(dead_code)] // Email caching system
pub struct EmailCache {
    // account_email -> folder_name -> CachedFolder
    cache: HashMap<String, HashMap<String, CachedFolder>>,
    cache_duration: Duration,
}

impl EmailCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            cache_duration: Duration::from_secs(300), // 5 minutes
        }
    }

    #[allow(dead_code)] // Alternative constructor for custom cache duration
    pub fn with_duration(duration: Duration) -> Self {
        Self {
            cache: HashMap::new(),
            cache_duration: duration,
        }
    }

    #[allow(dead_code)] // Will be used for caching emails
    pub fn store_folder(&mut self, account: &str, folder: Folder, emails: Vec<Email>) {
        let cached_emails: Vec<CachedEmail> = emails
            .into_iter()
            .map(|email| CachedEmail {
                email,
                cached_at: Instant::now(),
                full_body_loaded: false,
            })
            .collect();

        let cached_folder = CachedFolder {
            folder,
            emails: cached_emails,
            last_sync: Instant::now(),
        };

        self.cache
            .entry(account.to_string())
            .or_insert_with(HashMap::new)
            .insert(cached_folder.folder.name.clone(), cached_folder);
    }

    #[allow(dead_code)] // Will be used for retrieving cached folders
    pub fn get_folder(&self, account: &str, folder_name: &str) -> Option<&CachedFolder> {
        self.cache
            .get(account)?
            .get(folder_name)
            .filter(|cached| self.is_fresh(cached.last_sync))
    }

    #[allow(dead_code)] // Will be used for retrieving cached emails
    pub fn get_emails(&self, account: &str, folder_name: &str) -> Option<Vec<Email>> {
        self.get_folder(account, folder_name)
            .map(|cached_folder| {
                cached_folder
                    .emails
                    .iter()
                    .map(|cached_email| cached_email.email.clone())
                    .collect()
            })
    }

    #[allow(dead_code)] // Will be used for getting cached folders
    pub fn get_folders(&self, account: &str) -> Vec<Folder> {
        self.cache
            .get(account)
            .map(|folders| {
                folders
                    .values()
                    .filter(|cached| self.is_fresh(cached.last_sync))
                    .map(|cached| cached.folder.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    #[allow(dead_code)] // Will be used for caching email bodies
    pub fn store_email_body(&mut self, account: &str, folder_name: &str, email_id: usize, body: String) {
        if let Some(cached_folder) = self.cache
            .get_mut(account)
            .and_then(|folders| folders.get_mut(folder_name))
        {
            if let Some(cached_email) = cached_folder
                .emails
                .iter_mut()
                .find(|e| e.email.id == email_id)
            {
                cached_email.email.body = body;
                cached_email.full_body_loaded = true;
                cached_email.cached_at = Instant::now();
            }
        }
    }

    #[allow(dead_code)] // Will be used for retrieving email bodies
    pub fn get_email_body(&self, account: &str, folder_name: &str, email_id: usize) -> Option<String> {
        self.cache
            .get(account)?
            .get(folder_name)?
            .emails
            .iter()
            .find(|e| e.email.id == email_id && e.full_body_loaded)?
            .email
            .body
            .clone()
            .into()
    }

    #[allow(dead_code)] // Will be used for cache freshness checking
    pub fn is_folder_fresh(&self, account: &str, folder_name: &str) -> bool {
        self.get_folder(account, folder_name).is_some()
    }

    #[allow(dead_code)] // Will be used for account cleanup
    pub fn clear_account(&mut self, account: &str) {
        self.cache.remove(account);
    }

    #[allow(dead_code)] // Will be used for cache maintenance
    pub fn clear_expired(&mut self) {
        let now = Instant::now();
        
        for account_cache in self.cache.values_mut() {
            account_cache.retain(|_, cached_folder| {
                now.duration_since(cached_folder.last_sync) < self.cache_duration
            });
        }

        // Remove empty account caches
        self.cache.retain(|_, account_cache| !account_cache.is_empty());
    }

    #[allow(dead_code)] // Helper for cache freshness
    fn is_fresh(&self, cached_at: Instant) -> bool {
        Instant::now().duration_since(cached_at) < self.cache_duration
    }

    #[allow(dead_code)] // Will be used for cache debugging
    pub fn cache_stats(&self) -> (usize, usize) {
        let account_count = self.cache.len();
        let total_emails: usize = self.cache
            .values()
            .flat_map(|folders| folders.values())
            .map(|folder| folder.emails.len())
            .sum();
        
        (account_count, total_emails)
    }
}

impl Default for EmailCache {
    fn default() -> Self {
        Self::new()
    }
}