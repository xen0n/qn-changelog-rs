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
