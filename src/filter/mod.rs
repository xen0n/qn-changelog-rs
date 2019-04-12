use regex;

use super::config;
use super::entry;


lazy_static! {
    static ref DEPLOY_TITLE_RE: regex::Regex = regex::Regex::new(r"^B\d{6}",).unwrap();
    static ref MASTER_DEV_RE: regex::Regex =
        regex::Regex::new(r"(?i)master\s*->\s*develop",).unwrap();
    static ref DEV_MASTER_RE: regex::Regex =
        regex::Regex::new(r"(?i)develop\s*->\s*master",).unwrap();
    static ref QA_MASTER_DEV_RE: regex::Regex =
        regex::Regex::new(r"(?i)qamaster\s*->\s*qa",).unwrap();
    static ref QA_DEV_MASTER_RE: regex::Regex =
        regex::Regex::new(r"(?i)qa\s*->\s*qamaster",).unwrap();
}


fn is_deploy(title: &str) -> bool {
    DEPLOY_TITLE_RE.is_match(title)
}


fn is_release_pr(title: &str) -> bool {
    MASTER_DEV_RE.is_match(title) || DEV_MASTER_RE.is_match(title)
}


fn is_qa_release_pr(title: &str) -> bool {
    QA_MASTER_DEV_RE.is_match(title) || QA_DEV_MASTER_RE.is_match(title)
}


pub fn should_filter<T: AsRef<entry::ChangelogEntry>>(cfg: &config::Config, x: T) -> bool {
    if cfg.dont_filter {
        false
    } else {
        let title = x.as_ref().title();

        is_deploy(title) || is_release_pr(title) || is_qa_release_pr(title)
    }
}
