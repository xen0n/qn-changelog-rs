use std::io;

use super::super::entry;
use super::traits;

pub struct HtmlFormatter<'a> {
    w: Box<io::Write + 'a>,
}


impl<'a> HtmlFormatter<'a> {
    pub fn with_writer<W: io::Write + 'a>(w: W) -> Self {
        Self { w: Box::new(w) }
    }
}


impl<'a, C: traits::FormatterContext> traits::ChangelogFormatter<C> for HtmlFormatter<'a> {
    fn format_empty(&mut self, _: &C) -> io::Result<()> {
        writeln!(self.w, "no changelog")
    }

    fn format_prologue(&mut self, _: &C) -> io::Result<()> {
        write!(self.w, "<ul>")
    }

    fn format_epilogue(&mut self, _: &C) -> io::Result<()> {
        writeln!(self.w, "</ul>")
    }

    fn format_entry<E: AsRef<entry::ChangelogEntry>>(&mut self, _: &C, e: E) -> io::Result<()> {
        let e = e.as_ref();

        write!(self.w, "<li>[")?;

        let issues = e.issues();

        // workaround `?` not being able to propagate outside closure, and
        // `where` clauses not available to closures (?)
        macro_rules! write_link {
            ($sep: expr, $x: ident) => {
                write!(
                    self.w,
                    concat!($sep, "<a href={}>{}</a>"),
                    $x.link(),
                    $x.number()
                )?;
            };
        }

        if let Some((first_issue, rest)) = issues.split_first() {
            write_link!("", first_issue);
            for issue in rest {
                write_link!(", ", issue);
            }
        }
        writeln!(
            self.w,
            "] {} #{} (@{})</li>",
            e.title(),
            e.pr_number(),
            e.user()
        )?;

        Ok(())
    }
}
