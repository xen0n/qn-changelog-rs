use super::super::entry;


pub trait ChangelogFormatter {
    fn format_entry<E: AsRef<entry::ChangelogEntry>>(&mut self, E) -> ::std::io::Result<()>;

    fn format<E: AsRef<entry::ChangelogEntry>>(&mut self, entries: &[E]) -> ::std::io::Result<()> {
        for e in entries {
            self.format_entry(e)?;
        }

        Ok(())
    }
}
