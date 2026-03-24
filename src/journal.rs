//! Journal system for saving and loading readings

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::spreads::Reading;

/// Journal entry metadata
#[derive(Debug, Clone)]
pub struct JournalEntry {
    #[allow(dead_code)]
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub spread_name: String,
    pub card_count: usize,
    #[allow(dead_code)]
    pub file_path: PathBuf,
}

impl JournalEntry {
    /// Format the timestamp for display
    pub fn formatted_date(&self) -> String {
        self.timestamp.format("%Y-%m-%d %H:%M").to_string()
    }

    /// Format for listing
    #[allow(dead_code)]
    pub fn list_display(&self) -> String {
        format!(
            "{} - {} ({} cards)",
            self.formatted_date(),
            self.spread_name,
            self.card_count
        )
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

    /// Save a reading to the journal
    pub fn save_reading(&self, reading: &Reading) -> io::Result<PathBuf> {
        let timestamp = reading.timestamp.format("%Y-%m-%d-%H-%M-%S");
        let filename = format!("{}-{}.md", timestamp, sanitize_filename(&reading.spread.name));
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
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(entries)
    }

    /// Parse a journal entry from a file path
    fn parse_entry(path: &Path) -> Option<JournalEntry> {
        let filename = path.file_stem()?.to_str()?;
        let content = fs::read_to_string(path).ok()?;
        
        // Get file modification time as timestamp
        let metadata = fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;
        let timestamp = chrono::DateTime::from(modified);

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
            file_path: path.to_path_buf(),
        })
    }

    /// Load the content of a specific entry
    #[allow(dead_code)]
    pub fn load_entry_content(&self, id: &str) -> io::Result<String> {
        let path = self.base_path.join(format!("{}.md", id));
        fs::read_to_string(path)
    }

    /// Delete an entry
    #[allow(dead_code)]
    pub fn delete_entry(&self, id: &str) -> io::Result<()> {
        let path = self.base_path.join(format!("{}.md", id));
        fs::remove_file(path)
    }

    /// Get the base path
    #[allow(dead_code)]
    pub fn path(&self) -> &Path {
        &self.base_path
    }
}

impl Default for Journal {
    fn default() -> Self {
        Self::new().expect("Failed to create journal")
    }
}

/// Sanitize a string for use in a filename
fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace('/', "-")
        .replace('\\', "-")
        .replace(':', "-")
        .replace('*', "-")
        .replace('?', "-")
        .replace('"', "-")
        .replace('<', "-")
        .replace('>', "-")
        .replace('|', "-")
}

/// Extension trait for Reading to add markdown export
#[allow(dead_code)]
pub trait ReadingExt {
    fn to_markdown(&self) -> String;
}

#[allow(dead_code)]
impl ReadingExt for Reading {
    fn to_markdown(&self) -> String {
        use std::fmt::Write;
        
        let mut output = String::new();
        
        writeln!(
            &mut output,
            "# Tarot Reading — {}",
            self.timestamp.format("%Y-%m-%d %H:%M")
        )
        .unwrap();
        writeln!(&mut output).unwrap();
        writeln!(&mut output, "## Spread: {}", self.spread.name).unwrap();
        writeln!(&mut output).unwrap();
        
        if let Some(notes) = &self.notes {
            writeln!(&mut output, "### Notes").unwrap();
            writeln!(&mut output, "{}", notes).unwrap();
            writeln!(&mut output).unwrap();
        }
        
        for (i, (card, position)) in self
            .drawn
            .iter()
            .zip(self.spread.positions.iter())
            .enumerate()
        {
            writeln!(
                &mut output,
                "### {}. {} — {}",
                i + 1,
                position.name,
                card.card.name
            )
            .unwrap();
            
            if card.reversed {
                writeln!(&mut output, "**Reversed**").unwrap();
            } else {
                writeln!(&mut output, "**Upright**").unwrap();
            }
            writeln!(&mut output).unwrap();
            
            writeln!(&mut output, "**Keywords:** {}", card.keywords()).unwrap();
            writeln!(&mut output).unwrap();
            
            writeln!(&mut output, "**Meaning:**").unwrap();
            writeln!(&mut output, "{}", card.meaning()).unwrap();
            writeln!(&mut output).unwrap();
            
            if let Some(element) = card.card.element {
                writeln!(&mut output, "- **Element:** {:?}", element).unwrap();
            }
            if let Some(astrology) = card.card.astrology {
                writeln!(&mut output, "- **Astrology:** {}", astrology).unwrap();
            }
            writeln!(&mut output).unwrap();
        }
        
        output
    }
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
