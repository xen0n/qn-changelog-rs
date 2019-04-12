use chrono;
use serde_json;

use crate::errors::*;
use super::issues;
use super::traits;


#[derive(Debug)]
pub struct GithubPREntry {
    number: usize,
    title: String,
    issues: Vec<Box<traits::BugTrackerIssue>>,
    user: String,                               // TODO
    merged_at: chrono::DateTime<chrono::Local>, // TODO
}


impl traits::ChangelogEntry for GithubPREntry {
    fn pr_number(&self) -> usize {
        self.number
    }

    fn title<'a>(&'a self) -> &'a str {
        &self.title
    }

    fn issues<'a>(&'a self) -> &'a [Box<traits::BugTrackerIssue>] {
        &self.issues
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

        let title = x["title"].as_str().unwrap();
        let body = x["body"].as_str().unwrap();
        let issues_title = issues::CommonIssue::parse_all_from_title(title);
        // TODO: merge both
        let issues = if issues_title.len() > 0 {
            issues_title
        } else {
            issues::CommonIssue::parse_all_from_body(body)
        };
        let issues = issues
            .into_iter()
            .map(|x| Box::new(x) as Box<traits::BugTrackerIssue>)
            .collect();

        let merged_at = x["merged_at"].as_str().unwrap();
        use chrono::TimeZone;
        let merged_at = chrono::Utc.datetime_from_str(merged_at, "%+").unwrap();
        let merged_at = merged_at.with_timezone(&chrono::Local);

        Ok(Self {
            number: x["number"].as_u64().unwrap() as usize,
            title: title.to_owned(),
            issues: issues,
            user: (x["user"].as_object().unwrap())["login"]
                .as_str()
                .unwrap()
                .to_owned(),
            merged_at: merged_at,
        })
    }
}
