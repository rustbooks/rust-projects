// tests/integration_test.rs — run with `cargo test`

use rust_dialer::app_state::{seed_contacts, AppState};
use rust_dialer::models::{CallDirection, SortMode};

fn test_state() -> AppState {
    let mut s = AppState::load_empty();
    for c in seed_contacts() {
        let _ = s.add_contact(&c.first_name, &c.last_name, &c.phone);
    }
    s
}

#[test] fn add_contact_persists_to_view() {
    let mut s = test_state();
    let n = s.view.len();
    s.add_contact("Zara","Zeta","+9000000001").unwrap();
    assert_eq!(s.view.len(), n+1);
}

#[test] fn delete_removes_contact() {
    let mut s = test_state();
    let id = s.contacts[0].id.clone();
    assert!(s.delete_contact(&id));
    assert!(!s.contacts.iter().any(|c| c.id == id));
}

#[test] fn search_by_first_name() {
    let mut s = test_state();
    s.set_search("alice");
    assert!(s.view.iter().all(|c| c.display_name().to_lowercase().contains("alice")));
}

#[test] fn search_empty_returns_all() {
    let mut s = test_state();
    let total = s.contacts.len();
    s.set_search("");
    assert_eq!(s.view.len(), total);
}

#[test] fn sort_first_name() {
    let mut s = test_state();
    s.set_sort_mode(SortMode::FirstName);
    let names: Vec<_> = s.view.iter().map(|c| c.first_name.to_lowercase()).collect();
    let mut sorted = names.clone(); sorted.sort_unstable();
    assert_eq!(names, sorted);
}

#[test] fn sort_last_name() {
    let mut s = test_state();
    s.set_sort_mode(SortMode::LastName);
    let names: Vec<_> = s.view.iter().map(|c| c.last_name.to_lowercase()).collect();
    let mut sorted = names.clone(); sorted.sort_unstable();
    assert_eq!(names, sorted);
}

#[test] fn toggle_favorite() {
    let mut s  = test_state();
    let id     = s.contacts[0].id.clone();
    let before = s.contacts[0].is_favorite;
    let after  = s.toggle_favorite(&id).unwrap();
    assert_ne!(before, after);
}

#[test] fn record_call_in_history() {
    let mut s = test_state();
    s.record_call("Test","+ 1000000000", CallDirection::Outgoing, 30);
    assert_eq!(s.call_history[0].contact_name, "Test");
}

#[test] fn add_empty_name_fails() {
    let mut s = test_state();
    assert!(s.add_contact("","","+1234567890").is_err());
}
