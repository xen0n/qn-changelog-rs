pub trait ChangelogEntry: ::std::fmt::Debug {
    fn pr_number(&self) -> usize;
    fn title<'a>(&'a self) -> &'a str;
    fn issues<'a>(&'a self) -> &'a [Box<BugTrackerIssue>];
    fn user<'a>(&'a self) -> &'a str; // TODO
    fn merged_at(&self) -> ::chrono::DateTime<::chrono::Local>;
}


pub trait BugTrackerIssue: ::std::fmt::Debug {
    fn number<'a>(&'a self) -> &'a str;
    fn link<'a>(&'a self) -> &'a str;
}
