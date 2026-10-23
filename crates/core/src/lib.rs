pub mod models;

use crate::models::{Document, Folder, Vault};

pub fn create_vault(name: Option<String>) -> Vault {
    match name {
        Some(value) => Vault::new(value),
        None => Vault::new("New vault".to_string()),
    }
}

pub fn create_document(name: Option<String>, vault: &Vault, folder: Option<Folder>) -> Document {
    Document::new(name.unwrap_or("New Document".to_string()), vault, folder)
}

pub fn create_folder(name: Option<String>, vault: &Vault, parent_id: Option<&String>) -> Folder {
    match name {
        Some(value) => Folder::new(vault, value, parent_id),
        None => Folder::new(vault, "New Folder".to_string(), parent_id),
    }
}
