use octocrab;
use regex;

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
    client: octocrab::Octocrab,
    cfg: &'a config::Config,
}


impl<'a> GitHubSource<'a> {
    pub fn new(cfg: &'a config::Config) -> Result<Self> {
        let oc = octocrab::OctocrabBuilder::new()
            .personal_token(cfg.token.clone())
            .build()?;

        Ok(Self {
            client: oc,
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
    pub async fn get_prs(&self) -> Result<Vec<Box<dyn entry::ChangelogEntry>>> {
        let url = format!(
            "repos/{}/{}/compare/{}...{}",
            self.user(),
            self.repo(),
            self.base_branch(),
            self.head_branch(),
        );

        match self.client.get(&url, None::<&()>).await {
            Ok(resp) => {
                let resp: CompareCommitsResponse = resp;
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

                let prs: Vec<entry::GithubPREntry> = {
                    let mut tmp = Vec::new();
                    for pr_id in pr_ids {
                        let pr = self.get_pr(pr_id).await.unwrap();
                        tmp.push(pr);
                    }
                    tmp
                };

                let result = prs
                    .into_iter()
                    .map(|x| Box::new(x) as Box<dyn entry::ChangelogEntry>)
                    .collect();

                Ok(result)
            }

            Err(_) => Err(QnChangelogError::UnexpectedInput.into()),
        }
    }

    async fn get_pr(&self, id: usize) -> Result<entry::GithubPREntry> {
        let url = format!("/repos/{}/{}/pulls/{}", self.user(), self.repo(), id,);

        match self.client.get(url, None::<&()>).await {
            Ok(resp) => Ok(entry::GithubPREntry::from_pr_object(&resp)?),
            Err(_) => Err(QnChangelogError::UnexpectedInput.into()),
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
