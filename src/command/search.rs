use std::process::Command;
use std::time::{Duration, Instant};
use alfred::core::{AlfredConst, AlfredUtils};
use alfred::script_filter::{Item, Mod, ScriptFilter, Variable};
use alfred::updater::{Updater, version_compare};
use crate::dictionary::{DictionaryConfig, DictionaryManager};
use crate::{SEARCH_LIMIT, SearchArgs, workflow_utils};

pub async fn run_search(args: SearchArgs) -> Result<(), Box<dyn std::error::Error>> {
    ScriptFilter::reset();

    if args.spell.len() <= 1 {
        ScriptFilter::item(Item::new("Input more than one letter"));
        AlfredUtils::output(ScriptFilter::output());
        return Ok(());
    }

    let t1 = Instant::now();
    let config = DictionaryConfig::new(args.completion_file.clone(), args.db_file.clone());
    let manager = DictionaryManager::new(config);

    let mut items: Vec<Item> = Vec::new();
    items.push(Item::new(&args.spell).arg(&args.spell).subtitle("Type enter to check in Eudic"));

    if let Some(ref db_file) = args.db_file {
        if !db_file.is_empty() && std::path::Path::new(db_file).exists() {
            let spell: String = args.spell.split_whitespace().collect();
            let matches = manager.find_matches_in_db(&spell, SEARCH_LIMIT);
            for entry in matches {
                let explanation = entry.translation.as_ref().or(entry.definition.as_ref()).map(|s| s.replace('\n', "; ")).unwrap_or_default();
                let phonetic = entry.phonetic.as_deref().unwrap_or("");
                let collins_rate = "⭐️".repeat(entry.collins.unwrap_or(0) as usize);
                let mut importance_info: Vec<String> = Vec::new();
                if let Some(c) = entry.collins {
                    importance_info.push(format!("COLLINS: {}", "⭐️".repeat(c as usize)));
                }
                if entry.oxford.is_some() {
                    importance_info.push("OXFORD 3000".to_string());
                }
                if let Some(bnc) = entry.bnc {
                    if bnc != 0 {
                        importance_info.push(format!("BNC: {}", bnc));
                    }
                }
                if let Some(frq) = entry.frq {
                    if frq != 0 {
                        importance_info.push(format!("COCA: {}", frq));
                    }
                }
                if let Some(tag_info) = entry.tag_info() {
                    importance_info.push(tag_info);
                }
                let title = workflow_utils::aligned_text(&entry.word, &collins_rate);
                let subtitle = workflow_utils::aligned_text(&explanation, phonetic);
                let cmd_subtitle = entry.exchange_info().unwrap_or_default();
                let alt_subtitle = importance_info.join("; ");
                items.push(
                    Item::new(title)
                        .subtitle(subtitle)
                        .arg(&entry.word)
                        .cmd(Mod::new().subtitle(cmd_subtitle))
                        .alt(Mod::new().subtitle(alt_subtitle)),
                );
            }
        } else {
            items.push(Item::new(format!("db_file not exist: {}", db_file)));
        }
    } else if let Some(completion_file) = args.completion_file {
        if !completion_file.is_empty() && std::path::Path::new(&completion_file).exists() {
            let matches = manager.find_matches_in_completion(&completion_file, &args.spell, SEARCH_LIMIT).await;
            for word in matches {
                items.push(Item::new(&word).arg(&word));
            }
        }else {
            items.push(Item::new(format!("completion_file not exist: {}", completion_file)));
        }
    }

    for item in items {
        ScriptFilter::item(item);
    }

    let t2 = Instant::now();
    AlfredUtils::log(format!("search time duration: {:?}", t2 - t1));

    let updater = Updater::new(crate::GITHUB_REPO, crate::WORKFLOW_ASSET_NAME, Duration::from_secs(60 * 60 * 24));
    let alfred = AlfredConst::shared();
    if let Some(cached) = updater.read_cached_release().await.ok().and_then(|o| o) {
        if updater.cache_valid(&cached) {
            if let Some(ref current_version) = alfred.workflow_version {
                if version_compare(current_version, &cached.tag_name) == std::cmp::Ordering::Less {
                    ScriptFilter::item(
                        Item::new("New version available on GitHub, type [Enter] to update")
                            .subtitle(format!("current version: {}, remote version: {}", current_version, cached.tag_name))
                            .arg("update")
                            .variable(Variable::new(Some("HAS_UPDATE".into()), Some("1".into()))),
                    );
                }
            }
        }
    }

    AlfredUtils::output(ScriptFilter::output());

    if let Some(cached) = updater.read_cached_release().await.ok().and_then(|o| o) {
        if !updater.cache_valid(&cached) {
            AlfredUtils::log("cache invalid");
            check_for_update_silently();
        }
    } else {
        check_for_update_silently();
    }

    Ok(())
}

fn check_for_update_silently() {
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return,
    };
    let status = Command::new("/usr/bin/nohup")
        .arg(&exe)
        .args(["update", "--action", "check"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    match status {
        Ok(_) => AlfredUtils::log("Update check completed in the background"),
        Err(e) => AlfredUtils::log(format!("Failed to start update process: {}", e)),
    }
}
