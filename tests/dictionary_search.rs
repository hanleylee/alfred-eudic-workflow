//! Library integration tests for dictionary search across real temp files.

use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use alfred_eudic::dictionary::{DictionaryConfig, DictionaryManager};
use rusqlite::Connection;

fn unique_temp(prefix: &str, ext: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{nanos}-{n}.{ext}"))
}

fn write_completion(content: &str) -> PathBuf {
    let path = unique_temp("alfred-eudic-dict", "txt");
    let mut file = std::fs::File::create(&path).expect("create completion file");
    file.write_all(content.as_bytes()).expect("write completion");
    path
}

fn create_db(rows: &[(&str, &str, &str)]) -> PathBuf {
    let path = unique_temp("alfred-eudic-dict", "sqlite");
    let conn = Connection::open(&path).expect("open db");
    conn.execute_batch(
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
    .expect("schema");
    for (i, (word, sw, translation)) in rows.iter().enumerate() {
        conn.execute(
            "INSERT INTO stardict (id, word, sw, translation) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![i as i64 + 1, word, sw, translation],
        )
        .expect("insert");
    }
    path
}

#[tokio::test]
async fn manager_searches_completion_and_database_together_via_config() {
    let completion = write_completion("cat\ncatch\ncategory\ndog\n");
    let db = create_db(&[("cat", "cat", "猫"), ("catch", "catch", "抓住"), ("dog", "dog", "狗")]);

    let manager = DictionaryManager::new(DictionaryConfig::new(
        Some(completion.to_string_lossy().into_owned()),
        Some(db.to_string_lossy().into_owned()),
    ));

    let completion_hits = manager.find_matches_in_completion(completion.to_str().unwrap(), "cat", 10).await;
    let db_hits = manager.find_matches_in_db("cat", 10);

    let _ = std::fs::remove_file(&completion);
    let _ = std::fs::remove_file(&db);

    assert_eq!(completion_hits, vec!["cat", "catch", "category"]);
    assert_eq!(db_hits.iter().map(|e| e.word.as_str()).collect::<Vec<_>>(), vec!["cat", "catch"]);
    assert_eq!(db_hits[0].translation.as_deref(), Some("猫"));
}

#[tokio::test]
async fn manager_returns_empty_when_db_path_missing() {
    let missing = unique_temp("alfred-eudic-missing-db", "sqlite");
    let manager = DictionaryManager::new(DictionaryConfig::new(None, Some(missing.to_string_lossy().into_owned())));
    assert!(manager.find_matches_in_db("anything", 10).is_empty());
}

#[test]
fn manager_respects_search_limit_from_database() {
    let db = create_db(&[
        ("test", "test", "t1"),
        ("testament", "testament", "t2"),
        ("tester", "tester", "t3"),
        ("testing", "testing", "t4"),
    ]);
    let manager = DictionaryManager::new(DictionaryConfig::new(None, Some(db.to_string_lossy().into_owned())));
    let hits = manager.find_matches_in_db("test", 2);
    let _ = std::fs::remove_file(&db);
    assert_eq!(hits.len(), 2);
    assert_eq!(hits[0].word, "test");
}
