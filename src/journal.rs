//! Journal system for saving and loading readings

use chrono::TimeZone;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::spreads::Reading;

/// Journal entry metadata
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub spread_name: String,
    pub card_count: usize,
}

impl JournalEntry {
    /// Format the timestamp for display
    pub fn formatted_date(&self) -> String {
        self.timestamp.format("%Y-%m-%d %H:%M").to_string()
    }
}

/// Journal manager for saving and loading readings
pub struct Journal {
    base_path: PathBuf,
}

impl Journal {
    /// Create a new journal manager
    pub fn new() -> io::Result<Self> {
        let base_path = Self::journal_dir()?;
        fs::create_dir_all(&base_path)?;
        
        Ok(Self { base_path })
    }

    /// Get the journal directory path
    fn journal_dir() -> io::Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
        Ok(home.join(".arcana").join("journal"))
    }

    /// Returns true if the journal is in a disabled state (could not be initialized)
    pub fn is_disabled(&self) -> bool {
        self.base_path.as_os_str().is_empty()
    }

    /// Save a reading to the journal
    pub fn save_reading(&self, reading: &Reading) -> io::Result<PathBuf> {
        if self.is_disabled() {
            return Err(io::Error::other(
                "Journal is disabled (could not create journal directory)"
            ));
        }
        let timestamp = reading.timestamp.format("%Y-%m-%d-%H-%M-%S");
        let filename = format!("{}-{}.md", timestamp, sanitize_filename(reading.spread.name));
        let file_path = self.base_path.join(&filename);

        let content = reading.to_markdown();
        fs::write(&file_path, content)?;

        Ok(file_path)
    }

    /// Load all journal entries
    pub fn load_entries(&self) -> io::Result<Vec<JournalEntry>> {
        let mut entries = Vec::new();

        if !self.base_path.exists() {
            return Ok(entries);
        }

        for entry in fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(journal_entry) = Self::parse_entry(&path) {
                    entries.push(journal_entry);
                }
            }
        }

        // Sort by timestamp (newest first)
        entries.sort_by_key(|b| std::cmp::Reverse(b.timestamp));

        Ok(entries)
    }

    /// Parse a journal entry from a file path
    fn parse_entry(path: &Path) -> Option<JournalEntry> {
        let filename = path.file_stem()?.to_str()?;
        let content = fs::read_to_string(path).ok()?;
        
        // Parse timestamp from filename: YYYY-MM-DD-HH-MM-SS-...
        let timestamp = if filename.len() >= 19 {
            let date_str = &filename[..19];
            chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d-%H-%M-%S")
                .ok()
                .and_then(|naive| chrono::Local.from_local_datetime(&naive).single())
        } else {
            None
        };
        let timestamp = timestamp.unwrap_or_else(chrono::Local::now);

        // Parse spread name from content
        let spread_name = content
            .lines()
            .find(|line| line.starts_with("## Spread:"))
            .map(|line| line.replace("## Spread:", "").trim().to_string())
            .unwrap_or_else(|| "Unknown Spread".to_string());

        // Count cards from content
        let card_count = content.lines().filter(|l| l.starts_with("### ")).count();

        Some(JournalEntry {
            id: filename.to_string(),
            timestamp,
            spread_name,
            card_count,
        })
    }

    /// Load the content of a specific entry
    pub fn load_entry_content(&self, id: &str) -> io::Result<String> {
        if self.is_disabled() {
            return Err(io::Error::other(
                "Journal is disabled"
            ));
        }
        let path = self.base_path.join(format!("{}.md", id));
        fs::read_to_string(path)
    }
}

impl Default for Journal {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            base_path: PathBuf::new(),
        })
    }
}

/// Sanitize a string for use in a filename
fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .replace([' ', '/', '\\', ':', '*', '?', '"', '<', '>', '|'], "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Three Card Spread"), "three-card-spread");
        assert_eq!(sanitize_filename("Celtic/Cross"), "celtic-cross");
    }
}
