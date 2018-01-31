use regex;

use super::traits;


#[derive(Debug)]
pub struct CommonIssue {
    number: String,
    link: String,
}


impl traits::BugTrackerIssue for CommonIssue {
    fn number<'a>(&'a self) -> &'a str {
        &self.number
    }

    fn link<'a>(&'a self) -> &'a str {
        &self.link
    }
}


lazy_static! {
    static ref JIRA_ISSUE_RE: regex::Regex = regex::Regex::new(
        r"(?P<link>https?://jira\.[^/]+/browse/(?P<number>[0-9A-Za-z-]+))",
        ).unwrap();
}


impl CommonIssue {
    pub fn new<N, L>(number: N, link: L) -> Self
    where
        N: AsRef<str>,
        L: AsRef<str>,
    {
        Self {
            number: number.as_ref().to_owned(),
            link: link.as_ref().to_owned(),
        }
    }

    pub fn parse_all_from_body(body: &str) -> Vec<Self> {
        JIRA_ISSUE_RE
            .captures_iter(body)
            .map(|c| Self::new(&c["number"], &c["link"]))
            .collect()
    }
}
