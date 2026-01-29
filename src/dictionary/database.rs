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

    /// Search for words by spell(s). Single spell: prefix match on `sw`. Multiple: LIKE %spell% each.
    pub fn search_word(&self, spell: &str, limit: u32) -> Result<Vec<StardictEntry>, rusqlite::Error> {
        if spell.is_empty() {
            return Ok(Vec::new());
        }

        let limit_i = limit.min(1000);
        let sql = format!("
        SELECT id, word, sw, phonetic, definition, translation, pos, collins, oxford, tag, bnc, frq, exchange, detail, audio
        FROM {TABLE}
        WHERE sw LIKE '{spell}%'
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
