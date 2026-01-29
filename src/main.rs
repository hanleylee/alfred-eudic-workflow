mod command;
mod dictionary;
mod workflow_utils;

use alfred::updater_cli::{UpdateAction, run_default_update};
use clap::{Parser, Subcommand};

use command::run_search;

const GITHUB_REPO: &str = "hanleylee/alfred-eudic-workflow";
const WORKFLOW_ASSET_NAME: &str = "EudicSearch.alfredworkflow";
const SEARCH_LIMIT: u32 = 30;

#[derive(Parser)]
#[command(name = "alfred-eudic")]
#[command(about = "Tool used to quickly search matched words by partial query")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Perform a search query
    Search {
        /// File used for completion items
        #[arg(long, env = "ALFRED_EUDIC_COMPLETION_FILE")]
        completion_file: Option<String>,

        /// Database file used for explanation (ECDICT stardict)
        #[arg(long, env = "ALFRED_EUDIC_DATABASE_FILE")]
        db_file: Option<String>,

        /// Spell of the word you want to query
        #[arg(default_value = "are")]
        spell: String,
    },
    /// Update workflow
    Update {
        #[command(subcommand)]
        action: UpdateAction,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Search { completion_file, db_file, spell } => run_search(SearchArgs {completion_file, db_file, spell }).await?,
        Commands::Update { action } => run_default_update(GITHUB_REPO, WORKFLOW_ASSET_NAME, action).await?,
    }
    Ok(())
}

pub struct SearchArgs {
    pub completion_file: Option<String>,
    pub db_file: Option<String>,
    pub spell: String,
}
