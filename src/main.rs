use core::{create_document, create_folder, create_vault, models::Vault};

fn create_new_vault() -> Vault {
    let vault = create_vault(Some("Mon crate".to_string()));
    println!("Vault créé : {}", vault.name);
    vault
}

fn main() {
    let vault = create_new_vault();
    let parent = create_folder(Some("Company".to_string()), &vault, None);
    let child = create_folder(Some("Meetings".to_string()), &vault, Some(&parent.id));
    println!("Dossiers créés : {} , {}", parent.name, child.name);

    let document_in_child = create_document(Some("Report".to_string()), &vault, Some(child));
    let document_in_parent = create_document(Some("Roadmap".to_string()), &vault, Some(parent));
    let document_in_vault = create_document(Some("Summary".to_string()), &vault, None);

    println!(
        "Documents créés : {}, {}, {}",
        document_in_child.name, document_in_parent.name, document_in_vault.name
    );

    println!(
        "Uuids : {}, {}, {}",
        document_in_child.id, document_in_parent.id, document_in_vault.id
    );
}
