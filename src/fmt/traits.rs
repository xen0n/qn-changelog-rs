use super::super::entry;


pub trait ChangelogFormatter {
    fn format_entry<E: entry::ChangelogEntry>(&mut self, E);
}
