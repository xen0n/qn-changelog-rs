use std::io;

use super::super::entry;
use super::traits;

pub struct MarkdownFormatter<'a> {
    w: Box<dyn io::Write + 'a>,
}


impl<'a> MarkdownFormatter<'a> {
    pub fn with_writer<W: io::Write + 'a>(w: W) -> Self {
        Self { w: Box::new(w) }
    }
}


impl<'a, C: traits::FormatterContext> traits::ChangelogFormatter<C> for MarkdownFormatter<'a> {
    fn format_empty(&mut self, _: &C) -> io::Result<()> {
        writeln!(self.w, "no changelog")
    }

    fn format_prologue(&mut self, _: &C) -> io::Result<()> {
        Ok(())
    }

    fn format_epilogue(&mut self, _: &C) -> io::Result<()> {
        Ok(())
    }

    fn format_entry<E: AsRef<dyn entry::ChangelogEntry>>(&mut self, _: &C, e: E) -> io::Result<()> {
        let e = e.as_ref();

        write!(self.w, "* [")?;

        let issues = e.issues();

        // workaround `?` not being able to propagate outside closure, and
        // `where` clauses not available to closures (?)
        macro_rules! write_link {
            ($sep: expr, $x: ident) => {
                write!(self.w, concat!($sep, "[{}]({})"), $x.number(), $x.link())?;
            };
        }

        if let Some((first_issue, rest)) = issues.split_first() {
            write_link!("", first_issue);
            for issue in rest {
                write_link!(",", issue);
            }
        }
        writeln!(self.w, "] {} #{} (@{})", e.title(), e.pr_number(), e.user())?;

        Ok(())
    }
}
