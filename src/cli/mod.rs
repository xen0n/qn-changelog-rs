use docopt;

use super::config;
use super::filter;
use super::fmt;
use super::source;


const USAGE: &'static str = "
Qiniu changelog generator (Rust port).

Usage:
  qn-changelog [options] [base] [head]
  qn-changelog (-h | --help)

<base> and <head> can be branch name or tag or commit hash

Options:
  -u USER, --user USER      GitHub user [default: qbox]
  -r REPO, --repo REPO      GitHub repo name [default: portal-v4]
  -t TOKEN, --token TOKEN   GitHub access token
  -a, --all                 show all pull-request, not filter deploy pr
  --before TIME             filter changelog before time
  --after TIME              filter changelog after time
  -f FMT, --format FMT      result format [default: markdown]
                            supported formats: html, jira, markdown
  -c, --copy                also copy results to system clipboard
                            (feature=clipboard builds only)
  -h, --help                Show help
";


#[derive(Debug, Deserialize)]
struct Args {
    arg_base: Option<String>,
    arg_head: Option<String>,

    flag_user: String,
    flag_repo: String,
    flag_token: Option<String>,
    flag_all: bool,
    flag_before: Option<String>,
    flag_after: Option<String>,
    flag_format: config::OutputFormat,
    flag_copy: bool,
}


pub(crate) fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    // println!("{:?}", args);

    let prefs = config::preference::UserPreference::load().unwrap();
    let (token, should_update_token) = match (prefs.token(), args.flag_token) {
        (_, Some(t)) => (t, true),
        (Some(t), None) => (t.to_string(), false),
        (None, None) => {
            // TODO
            panic!("token must be configured or specified");
        }
    };

    if should_update_token {
        let mut new_prefs = prefs.clone();
        new_prefs.set_token(&token);
        new_prefs.save().unwrap();
    }

    let (base, head) = {
        let base = args.arg_base.unwrap_or("master".to_owned());
        let head = args.arg_head.unwrap_or("develop".to_owned());
        (base, head)
    };

    let cfg = config::Config {
        token: token.to_string(),
        format: args.flag_format,
        user: args.flag_user,
        repo: args.flag_repo,
        base_branch: base,
        head_branch: head,
        dont_filter: args.flag_all,
    };

    // println!("{:?}", cfg);

    // TODO
    let src = source::GitHubSource::new(&cfg).unwrap();
    let prs = src.get_prs().unwrap();
    let entries: Vec<_> = prs
        .into_iter()
        .filter(|x| !filter::should_filter(&cfg, x))
        .collect();

    let output_buf = {
        use bytes::BufMut;

        let buf = Vec::new();
        let mut writer = buf.writer();

        // ChangelogFormatter is not object-safe, so we can't do `Box::new(sink)`
        // and reuse code
        use fmt::ChangelogFormatter;
        match cfg.format {
            config::OutputFormat::Html => {
                let mut sink = fmt::HtmlFormatter::with_writer(&mut writer);
                sink.format(&prefs, &entries).unwrap();
            }
            config::OutputFormat::Jira => {
                let mut sink = fmt::JiraFormatter::with_writer(&mut writer);
                sink.format(&prefs, &entries).unwrap();
            }
            config::OutputFormat::Markdown => {
                let mut sink = fmt::MarkdownFormatter::with_writer(&mut writer);
                sink.format(&prefs, &entries).unwrap();
            }
        }

        writer.into_inner()
    };

    // output to stdout
    let mut stdout = ::std::io::stdout();
    use ::std::io::Write;
    stdout.write_all(&output_buf).unwrap();

    // copy to clipboard
    if args.flag_copy {
        copy_to_clipboard(&output_buf);
    }
}

#[cfg(not(feature = "clipboard"))]
fn copy_to_clipboard<T: AsRef<[u8]>>(content: T) {
    // clipboard feature is disabled, nothing to do
    // TODO: print warning?
}

#[cfg(feature = "clipboard")]
fn copy_to_clipboard<T: AsRef<[u8]>>(content: T) {
    use ::clipboard::ClipboardProvider;
    use ::clipboard::ClipboardContext;

    let content = content.as_ref();
    let content = String::from_utf8_lossy(content);

    let content_to_copy = content.into_owned();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content_to_copy).unwrap();
}
