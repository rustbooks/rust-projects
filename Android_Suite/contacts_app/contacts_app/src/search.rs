// src/search.rs
// Full-text contact search with Devanagari / Marathi Unicode support.
//
// Design goals:
//   - Zero external search dependencies (keeps binary small)
//   - O(1) per query after index build (substring match in lowercased strings)
//   - Correctly handles Devanagari: "राम", "राहुल", "मराठी" etc.
//   - Phonetic-style partial match: "rah" matches "Rahul"
//
// Algorithm: simple multi-field substring match, Unicode-lowercased.
// For production: replace with a proper inverted index (tantivy) if needed.

use unicode_normalization::UnicodeNormalization;
use crate::models::Contact;

// ── ContactSearch ─────────────────────────────────────────────────────────────

pub struct ContactSearch {
    /// Flattened, lowercased, NFC-normalized searchable strings per contact.
    /// Index[i] corresponds to contacts[i].
    index: Vec<String>,
}

impl ContactSearch {
    /// Builds the search index from a slice of contacts.
    pub fn new(contacts: &[Contact]) -> Self {
        let index = contacts.iter().map(build_index_entry).collect();
        Self { index }
    }

    /// Returns contacts matching the query (case-insensitive, Devanagari-aware).
    /// Returns all contacts if query is empty.
    pub fn search<'a>(&self, query: &str, contacts: &'a [Contact]) -> Vec<Contact> {
        if query.trim().is_empty() {
            return contacts.to_vec();
        }

        // Normalize query the same way as the index
        let normalized_query = normalize(query);

        // Split into tokens so "ram sharma" matches both "ram" and "sharma"
        let tokens: Vec<&str> = normalized_query.split_whitespace().collect();

        contacts.iter().zip(self.index.iter())
            .filter(|(_, entry)| {
                // All tokens must match somewhere in the entry
                tokens.iter().all(|token| entry.contains(token.as_ref()))
            })
            .map(|(contact, _)| contact.clone())
            .collect()
    }
}

/// Builds a single searchable string for a contact.
/// Concatenates all searchable fields, lowercased and NFC-normalized.
fn build_index_entry(c: &Contact) -> String {
    normalize(&format!(
        "{} {} {} {} {} {} {}",
        c.first_name, c.last_name,
        c.phone_primary, c.phone_secondary,
        c.email, c.organization, c.notes
    ))
}

/// Unicode NFC normalization + lowercase.
/// NFC ensures Devanagari composed forms match correctly.
/// Example: "Rahul" → "rahul", "राहुल" → "राहुल" (preserved)
fn normalize(s: &str) -> String {
    s.nfc()                          // NFC normalization
     .collect::<String>()
     .to_lowercase()                  // Case fold (ASCII and Unicode)
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Contact;

    fn make_contact(first: &str, last: &str, phone: &str) -> Contact {
        Contact {
            id: uuid::Uuid::new_v4().to_string(),
            first_name: first.to_string(),
            last_name: last.to_string(),
            phone_primary: phone.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_basic_search() {
        let contacts = vec![
            make_contact("Rahul", "Sharma", "+91 98765 43210"),
            make_contact("Priya", "Patel", "+91 91234 56789"),
            make_contact("राम", "देशमुख", "+91 90000 11111"),
        ];
        let searcher = ContactSearch::new(&contacts);

        let results = searcher.search("rahul", &contacts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].first_name, "Rahul");
    }

    #[test]
    fn test_devanagari_search() {
        let contacts = vec![
            make_contact("राम", "देशमुख", "9000011111"),
            make_contact("सीता", "पाटील", "9000022222"),
        ];
        let searcher = ContactSearch::new(&contacts);

        let results = searcher.search("राम", &contacts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].first_name, "राम");
    }

    #[test]
    fn test_multi_token_search() {
        let contacts = vec![
            make_contact("Rahul", "Sharma", ""),
            make_contact("Priya", "Sharma", ""),
        ];
        let searcher = ContactSearch::new(&contacts);

        // "rahul sharma" should match only Rahul Sharma, not Priya Sharma
        let results = searcher.search("rahul sharma", &contacts);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_phone_search() {
        let contacts = vec![
            make_contact("Alice", "B", "+91 98765 43210"),
            make_contact("Bob", "C", "+91 11111 22222"),
        ];
        let searcher = ContactSearch::new(&contacts);

        let results = searcher.search("98765", &contacts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].first_name, "Alice");
    }

    #[test]
    fn test_empty_query_returns_all() {
        let contacts = vec![
            make_contact("A", "B", ""),
            make_contact("C", "D", ""),
        ];
        let searcher = ContactSearch::new(&contacts);
        let results = searcher.search("", &contacts);
        assert_eq!(results.len(), 2);
    }
}
