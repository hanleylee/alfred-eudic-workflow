use alfred::core::AlfredUtils;
use std::path::Path;

use super::database::StardictDatabase;
use super::entry::StardictEntry;

#[derive(Debug, Clone)]
pub struct DictionaryConfig {
    pub completion_file: Option<String>,
    pub db_file: Option<String>,
}

impl DictionaryConfig {
    pub fn new(completion_file: Option<String>, db_file: Option<String>) -> Self {
        Self { completion_file, db_file }
    }
}

pub struct DictionaryManager {
    config: DictionaryConfig,
    database: Option<StardictDatabase>,
}

impl DictionaryManager {
    pub fn new(config: DictionaryConfig) -> Self {
        let database = config.db_file.as_ref().filter(|p| Path::new(p).exists()).and_then(|p| StardictDatabase::new(p).ok());
        Self { config, database }
    }

    /// Search in SQLite stardict. Returns matches up to `limit`.
    pub fn find_matches_in_db(&self, spell: &str, limit: u32) -> Vec<StardictEntry> {
        AlfredUtils::log(format!("database file: {}", self.config.db_file.as_deref().unwrap_or("")));
        let Some(ref db) = self.database else {
            return Vec::new();
        };
        match db.search_word(spell, limit) {
            Ok(entries) => entries,
            Err(e) => {
                AlfredUtils::log(format!("DB search error: {}", e));
                Vec::new()
            }
        }
    }

    /// Search in completion file (sorted word list). Binary search for prefix, then collect up to `limit`.
    pub async fn find_matches_in_completion(&self, completion_file: &str, spell: &str, limit: u32) -> Vec<String> {
        AlfredUtils::log(format!("completion file: {:?}", self.config.completion_file));
        let content = match tokio::fs::read_to_string(completion_file).await {
            Ok(c) => c,
            Err(e) => {
                panic!("Failed to read completion file: {}", e);
            }
        };
        let words = split_lines_concurrent(&content).await;
        let prefix_len = spell.len();
        let begin = match binary_search_match_prefix(&words, spell) {
            Some(i) => i,
            None => return Vec::new(),
        };
        let mut result = Vec::with_capacity(limit as usize);
        for i in begin..words.len() {
            if result.len() >= limit as usize {
                break;
            }
            if words[i].len() >= prefix_len && words[i].get(..prefix_len) == Some(spell) {
                result.push(words[i].to_string());
            }
        }
        result
    }
}

/// Binary search: first index where prefix of element >= target (first match position).
fn binary_search_match_prefix(words: &[String], target: &str) -> Option<usize> {
    let prefix_len = target.len();
    let mut low = 0;
    let mut high = words.len();
    while low < high {
        let mid = (low + high) / 2;
        let mid_prefix = words[mid].get(..prefix_len).unwrap_or(&words[mid]);
        if mid_prefix == target {
            high = mid;
        } else if mid_prefix < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if low < words.len() && words[low].get(..prefix_len) == Some(target) { Some(low) } else { None }
}

/// Split content by newlines, processing in chunks (similar to Swift splitConcurrently).
async fn split_lines_concurrent(content: &str) -> Vec<String> {
    const CHUNK_SIZE: usize = 50_000;
    let chunks: Vec<&str> = content.as_bytes().chunks(CHUNK_SIZE).map(|c| std::str::from_utf8(c).unwrap_or("")).collect();
    let mut parts: Vec<Vec<String>> = Vec::with_capacity(chunks.len());
    for chunk in chunks {
        let lines: Vec<String> = chunk.lines().map(|s| s.to_string()).collect();
        parts.push(lines);
    }
    parts.into_iter().flatten().collect()
}
