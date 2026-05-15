// src/csv_io.rs
// CSV import/export for contacts.
// Uses the `csv` crate for robust RFC 4180 compliance.
// Column layout follows Google Contacts CSV export format for maximum compatibility.

use std::path::Path;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::models::Contact;

// ── CSV Row Schema ────────────────────────────────────────────────────────────
// Matches Google Contacts / Outlook CSV column names for interoperability.

#[derive(Debug, Serialize, Deserialize)]
struct CsvRow {
    #[serde(rename = "First Name")]
    first_name: String,
    #[serde(rename = "Last Name")]
    last_name: String,
    #[serde(rename = "Phone 1 - Value")]
    phone_primary: String,
    #[serde(rename = "Phone 2 - Value")]
    phone_secondary: String,
    #[serde(rename = "E-mail 1 - Value")]
    email: String,
    #[serde(rename = "Organization 1 - Name")]
    organization: String,
    #[serde(rename = "Organization 1 - Title")]
    job_title: String,
    #[serde(rename = "Website 1 - Value")]
    website: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "Group Membership")]
    group: String,
}

// ── Import ────────────────────────────────────────────────────────────────────

pub fn import(path: &Path) -> Result<Vec<Contact>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)          // Allow rows with fewer columns
        .trim(csv::Trim::All)
        .from_path(path)
        .with_context(|| format!("Failed to open CSV: {}", path.display()))?;

    let mut contacts = Vec::new();

    for result in reader.deserialize::<CsvRow>() {
        match result {
            Ok(row) => {
                if row.first_name.is_empty() && row.last_name.is_empty() {
                    continue; // Skip empty rows
                }
                contacts.push(Contact {
                    id: uuid::Uuid::new_v4().to_string(),
                    first_name: row.first_name,
                    last_name: row.last_name,
                    phone_primary: row.phone_primary,
                    phone_secondary: row.phone_secondary,
                    email: row.email,
                    organization: row.organization,
                    job_title: row.job_title,
                    website: row.website,
                    notes: row.notes,
                    group: row.group,
                    ..Default::default()
                });
            }
            Err(e) => {
                log::warn!("Skipping malformed CSV row: {e}");
            }
        }
    }

    Ok(contacts)
}

// ── Export ────────────────────────────────────────────────────────────────────

pub fn export(contacts: &[Contact], path: &Path) -> Result<usize> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .from_path(path)
        .with_context(|| format!("Failed to create CSV: {}", path.display()))?;

    for contact in contacts {
        writer.serialize(CsvRow {
            first_name: contact.first_name.clone(),
            last_name: contact.last_name.clone(),
            phone_primary: contact.phone_primary.clone(),
            phone_secondary: contact.phone_secondary.clone(),
            email: contact.email.clone(),
            organization: contact.organization.clone(),
            job_title: contact.job_title.clone(),
            website: contact.website.clone(),
            notes: contact.notes.clone(),
            group: contact.group.clone(),
        })?;
    }

    writer.flush()?;
    Ok(contacts.len())
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_csv_roundtrip() {
        let contacts = vec![
            Contact {
                id: "1".to_string(),
                first_name: "Rahul".to_string(),
                last_name: "Sharma".to_string(),
                phone_primary: "+91 98765 43210".to_string(),
                email: "rahul@example.com".to_string(),
                ..Default::default()
            },
            Contact {
                id: "2".to_string(),
                first_name: "राम".to_string(),
                last_name: "देशमुख".to_string(),
                phone_primary: "9000011111".to_string(),
                ..Default::default()
            },
        ];

        let tmp = NamedTempFile::new().unwrap();
        export(&contacts, tmp.path()).unwrap();
        let imported = import(tmp.path()).unwrap();

        assert_eq!(imported.len(), 2);
        assert_eq!(imported[0].first_name, "Rahul");
        assert_eq!(imported[1].first_name, "राम");
    }
}
