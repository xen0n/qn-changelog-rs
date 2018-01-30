pub mod preference;


#[derive(Debug, Deserialize)]
pub struct Config {
    token: String,
    format: OutputFormat,
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Deserialize)]
pub enum OutputFormat {
    Html,
    Markdown,
}
