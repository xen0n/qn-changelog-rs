use github_rs;

use super::super::config;


// TODO: refactor to use traits
pub fn get_commits(cfg: &config::Config) {
    let client = github_rs::client::Github::new(&cfg.token).unwrap();

    // TODO: compare is not implemented, 囍
    let x = client.get()
        .repos()
        .owner(&cfg.user)
        .repo(&cfg.repo)
        .compare()
        .base(&cfg.base_branch)
        .head(&cfg.head_branch)
        .execute();

    println!("{:?}", x);
}
