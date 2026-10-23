use ::uuid::Uuid;

pub struct Document {
    pub id: String,
    pub folder_id: Option<String>,
    pub vault_id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Vault {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub id: String,
    pub parent_id: Option<String>,
    pub vault_id: String,
    pub name: String,
}

impl Vault {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
        }
    }
}

impl Document {
    pub fn new(name: String, vault: &Vault, folder: Option<Folder>) -> Self {
        match folder {
            Some(f) => Self {
                id: Uuid::new_v4().to_string(),
                folder_id: Some(f.id),
                vault_id: vault.id.clone(),
                name,
            },
            None => Self {
                id: Uuid::new_v4().to_string(),
                folder_id: None,
                vault_id: vault.id.clone(),
                name,
            },
        }
    }
}

impl Folder {
    pub fn new(vault: &Vault, name: String, parent: Option<&String>) -> Self {
        match parent {
            Some(path) => Self {
                id: Uuid::new_v4().to_string(),
                parent_id: Some(path.clone()),
                vault_id: vault.id.clone(),
                name,
            },
            None => Self {
                id: Uuid::new_v4().to_string(),
                parent_id: None,
                vault_id: vault.id.clone(),
                name,
            },
        }
    }
}
