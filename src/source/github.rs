use failure::SyncFailure;
use github_rs;
use github_rs::client::Executor;
use regex;
use serde_json;

use super::super::config;
use super::super::entry;
use crate::errors::*;


lazy_static! {
    static ref MERGE_TITLE_FORMAT_RE: regex::Regex =
        regex::Regex::new(r"^Merge pull request #(\d+)").unwrap();
}


#[derive(Deserialize)]
struct CompareCommitsResponse {
    commits: Vec<CompareCommitInfo>,
}


#[derive(Deserialize)]
struct CompareCommitInfo {
    commit: CommitInfo,
    parents: Vec<CommitParentInfo>,
}


#[derive(Deserialize)]
struct CommitInfo {
    message: String,
}


#[derive(Deserialize)]
struct CommitParentInfo {
    // XXX unused now, put here only for serde to recognize this as a struct
    #[serde(rename = "sha")]
    _sha: String,
}


pub struct GitHubSource<'a> {
    client: github_rs::client::Github,
    cfg: &'a config::Config,
}


impl<'a> GitHubSource<'a> {
    pub fn new(cfg: &'a config::Config) -> Result<Self> {
        Ok(Self {
            client: github_rs::client::Github::new(&cfg.token).map_err(SyncFailure::new)?,
            cfg: cfg,
        })
    }

    fn user(&'a self) -> &'a str {
        &self.cfg.user
    }

    fn repo(&'a self) -> &'a str {
        &self.cfg.repo
    }

    fn base_branch(&'a self) -> &'a str {
        &self.cfg.base_branch
    }

    fn head_branch(&'a self) -> &'a str {
        &self.cfg.head_branch
    }

    // TODO: refactor to use traits
    pub fn get_prs(&self) -> Result<Vec<Box<dyn entry::ChangelogEntry>>> {
        let x = self
            .client
            .get()
            .repos()
            .owner(self.user())
            .repo(self.repo())
            .compare()
            .base(self.base_branch())
            .head(self.head_branch())
            .execute::<CompareCommitsResponse>()
            .map_err(SyncFailure::new)?;

        // TODO
        let (_hdrs, _status, resp) = x;

        // println!("{:?} {:?}", status, hdrs);
        if let Some(resp) = resp {
            let pr_ids = resp
                .commits
                .into_iter()
                .map(GitHubCommit::from_compare_commit_info_object)
                .filter(|x| x.is_merge_commit())
                .map(|x| {
                    MERGE_TITLE_FORMAT_RE
                        .captures(&x.title)
                        .map(|c| (&c[1]).parse().unwrap())
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap());

            let result = pr_ids
                .map(|id| self.get_pr(id).unwrap())
                .map(|x| Box::new(x) as Box<dyn entry::ChangelogEntry>)
                .collect();

            Ok(result)
        } else {
            Err(QnChangelogError::UnexpectedInput.into())
        }
    }

    fn get_pr(&self, id: usize) -> Result<entry::GithubPREntry> {
        let x = self
            .client
            .get()
            .repos()
            .owner(self.user())
            .repo(self.repo())
            .pulls()
            .number(&format!("{}", id))
            .execute::<serde_json::Value>()
            .map_err(SyncFailure::new)?;

        let (_hdrs, _status, resp) = x;
        if let Some(resp) = resp {
            Ok(entry::GithubPREntry::from_pr_object(&resp)?)
        } else {
            Err(QnChangelogError::UnexpectedInput.into())
        }
    }
}


struct GitHubCommit {
    title: String,
    is_merge: bool,
}


impl GitHubCommit {
    fn from_compare_commit_info_object(x: CompareCommitInfo) -> Self {
        Self {
            title: x.commit.message,
            is_merge: x.parents.len() > 1,
        }
    }

    fn is_merge_commit(&self) -> bool {
        self.is_merge
    }
}
