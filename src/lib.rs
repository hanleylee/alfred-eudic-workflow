pub mod command;
pub mod dictionary;
pub mod workflow_utils;

pub use command::run_search;

pub const GITHUB_REPO: &str = "hanleylee/alfred-eudic-workflow";
pub const WORKFLOW_ASSET_NAME: &str = "EudicSearch.alfredworkflow";
pub const SEARCH_LIMIT: u32 = 30;

pub struct SearchArgs {
    pub completion_file: Option<String>,
    pub db_file: Option<String>,
    pub spell: String,
}
