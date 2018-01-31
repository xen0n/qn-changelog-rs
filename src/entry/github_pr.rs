use serde_json;

use errors::*;
use super::traits;


#[derive(Debug)]
pub struct GithubPREntry {
    number: usize,
    title: String,
    issue_numbers: Vec<String>,
    user: String,  // TODO
    merged_at: (), // TODO
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

    fn merged_at(&self) -> () {
        ()
    }
}

// TODO: use TryFrom
impl GithubPREntry {
    pub fn from_pr_object(x: &serde_json::Value) -> Result<Self> {
        let x = x.as_object().unwrap();

        Ok(Self {
            number: x["number"].as_u64().unwrap() as usize,
            title: x["title"].to_string(),
            issue_numbers: vec![],
            user: (x["user"].as_object().unwrap())["login"].to_string(),
            merged_at: (),
        })
    }
}
