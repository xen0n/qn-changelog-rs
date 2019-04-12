use super::super::entry;


pub trait FormatterContext {
    fn github_id_to_ldap<T: AsRef<str>>(&self, github_id: T) -> Option<String>;
}


pub trait ChangelogFormatter<C: FormatterContext> {
    fn format_empty(&mut self, fcx: &C) -> ::std::io::Result<()>;
    fn format_prologue(&mut self, fcx: &C) -> ::std::io::Result<()>;
    fn format_epilogue(&mut self, fcx: &C) -> ::std::io::Result<()>;
    fn format_entry<E: AsRef<entry::ChangelogEntry>>(&mut self, fcx: &C, entry: E) -> ::std::io::Result<()>;

    fn format<E: AsRef<entry::ChangelogEntry>>(&mut self, ctx: &C, entries: &[E]) -> ::std::io::Result<()> {
        if entries.len() == 0 {
            return self.format_empty(ctx);
        }

        self.format_prologue(ctx)?;
        for e in entries {
            self.format_entry(ctx, e)?;
        }
        self.format_epilogue(ctx)?;

        Ok(())
    }
}
