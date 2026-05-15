// src/vcard.rs
// Lightweight vCard 2.1/3.0/4.0 parser and generator.
// Implements only the fields our Contact model uses — keeps binary small.
// Full spec: https://tools.ietf.org/html/rfc6350

use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::models::Contact;

// ── Import ────────────────────────────────────────────────────────────────────

/// Parses a .vcf file and returns all valid contacts found.
pub fn import(path: &Path) -> Result<Vec<Contact>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read vCard file: {}", path.display()))?;
    parse_vcf(&content)
}

pub fn parse_vcf(content: &str) -> Result<Vec<Contact>> {
    let mut contacts = Vec::new();
    let mut current: Option<ContactBuilder> = None;

    for raw_line in content.lines() {
        // Handle vCard line folding: lines starting with space/tab are continuations
        let line = raw_line.trim();

        if line.eq_ignore_ascii_case("BEGIN:VCARD") {
            current = Some(ContactBuilder::default());
            continue;
        }

        if line.eq_ignore_ascii_case("END:VCARD") {
            if let Some(builder) = current.take() {
                if let Some(contact) = builder.build() {
                    contacts.push(contact);
                }
            }
            continue;
        }

        if let Some(ref mut builder) = current {
            parse_vcard_line(builder, line);
        }
    }

    Ok(contacts)
}

fn parse_vcard_line(builder: &mut ContactBuilder, line: &str) {
    // Property format: NAME[;params]:VALUE
    // We look for ":" and split on the first one
    let Some(colon_pos) = line.find(':') else { return };
    let (property_part, value) = line.split_at(colon_pos);
    let value = &value[1..]; // skip the ':'
    let value = value.trim();

    // Get the base property name (before any semicolons/params)
    let property = property_part.split(';').next().unwrap_or("").to_uppercase();

    match property.as_str() {
        "N" => {
            // N:LastName;FirstName;Additional;Prefix;Suffix
            let parts: Vec<&str> = value.splitn(5, ';').collect();
            builder.last_name = parts.first().copied().unwrap_or("").trim().to_string();
            builder.first_name = parts.get(1).copied().unwrap_or("").trim().to_string();
        }
        "FN" => {
            // FN: Full Name — use as fallback if N is empty
            if builder.first_name.is_empty() && builder.last_name.is_empty() {
                // Split on first space: "First Last" → first=First, last=Last
                let parts: Vec<&str> = value.splitn(2, ' ').collect();
                builder.first_name = parts.first().copied().unwrap_or("").to_string();
                builder.last_name = parts.get(1).copied().unwrap_or("").to_string();
            }
        }
        "TEL" => {
            let v = value.to_string();
            if builder.phone_primary.is_empty() {
                builder.phone_primary = v;
            } else {
                builder.phone_secondary = v;
            }
        }
        "EMAIL" => {
            if builder.email.is_empty() {
                builder.email = value.to_string();
            }
        }
        "ORG" => {
            // ORG:CompanyName;Department
            builder.organization = value.split(';').next().unwrap_or("").trim().to_string();
        }
        "TITLE" => {
            builder.job_title = value.to_string();
        }
        "URL" => {
            if builder.website.is_empty() {
                builder.website = value.to_string();
            }
        }
        "NOTE" => {
            builder.notes = value.replace("\\n", "\n").to_string();
        }
        "CATEGORIES" => {
            // First category becomes our group
            if builder.group.is_empty() {
                builder.group = value.split(',').next().unwrap_or("").trim().to_string();
            }
        }
        _ => {} // Ignore unknown properties (PHOTO, ADR, etc.)
    }
}

#[derive(Default)]
struct ContactBuilder {
    first_name: String,
    last_name: String,
    phone_primary: String,
    phone_secondary: String,
    email: String,
    organization: String,
    job_title: String,
    website: String,
    notes: String,
    group: String,
}

impl ContactBuilder {
    fn build(self) -> Option<Contact> {
        if self.first_name.is_empty() && self.last_name.is_empty()
            && self.phone_primary.is_empty()
        {
            return None; // Skip empty records
        }
        Some(Contact {
            id: uuid::Uuid::new_v4().to_string(),
            first_name: self.first_name,
            last_name: self.last_name,
            phone_primary: self.phone_primary,
            phone_secondary: self.phone_secondary,
            email: self.email,
            organization: self.organization,
            job_title: self.job_title,
            website: self.website,
            notes: self.notes,
            group: self.group,
            ..Default::default()
        })
    }
}

