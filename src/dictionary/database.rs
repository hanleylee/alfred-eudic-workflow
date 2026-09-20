use alfred::core::AlfredUtils;
use rusqlite::Connection;

use super::entry::StardictEntry;

const TABLE: &str = "stardict";

pub struct StardictDatabase {
    conn: Connection,
}

impl StardictDatabase {
    pub fn new(database_path: &str) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(database_path)?;
        AlfredUtils::log(format!("Connected to database at {}", database_path));
        Ok(Self { conn })
    }

    /// Search for words by spell: prefix match on `sw`, exact `word` match first, then by length and alphabet.
    pub fn search_word(&self, spell: &str, limit: u32) -> Result<Vec<StardictEntry>, rusqlite::Error> {
        if spell.is_empty() {
            return Ok(Vec::new());
        }

        let limit_i = limit.min(1000);
        let sql = format!("
        SELECT id, word, sw, phonetic, definition, translation, pos, collins, oxford, tag, bnc, frq, exchange, detail, audio
        FROM {TABLE}
        WHERE sw LIKE '{spell}%'
        ORDER BY (word = '{spell}') DESC, length(word), word
        LIMIT {limit_i}
        ",
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let mut rows = stmt.raw_query();
        let mut entries = Vec::new();
        while let Some(row) = rows.next()? {
            entries.push(row_to_entry(&row)?);
        }
        Ok(entries)
    }
}

fn row_to_entry(row: &rusqlite::Row<'_>) -> Result<StardictEntry, rusqlite::Error> {
    Ok(StardictEntry {
        id: row.get(0)?,
        word: row.get(1)?,
        sw: row.get(2)?,
        phonetic: row.get(3)?,
        definition: row.get(4)?,
        translation: row.get(5)?,
        pos: row.get(6)?,
        collins: row.get(7)?,
        oxford: row.get(8)?,
        tag: row.get(9)?,
        bnc: row.get(10)?,
        frq: row.get(11)?,
        exchange: row.get(12)?,
        detail: row.get(13)?,
        audio: row.get(14)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memory_db_with_rows(rows: &[(&str, &str)]) -> StardictDatabase {
        let db = StardictDatabase { conn: Connection::open_in_memory().unwrap() };
        db.conn
            .execute_batch(
                "CREATE TABLE stardict (
                    id INTEGER PRIMARY KEY,
                    word TEXT NOT NULL,
                    sw TEXT NOT NULL,
                    phonetic TEXT,
                    definition TEXT,
                    translation TEXT,
                    pos TEXT,
                    collins INTEGER,
                    oxford INTEGER,
                    tag TEXT,
                    bnc INTEGER,
                    frq INTEGER,
                    exchange TEXT,
                    detail TEXT,
                    audio TEXT
                );",
            )
            .unwrap();
        for (i, (word, sw)) in rows.iter().enumerate() {
            db.conn
                .execute(
                    "INSERT INTO stardict (id, word, sw, translation) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![i as i64 + 1, word, sw, format!("tr-{word}")],
                )
                .unwrap();
        }
        db
    }

    #[test]
    fn search_word_returns_empty_for_empty_spell() {
        let db = memory_db_with_rows(&[("apple", "apple")]);
        assert!(db.search_word("", 10).unwrap().is_empty());
    }

    #[test]
    fn search_word_prefix_match_prefers_exact_then_shorter() {
        let db = memory_db_with_rows(&[("app", "app"), ("apple", "apple"), ("application", "application"), ("banana", "banana")]);
        let words: Vec<_> = db.search_word("app", 10).unwrap().into_iter().map(|e| e.word).collect();
        assert_eq!(words, vec!["app", "apple", "application"]);
    }

    #[test]
    fn search_word_respects_limit() {
        let db = memory_db_with_rows(&[("cat", "cat"), ("catch", "catch"), ("category", "category")]);
        let words: Vec<_> = db.search_word("cat", 2).unwrap().into_iter().map(|e| e.word).collect();
        assert_eq!(words, vec!["cat", "catch"]);
    }

    #[test]
    fn search_word_maps_row_fields() {
        let db = memory_db_with_rows(&[("go", "go")]);
        let entry = &db.search_word("go", 1).unwrap()[0];
        assert_eq!(entry.word, "go");
        assert_eq!(entry.sw, "go");
        assert_eq!(entry.translation.as_deref(), Some("tr-go"));
    }
}
