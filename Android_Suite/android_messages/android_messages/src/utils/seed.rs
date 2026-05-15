// src/utils/seed.rs
// Optional: populate the database with demo data for development.
// Call `seed_demo_data(&db)` from main() when a --demo flag is passed.

use anyhow::Result;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::db::Database;

pub fn seed_demo_data(db: &Database) -> Result<()> {
    // Only seed if no conversations exist yet
    let existing = db.conversations().list_all()?;
    if !existing.is_empty() {
        return Ok(());
    }

    let contacts = vec![
        ("Alice Johnson",   "+1 555 010 1111"),
        ("Bob Smith",       "+1 555 020 2222"),
        ("Carol Danvers",   "+44 7700 900123"),
        ("David Lee",       "+91 98765 43210"),
        ("Eve Martinez",    "+1 555 050 5555"),
    ];

    let messages_per_conv = vec![
        vec![
            (false, "Hey! Are you coming to the party tonight?"),
            (true,  "Absolutely! What time does it start?"),
            (false, "Around 8pm. Bring some snacks 😄"),
            (true,  "I'll bring chips and salsa. See you then! 🎉"),
        ],
        vec![
            (true,  "Did you get the report I sent?"),
            (false, "Yes, reviewing it now. Looks great so far!"),
            (true,  "Let me know if you need any changes."),
            (false, "Will do. Should have feedback by tomorrow."),
            (true,  "Perfect, no rush 👍"),
        ],
        vec![
            (false, "Happy birthday! 🎂🎉"),
            (true,  "Thank you so much! You remembered!"),
            (false, "Of course! Have a wonderful day!"),
        ],
        vec![
            (true,  "Can you send me that recipe again?"),
            (false, "Sure! 1 cup flour, 2 eggs, 200ml milk…"),
            (false, "Mix until smooth, cook on medium heat."),
            (true,  "Awesome, making it for dinner tonight!"),
            (false, "Let me know how it turns out! 🍳"),
        ],
        vec![
            (false, "The meeting has been moved to 3pm."),
            (true,  "Got it, thanks for the heads up."),
            (false, "See you there!"),
        ],
    ];

    for (i, (name, phone)) in contacts.iter().enumerate() {
        let conv = db.conversations().create(phone, name)?;
        let msgs = &messages_per_conv[i];

        for (j, (is_sent, body)) in msgs.iter().enumerate() {
            let offset = msgs.len() - j;
            let time = Utc::now() - Duration::minutes((offset * 15) as i64);

            // Insert message directly with a custom timestamp
            let conn = db.conn();
            conn.execute(
                r#"INSERT INTO messages
                   (id, conversation_id, body, sent_at, is_sent, is_read, status, is_encrypted)
                   VALUES (?1,?2,?3,?4,?5,?6,?7,0)"#,
                rusqlite::params![
                    Uuid::new_v4().to_string(),
                    conv.id,
                    body,
                    time.to_rfc3339(),
                    *is_sent as i64,
                    1i64, // all demo messages are read
                    "delivered",
                ],
            )?;
        }

        // Update snippet to last message
        let last = msgs.last().map(|(_, b)| *b).unwrap_or("");
        db.conversations().update_snippet(&conv.id, last, false)?;

        // First conversation gets 3 unread messages for demo
        if i == 0 {
            let conn = db.conn();
            conn.execute(
                "UPDATE conversations SET unread_count = 3 WHERE id = ?1",
                [&conv.id],
            )?;
        }
    }

    log::info!("Demo data seeded: {} conversations", contacts.len());
    Ok(())
}
