use github_rs;
use github_rs::client::Executor;
use regex;

use errors::*;
use super::super::config;


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


// TODO: refactor to use traits
pub fn get_pr_ids(cfg: &config::Config) -> Result<Vec<usize>> {
    let client = github_rs::client::Github::new(&cfg.token)?;

    let x = client
        .get()
        .repos()
        .owner(&cfg.user)
        .repo(&cfg.repo)
        .compare()
        .base(&cfg.base_branch)
        .head(&cfg.head_branch)
        .execute::<CompareCommitsResponse>()?;

    // TODO
    let (_hdrs, _status, resp) = x;

    // println!("{:?} {:?}", status, hdrs);
    if let Some(resp) = resp {
        let result = resp.commits
            .into_iter()
            .map(GitHubCommit::from_compare_commit_info_object)
            .filter(|x| x.is_merge_commit())
            .map(|x| {
                MERGE_TITLE_FORMAT_RE
                    .captures(&x.title)
                    .map(|c| (&c[1]).parse().unwrap())
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        Ok(result)
    } else {
        Err(ErrorKind::UnexpectedInput.into())
    }
}


pub struct GitHubCommit {
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
