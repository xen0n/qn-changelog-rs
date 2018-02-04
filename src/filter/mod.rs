use regex;

use super::entry;


lazy_static! {
    static ref DEPLOY_TITLE_RE: regex::Regex = regex::Regex::new(
        r"^B\d{6}",
    ).unwrap();
    static ref MASTER_DEV_RE: regex::Regex = regex::Regex::new(
        r"(?i)master\s*->\s*develop",
    ).unwrap();
    static ref DEV_MASTER_RE: regex::Regex = regex::Regex::new(
        r"(?i)develop\s*->\s*master",
    ).unwrap();
}


fn is_deploy(title: &str) -> bool {
    DEPLOY_TITLE_RE.is_match(title)
}


fn is_release_pr(title: &str) -> bool {
    MASTER_DEV_RE.is_match(title) || DEV_MASTER_RE.is_match(title)
}


pub fn should_filter<T: AsRef<entry::ChangelogEntry>>(x: T) -> bool {
    let title = x.as_ref().title();

    is_deploy(title) || is_release_pr(title)
}
