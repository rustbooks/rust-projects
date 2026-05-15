// tests/integration_test.rs
// Integration tests that exercise the full DB + crypto stack without a UI.

use android_messages::crypto::Encryptor;

#[test]
fn crypto_round_trip() {
    let key = [0xABu8; 32];
    let enc = Encryptor::from_key_bytes(&key);
    let plain = "Test message with emoji 🔒🎉";
    let ct    = enc.encrypt(plain).expect("encrypt");
    let pt    = enc.decrypt(&ct).expect("decrypt");
    assert_eq!(pt, plain);
}

#[test]
fn crypto_different_nonces() {
    let key = [0x12u8; 32];
    let enc = Encryptor::from_key_bytes(&key);
    let ct1 = enc.encrypt("hello").expect("e1");
    let ct2 = enc.encrypt("hello").expect("e2");
    // Nonces are random → ciphertexts should differ even for same plaintext.
    assert_ne!(ct1, ct2);
}

#[test]
fn db_conversation_crud() {
    use std::sync::Arc;
    // Use an in-memory SQLite database for tests.
    let db = Arc::new(
        android_messages::db::Database::open_in_memory()
            .expect("in-memory db"),
    );

    // Create
    let conv = db.conversations().create("+1 555 999 0000", "Test User")
        .expect("create");
    assert_eq!(conv.contact_name, "Test User");
    assert_eq!(conv.unread_count, 0);

    // List
    let list = db.conversations().list_all().expect("list");
    assert_eq!(list.len(), 1);

    // Mark read (should be no-op since unread = 0)
    db.conversations().mark_read(&conv.id).expect("mark_read");

    // Delete
    db.conversations().delete(&conv.id).expect("delete");
    let list = db.conversations().list_all().expect("list after delete");
    assert!(list.is_empty());
}

#[test]
fn db_message_insert_and_retrieve() {
    use std::sync::Arc;
    let db = Arc::new(
        android_messages::db::Database::open_in_memory()
            .expect("in-memory db"),
    );

    let conv = db.conversations().create("+1 555 111 2222", "Msg Test")
        .expect("create conv");

    db.messages().insert(&conv.id, "Hello world", true, None, false)
        .expect("insert");

    let msgs = db.messages().for_conversation(&conv.id).expect("retrieve");
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].body, "Hello world");
    assert!(msgs[0].is_sent);
}

#[test]
fn search_conversations() {
    use std::sync::Arc;
    let db = Arc::new(
        android_messages::db::Database::open_in_memory()
            .expect("in-memory db"),
    );

    db.conversations().create("+1 555 111", "Alice Smith").expect("c1");
    db.conversations().create("+1 555 222", "Bob Jones").expect("c2");
    db.conversations().create("+1 555 333", "Alice Wonderland").expect("c3");

    let results = db.conversations().search("Alice").expect("search");
    assert_eq!(results.len(), 2);

    let results = db.conversations().search("Jones").expect("search2");
    assert_eq!(results.len(), 1);

    let results = db.conversations().search("Nobody").expect("search3");
    assert!(results.is_empty());
}
