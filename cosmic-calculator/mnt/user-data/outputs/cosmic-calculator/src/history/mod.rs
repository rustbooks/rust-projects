use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub expression: String,
    pub result: String,
    pub timestamp: String,
    pub mode: String,
}

#[derive(Debug, Clone, Default)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
    pub max_entries: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 100,
        }
    }

    pub fn push(&mut self, expression: String, result: String, mode: String) {
        let entry = HistoryEntry {
            expression,
            result,
            timestamp: chrono_now(),
            mode,
        };
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

fn chrono_now() -> String {
    // Simple timestamp without chrono dependency
    "Now".to_string()
}
