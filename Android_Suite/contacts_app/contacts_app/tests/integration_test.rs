// tests/integration_test.rs
// Integration tests for ContactManager, search, vCard, CSV, and crypto.
// Run with: cargo test

use tempfile::TempDir;
use contacts_app::contacts::ContactManager;
use contacts_app::models::{Contact, SortKey};
use contacts_app::vcard;
use contacts_app::csv_io;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn temp_manager() -> (TempDir, ContactManager) {
    let dir = TempDir::new().expect("temp dir");
    let mgr = ContactManager::new(dir.path().to_path_buf()).expect("manager");
    (dir, mgr)
}

fn sample_contact(first: &str, last: &str) -> Contact {
    Contact {
        first_name: first.to_string(),
        last_name: last.to_string(),
        phone_primary: format!("+1 555 {:04}", first.len() * 1111 % 10000),
        email: format!("{}@example.com", first.to_lowercase()),
        ..Default::default()
    }
}

// ── CRUD Tests ────────────────────────────────────────────────────────────────

#[test]
fn test_create_and_retrieve() {
    let (_dir, mut mgr) = temp_manager();
    let id = mgr.upsert(sample_contact("Alice", "Smith")).unwrap();
    assert!(!id.is_empty());
    let all = mgr.all_sorted();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0].first_name, "Alice");
}

#[test]
fn test_update_contact() {
    let (_dir, mut mgr) = temp_manager();
    let id = mgr.upsert(sample_contact("Bob", "Jones")).unwrap();
    let mut updated = mgr.get(&id).unwrap().clone();
    updated.first_name = "Robert".to_string();
    mgr.upsert(updated).unwrap();
    let all = mgr.all_sorted();
    assert_eq!(all[0].first_name, "Robert");
    assert_eq!(all[0].id, id);
}

#[test]
fn test_delete_contact() {
    let (_dir, mut mgr) = temp_manager();
    let id = mgr.upsert(sample_contact("Charlie", "Brown")).unwrap();
    assert_eq!(mgr.all_sorted().len(), 1);
    mgr.delete(&id).unwrap();
    assert_eq!(mgr.all_sorted().len(), 0);
}

#[test]
fn test_delete_nonexistent_returns_error() {
    let (_dir, mut mgr) = temp_manager();
    assert!(mgr.delete("nonexistent-id").is_err());
}

#[test]
fn test_toggle_favorite() {
    let (_dir, mut mgr) = temp_manager();
    let id = mgr.upsert(sample_contact("Diana", "Prince")).unwrap();
    assert!(!mgr.get(&id).unwrap().is_favorite);
    assert!(mgr.toggle_favorite(&id).unwrap());
    assert!(mgr.get(&id).unwrap().is_favorite);
    assert!(!mgr.toggle_favorite(&id).unwrap());
}

#[test]
fn test_empty_name_rejected() {
    let (_dir, mut mgr) = temp_manager();
    let result = mgr.upsert(Contact {
        phone_primary: "123".to_string(),
        ..Default::default()
    });
    assert!(result.is_err());
}

// ── Sort Tests ────────────────────────────────────────────────────────────────

#[test]
fn test_sort_by_first_name() {
    let (_dir, mut mgr) = temp_manager();
    mgr.upsert(sample_contact("Zara", "A")).unwrap();
    mgr.upsert(sample_contact("Alice", "B")).unwrap();
    mgr.upsert(sample_contact("Mike", "C")).unwrap();
    mgr.set_sort(SortKey::FirstName);
    let sorted = mgr.all_sorted();
    assert_eq!(sorted[0].first_name, "Alice");
    assert_eq!(sorted[2].first_name, "Zara");
}

#[test]
fn test_sort_by_last_name() {
    let (_dir, mut mgr) = temp_manager();
    mgr.upsert(sample_contact("A", "Zebra")).unwrap();
    mgr.upsert(sample_contact("B", "Apple")).unwrap();
    mgr.set_sort(SortKey::LastName);
    let sorted = mgr.all_sorted();
    assert_eq!(sorted[0].last_name, "Apple");
    assert_eq!(sorted[1].last_name, "Zebra");
}

// ── Search Tests ──────────────────────────────────────────────────────────────

#[test]
fn test_search_ascii() {
    let (_dir, mut mgr) = temp_manager();
    mgr.upsert(sample_contact("Alice", "Smith")).unwrap();
    mgr.upsert(sample_contact("Bob", "Jones")).unwrap();
    let results = mgr.search("alice");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].first_name, "Alice");
}

#[test]
fn test_search_devanagari() {
    let (_dir, mut mgr) = temp_manager();
    mgr.upsert(Contact {
        first_name: "राम".to_string(),
        last_name: "देशमुख".to_string(),
        phone_primary: "9000011111".to_string(),
        ..Default::default()
    }).unwrap();
    mgr.upsert(Contact {
        first_name: "सीता".to_string(),
        last_name: "पाटील".to_string(),
        phone_primary: "9000022222".to_string(),
        ..Default::default()
    }).unwrap();
    let results = mgr.search("राम");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].first_name, "राम");
}

