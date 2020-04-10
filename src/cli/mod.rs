use structopt::StructOpt;

use super::config;
use super::filter;
use super::fmt;
use super::source;

#[derive(Debug, StructOpt)]
#[structopt(name = "qn-changelog-rs")]
struct Args {
    /// GitHub user
    #[structopt(short, long, default_value = "qbox")]
    user: String,
    /// GitHub repo name
    #[structopt(short, long, default_value = "portal-v4")]
    repo: String,
    /// GitHub access token
    #[structopt(short, long)]
    token: Option<String>,
    /// show all pull-request, not filter deploy pr
    #[structopt(short, long)]
    all: bool,
    /// result format
    #[structopt(short, long, parse(try_from_str), default_value = "markdown")]
    format: config::OutputFormat,
    /// consider base/head to be swapped, useful for hotfixes
    #[structopt(short = "x", long)]
    hotfix: bool,
    /// also copy results to system clipboard
    #[cfg(feature = "clipboard")]
    #[structopt(short, long)]
    copy: bool,

    #[structopt(default_value = "master")]
    base: String,
    #[structopt(default_value = "develop")]
    head: String,
}

pub(crate) fn main() {
    let args = Args::from_args();

    // println!("{:?}", args);

    let prefs = config::preference::UserPreference::load().unwrap();
    let (token, should_update_token) = match (prefs.token(), args.token) {
        (_, Some(t)) => (t.to_string(), true),
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

    let (base, head) = (args.base, args.head);
    let (base, head) = if args.hotfix {
        (head, base)
    } else {
        (base, head)
    };

    let cfg = config::Config {
        token: token.to_string(),
        format: args.format,
        user: args.user,
        repo: args.repo,
        base_branch: base,
        head_branch: head,
        dont_filter: args.all,
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
    if cfg!(feature = "clipboard") {
        if args.copy {
            copy_to_clipboard(&output_buf);
        }
    }
}

#[cfg(not(feature = "clipboard"))]
fn copy_to_clipboard<T: AsRef<[u8]>>(_content: T) {
    // clipboard feature is disabled, nothing to do
}

#[cfg(feature = "clipboard")]
fn copy_to_clipboard<T: AsRef<[u8]>>(content: T) {
    use ::clipboard::ClipboardContext;
    use ::clipboard::ClipboardProvider;

    let content = content.as_ref();
    let content = String::from_utf8_lossy(content);

    let content_to_copy = content.into_owned();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content_to_copy).unwrap();
}
