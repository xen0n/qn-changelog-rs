pub mod preference;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub format: OutputFormat,
    pub user: String,
    pub repo: String,
    pub base_branch: String,
    pub head_branch: String,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum OutputFormat {
    Html,
    Markdown,
}