#[test]
fn test_search_by_phone() {
    let (_dir, mut mgr) = temp_manager();
    mgr.upsert(Contact {
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        phone_primary: "+91 98765 43210".to_string(),
        ..Default::default()
    }).unwrap();
    let results = mgr.search("98765");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_search_empty_returns_all() {
    let (_dir, mut mgr) = temp_manager();
    mgr.upsert(sample_contact("A", "B")).unwrap();
    mgr.upsert(sample_contact("C", "D")).unwrap();
    let results = mgr.search("");
    assert_eq!(results.len(), 2);
}

// ── Persistence Tests ─────────────────────────────────────────────────────────

#[test]
fn test_persistence_across_manager_instances() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().to_path_buf();
    {
        let mut mgr = ContactManager::new(path.clone()).unwrap();
        mgr.upsert(sample_contact("Persistent", "Data")).unwrap();
    }
    {
        let mgr = ContactManager::new(path).unwrap();
        let all = mgr.all_sorted();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].first_name, "Persistent");
    }
}

// ── vCard Tests ───────────────────────────────────────────────────────────────

#[test]
fn test_vcard_import_basic() {
    let vcf = "BEGIN:VCARD\r\nVERSION:3.0\r\nN:Patil;Priya;;;\r\nFN:Priya Patil\r\nTEL;TYPE=CELL:+91 91234 56789\r\nEMAIL:priya@example.com\r\nORG:TechCorp\r\nEND:VCARD\r\n";
    let contacts = vcard::parse_vcf(vcf).unwrap();
    assert_eq!(contacts.len(), 1);
    assert_eq!(contacts[0].first_name, "Priya");
    assert_eq!(contacts[0].last_name, "Patil");
    assert_eq!(contacts[0].phone_primary, "+91 91234 56789");
}

#[test]
fn test_vcard_multiple_contacts() {
    let vcf = concat!(
        "BEGIN:VCARD\r\nVERSION:3.0\r\nN:A;First;;;\r\nFN:First A\r\nTEL:111\r\nEND:VCARD\r\n",
        "BEGIN:VCARD\r\nVERSION:3.0\r\nN:B;Second;;;\r\nFN:Second B\r\nTEL:222\r\nEND:VCARD\r\n",
    );
    let contacts = vcard::parse_vcf(vcf).unwrap();
    assert_eq!(contacts.len(), 2);
}

#[test]
fn test_vcard_devanagari_roundtrip() {
    let original = Contact {
        id: "test-id".to_string(),
        first_name: "राम".to_string(),
        last_name: "देशमुख".to_string(),
        phone_primary: "+91 90000 11111".to_string(),
        ..Default::default()
    };
    let vcf_str = {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        vcard::export(&[original], tmp.path()).unwrap();
        std::fs::read_to_string(tmp.path()).unwrap()
    };
    let imported = vcard::parse_vcf(&vcf_str).unwrap();
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].first_name, "राम");
    assert_eq!(imported[0].last_name, "देशमुख");
}

// ── CSV Tests ─────────────────────────────────────────────────────────────────

#[test]
fn test_csv_roundtrip() {
    let contacts = vec![Contact {
        id: "1".to_string(),
        first_name: "Export".to_string(),
        last_name: "Test".to_string(),
        phone_primary: "+1 555 0001".to_string(),
        email: "export@test.com".to_string(),
        ..Default::default()
    }];
    let tmp = tempfile::NamedTempFile::new().unwrap();
    csv_io::export(&contacts, tmp.path()).unwrap();
    let imported = csv_io::import(tmp.path()).unwrap();
    assert_eq!(imported.len(), 1);
    assert_eq!(imported[0].first_name, "Export");
    assert_eq!(imported[0].email, "export@test.com");
}

// ── Avatar Tests ──────────────────────────────────────────────────────────────

#[test]
fn test_avatar_color_deterministic() {
    let c1 = Contact::compute_avatar_color("Alice", "Smith");
    let c2 = Contact::compute_avatar_color("Alice", "Smith");
    assert_eq!(c1, c2);
}

#[test]
fn test_avatar_letter_ascii() {
    let c = Contact { first_name: "Alice".to_string(), ..Default::default() };
    assert_eq!(c.avatar_letter(), "A");
}

#[test]
fn test_avatar_letter_devanagari() {
    let c = Contact { first_name: "राम".to_string(), ..Default::default() };
    let letter = c.avatar_letter();
    assert_eq!(letter.chars().count(), 1, "Must be one Unicode char, got: {letter}");
}

// ── Crypto Tests ──────────────────────────────────────────────────────────────

#[test]
fn test_encryption_roundtrip() {
    use contacts_app::crypto;
    let salt = crypto::random_salt().unwrap();
    let key = crypto::derive_key("my-secure-password", &salt);
    let plaintext = "Sensitive contact data — नमस्ते!";
    let encrypted = crypto::encrypt(&key, plaintext.as_bytes()).unwrap();
    let decrypted = crypto::decrypt(&key, &encrypted).unwrap();
    assert_eq!(String::from_utf8(decrypted).unwrap(), plaintext);
}

#[test]
fn test_wrong_key_fails_decrypt() {
    use contacts_app::crypto;
    let salt = crypto::random_salt().unwrap();
    let key1 = crypto::derive_key("correct", &salt);
    let key2 = crypto::derive_key("wrong", &salt);
    let encrypted = crypto::encrypt(&key1, b"secret").unwrap();
    assert!(crypto::decrypt(&key2, &encrypted).is_err());
}

#[test]
fn test_unique_ciphertexts_from_same_plaintext() {
    use contacts_app::crypto;
    let salt = crypto::random_salt().unwrap();
    let key = crypto::derive_key("password", &salt);
    let enc1 = crypto::encrypt(&key, b"same").unwrap();
    let enc2 = crypto::encrypt(&key, b"same").unwrap();
    assert_ne!(enc1, enc2, "Each encryption must use a unique nonce");
}
