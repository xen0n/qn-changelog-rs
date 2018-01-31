pub trait ChangelogEntry: ::std::fmt::Debug {
    fn pr_number(&self) -> usize;
    fn title<'a>(&'a self) -> &'a str;
    fn issue_numbers<'a>(&'a self) -> Vec<&'a str>; // TODO: I don't want Vec's
    fn user<'a>(&'a self) -> &'a str; // TODO
    fn merged_at(&self) -> (); // TODO
}
