use super::super::entry;


pub trait ChangelogFormatter {
    fn format_entry<E: AsRef<entry::ChangelogEntry>>(&mut self, E) -> ::std::io::Result<()>;
}
