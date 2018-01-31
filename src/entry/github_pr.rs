use chrono;
use serde_json;

use errors::*;
use super::traits;


#[derive(Debug)]
pub struct GithubPREntry {
    number: usize,
    title: String,
    issue_numbers: Vec<String>,
    user: String,  // TODO
    merged_at: chrono::DateTime<chrono::Local>, // TODO
}


impl traits::ChangelogEntry for GithubPREntry {
    fn pr_number(&self) -> usize {
        self.number
    }

    fn title<'a>(&'a self) -> &'a str {
        &self.title
    }

    fn issue_numbers<'a>(&'a self) -> Vec<&'a str> {
        self.issue_numbers.iter().map(|x| x.as_ref()).collect()
    }

    fn user<'a>(&'a self) -> &'a str {
        &self.user
    }

    fn merged_at(&self) -> chrono::DateTime<chrono::Local> {
        self.merged_at.clone()
    }
}

// TODO: use TryFrom
impl GithubPREntry {
    pub fn from_pr_object(x: &serde_json::Value) -> Result<Self> {
        let x = x.as_object().unwrap();

        let merged_at = x["merged_at"].as_str().unwrap();
        use chrono::TimeZone;
        let merged_at = chrono::Utc.datetime_from_str(merged_at, "%+").unwrap();
        let merged_at = merged_at.with_timezone(&chrono::Local);

        Ok(Self {
            number: x["number"].as_u64().unwrap() as usize,
            title: x["title"].to_string(),
            issue_numbers: vec![],
            user: (x["user"].as_object().unwrap())["login"].to_string(),
            merged_at: merged_at,
        })
    }
}
