//! CLI integration tests: run the `alfred-eudic` binary and assert Script Filter JSON.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;
use serde_json::Value;

fn bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_alfred-eudic"));
    // Ignore host env so local Alfred workflow settings do not affect tests.
    cmd.env_remove("ALFRED_EUDIC_COMPLETION_FILE");
    cmd.env_remove("ALFRED_EUDIC_DATABASE_FILE");
    cmd
}

fn unique_temp(prefix: &str, ext: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{nanos}-{n}.{ext}"))
}

fn write_temp_file(content: &str, ext: &str) -> PathBuf {
    let path = unique_temp("alfred-eudic-it", ext);
    let mut file = std::fs::File::create(&path).expect("create temp file");
    file.write_all(content.as_bytes()).expect("write temp file");
    path
}

fn create_stardict_db(rows: &[(&str, &str, Option<&str>)]) -> PathBuf {
    let path = unique_temp("alfred-eudic-it", "sqlite");
    let conn = Connection::open(&path).expect("open sqlite");
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
    .expect("create schema");
    for (i, (word, sw, translation)) in rows.iter().enumerate() {
        conn.execute(
            "INSERT INTO stardict (id, word, sw, translation, phonetic, collins, tag, exchange)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                i as i64 + 1,
                word,
                sw,
                translation,
                "ˈæpl",
                3,
                "cet4",
                "s:apples"
            ],
        )
        .expect("insert row");
    }
    path
}

fn run_search(args: &[&str]) -> (String, String, i32) {
    let output = bin().args(args).output().expect("run alfred-eudic");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let code = output.status.code().unwrap_or(-1);
    (stdout, stderr, code)
}

/// Alfred logs may share stdout with Script Filter JSON; take the last JSON object line.
fn parse_script_filter(stdout: &str) -> Value {
    let json_line = stdout
        .lines()
        .rev()
        .find(|line| line.trim_start().starts_with('{'))
        .unwrap_or_else(|| panic!("no JSON Script Filter output in stdout:\n{stdout}"));
    serde_json::from_str(json_line).unwrap_or_else(|e| panic!("invalid JSON `{json_line}`: {e}"))
}

fn item_titles(filter: &Value) -> Vec<String> {
    filter["items"]
        .as_array()
        .expect("items array")
        .iter()
        .map(|item| item["title"].as_str().unwrap_or("").to_string())
        .collect()
}

fn cleanup(paths: &[&Path]) {
    for path in paths {
        let _ = std::fs::remove_file(path);
    }
}

#[test]
fn search_rejects_single_letter_query() {
    let (stdout, _stderr, code) = run_search(&["search", "a"]);
    assert_eq!(code, 0);
    let filter = parse_script_filter(&stdout);
    assert_eq!(item_titles(&filter), vec!["Input more than one letter"]);
}

#[test]
fn search_reports_missing_db_file() {
    let missing = unique_temp("alfred-eudic-missing", "sqlite");
    let missing_str = missing.to_string_lossy().to_string();
    let (stdout, _stderr, code) = run_search(&["search", "--db-file", &missing_str, "hello"]);
    assert_eq!(code, 0);
    let titles = item_titles(&parse_script_filter(&stdout));
    assert!(titles.iter().any(|t| t.contains("db_file not exist")));
    assert!(titles.iter().any(|t| t == "hello"));
}

#[test]
fn search_reports_missing_completion_file() {
    let missing = unique_temp("alfred-eudic-missing", "txt");
    let missing_str = missing.to_string_lossy().to_string();
    let (stdout, _stderr, code) = run_search(&["search", "--completion-file", &missing_str, "hello"]);
    assert_eq!(code, 0);
    let titles = item_titles(&parse_script_filter(&stdout));
    assert!(titles.iter().any(|t| t.contains("completion_file not exist")));
}

#[test]
fn search_completion_file_returns_prefix_matches() {
    let path = write_temp_file("apple\napply\napplication\nbanana\n", "txt");
    let path_str = path.to_string_lossy().to_string();
    let (stdout, _stderr, code) = run_search(&["search", "--completion-file", &path_str, "app"]);
    cleanup(&[&path]);
    assert_eq!(code, 0);
    let titles = item_titles(&parse_script_filter(&stdout));
    assert!(titles.iter().any(|t| t == "apple"));
    assert!(titles.iter().any(|t| t == "apply"));
    assert!(titles.iter().any(|t| t == "application"));
    // "app" itself is not an exact dictionary hit, so a fallback item is prepended.
    assert_eq!(titles.first().map(String::as_str), Some("app"));
}

#[test]
fn search_db_file_returns_matches_with_translation() {
    let db = create_stardict_db(&[("apple", "apple", Some("苹果")), ("apply", "apply", Some("申请")), ("banana", "banana", Some("香蕉"))]);
    let db_str = db.to_string_lossy().to_string();
    let (stdout, _stderr, code) = run_search(&["search", "--db-file", &db_str, "app"]);
    cleanup(&[&db]);
    assert_eq!(code, 0);
    let filter = parse_script_filter(&stdout);
    let items = filter["items"].as_array().expect("items");
    let apple = items.iter().find(|item| item["arg"].as_str() == Some("apple")).expect("apple item");
    assert!(apple["title"].as_str().unwrap_or("").contains("apple"));
    assert!(apple["subtitle"].as_str().unwrap_or("").contains("苹果"));
}

#[test]
fn search_exact_db_match_skips_fallback_item() {
    let db = create_stardict_db(&[("go", "go", Some("去"))]);
    let db_str = db.to_string_lossy().to_string();
    let (stdout, _stderr, code) = run_search(&["search", "--db-file", &db_str, "go"]);
    cleanup(&[&db]);
    assert_eq!(code, 0);
    let filter = parse_script_filter(&stdout);
    let first = &filter["items"][0];
    assert_eq!(first["arg"].as_str(), Some("go"));
    assert_ne!(first["subtitle"].as_str(), Some("Type enter to check in Eudic"));
    assert!(!filter["items"].as_array().unwrap().iter().any(|item| item["subtitle"].as_str() == Some("Type enter to check in Eudic")));
}
