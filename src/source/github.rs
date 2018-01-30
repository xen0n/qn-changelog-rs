use github_rs;
use github_rs::client::Executor;
use serde_json;

use super::super::config;


// TODO: refactor to use traits
pub fn get_commits(cfg: &config::Config) {
    let client = github_rs::client::Github::new(&cfg.token).unwrap();

    let x = client.get()
        .repos()
        .owner(&cfg.user)
        .repo(&cfg.repo)
        .compare()
        .base(&cfg.base_branch)
        .head(&cfg.head_branch)
        .execute::<serde_json::Value>();

    // TODO
    if let Ok((hdrs, status, json)) = x {
        println!("{:?} {:?}", status, hdrs);
        if let Some(json) = json {
            println!("{:?}", json);
        }
    }
}
