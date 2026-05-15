// src/history.rs
//
// Manages calculation history with JSON persistence.
// File: <user_data_dir>/android-calculator/history.json

use crate::error::{CalcError, CalcResult};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: String,
    pub timestamp: String,
}

impl HistoryEntry {
    pub fn new(expression: impl Into<String>, result: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            result: result.into(),
            timestamp: Local::now().format("%b %d, %H:%M").to_string(),
        }
    }
}

pub struct HistoryManager {
    entries: Vec<HistoryEntry>,
    file_path: PathBuf,
    max_entries: usize,
}

impl HistoryManager {
    pub fn new() -> Self {
        let path = data_dir().join("history.json");
        let mut mgr = Self {
            entries: Vec::new(),
            file_path: path,
            max_entries: 500,
        };
        // Load existing history, ignore errors (first run)
        let _ = mgr.load();
        mgr
    }

    pub fn add(&mut self, expression: impl Into<String>, result: impl Into<String>) {
        let entry = HistoryEntry::new(expression, result);
        self.entries.insert(0, entry); // newest first
        if self.entries.len() > self.max_entries {
            self.entries.truncate(self.max_entries);
        }
        let _ = self.save();
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.entries.len() {
            self.entries.remove(index);
            let _ = self.save();
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        let _ = self.save();
    }

    fn load(&mut self) -> CalcResult<()> {
        if !self.file_path.exists() {
            return Ok(());
        }
        let data = fs::read_to_string(&self.file_path)?;
        self.entries = serde_json::from_str(&data)?;
        Ok(())
    }

    fn save(&self) -> CalcResult<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(&self.entries)?;
        fs::write(&self.file_path, data)?;
        Ok(())
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}

fn data_dir() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("android-calculator")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry_creation() {
        let entry = HistoryEntry::new("2+2", "4");
        assert_eq!(entry.expression, "2+2");
        assert_eq!(entry.result, "4");
        assert!(!entry.timestamp.is_empty());
    }
}