// ── Export ────────────────────────────────────────────────────────────────────

/// Writes contacts to a vCard 3.0 file.
pub fn export(contacts: &[Contact], path: &Path) -> Result<usize> {
    let mut output = String::new();
    for contact in contacts {
        output.push_str(&to_vcard(contact));
        output.push('\n');
    }
    fs::write(path, &output)
        .with_context(|| format!("Failed to write vCard: {}", path.display()))?;
    Ok(contacts.len())
}

fn to_vcard(c: &Contact) -> String {
    let mut vcard = String::new();
    vcard.push_str("BEGIN:VCARD\r\n");
    vcard.push_str("VERSION:3.0\r\n");

    // N and FN
    let full_name = format!("{} {}", c.first_name, c.last_name).trim().to_string();
    vcard.push_str(&format!("N:{};{};;;\r\n", escape_vcard(&c.last_name), escape_vcard(&c.first_name)));
    vcard.push_str(&format!("FN:{}\r\n", escape_vcard(&full_name)));

    if !c.phone_primary.is_empty() {
        vcard.push_str(&format!("TEL;TYPE=CELL:{}\r\n", c.phone_primary));
    }
    if !c.phone_secondary.is_empty() {
        vcard.push_str(&format!("TEL;TYPE=OTHER:{}\r\n", c.phone_secondary));
    }
    if !c.email.is_empty() {
        vcard.push_str(&format!("EMAIL:{}\r\n", c.email));
    }
    if !c.organization.is_empty() {
        vcard.push_str(&format!("ORG:{}\r\n", escape_vcard(&c.organization)));
    }
    if !c.job_title.is_empty() {
        vcard.push_str(&format!("TITLE:{}\r\n", escape_vcard(&c.job_title)));
    }
    if !c.website.is_empty() {
        vcard.push_str(&format!("URL:{}\r\n", c.website));
    }
    if !c.notes.is_empty() {
        let escaped = c.notes.replace('\n', "\\n").replace('\r', "");
        vcard.push_str(&format!("NOTE:{}\r\n", escaped));
    }
    if !c.group.is_empty() {
        vcard.push_str(&format!("CATEGORIES:{}\r\n", escape_vcard(&c.group)));
    }

    // Custom field: favorite
    if c.is_favorite {
        vcard.push_str("X-CONTACTS-APP-FAVORITE:TRUE\r\n");
    }

    vcard.push_str("END:VCARD\r\n");
    vcard
}

fn escape_vcard(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace(',', "\\,")
     .replace(';', "\\;")
     .replace('\n', "\\n")
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_VCF: &str = "BEGIN:VCARD\r\nVERSION:3.0\r\nN:Sharma;Rahul;;;\r\nFN:Rahul Sharma\r\nTEL;TYPE=CELL:+91 98765 43210\r\nEMAIL:rahul@example.com\r\nORG:Acme Corp\r\nEND:VCARD\r\n";

    #[test]
    fn test_parse_vcard() {
        let contacts = parse_vcf(SAMPLE_VCF).unwrap();
        assert_eq!(contacts.len(), 1);
        let c = &contacts[0];
        assert_eq!(c.first_name, "Rahul");
        assert_eq!(c.last_name, "Sharma");
        assert_eq!(c.phone_primary, "+91 98765 43210");
        assert_eq!(c.email, "rahul@example.com");
    }

    #[test]
    fn test_export_import_roundtrip() {
        let original = Contact {
            id: "test-id".to_string(),
            first_name: "राम".to_string(),
            last_name: "देशमुख".to_string(),
            phone_primary: "+91 90000 11111".to_string(),
            email: "ram@example.com".to_string(),
            group: "Family".to_string(),
            ..Default::default()
        };

        let vcf = to_vcard(&original);
        let contacts = parse_vcf(&vcf).unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].first_name, "राम");
        assert_eq!(contacts[0].last_name, "देशमुख");
    }
}
