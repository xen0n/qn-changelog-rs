pub mod preference;


#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub format: OutputFormat,
    pub user: String,
    pub repo: String,
    pub base_branch: String,
    pub head_branch: String,
    pub dont_filter: bool,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum OutputFormat {
    Html,
    Jira,
    Markdown,
}


impl ::std::str::FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(OutputFormat::Html),
            "jira" => Ok(OutputFormat::Jira),
            "markdown" => Ok(OutputFormat::Markdown),
            _ => Err("unknown output format"),
        }
    }
}
