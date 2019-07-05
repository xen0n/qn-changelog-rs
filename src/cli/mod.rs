use clap;
use clap::value_t;

use super::config;
use super::filter;
use super::fmt;
use super::source;


pub(crate) fn main() {
    let args = clap::App::new("qn-changelog-rs")
        .about("Qiniu changelog generator (Rust port)")
        .arg(
            clap::Arg::with_name("user")
                .short("u")
                .long("user")
                .takes_value(true)
                .value_name("USER")
                .default_value("qbox")
                .required(false)
                .help("GitHub user"),
        )
        .arg(
            clap::Arg::with_name("repo")
                .short("r")
                .long("repo")
                .takes_value(true)
                .value_name("REPO")
                .default_value("portal-v4")
                .required(false)
                .help("GitHub repo name"),
        )
        .arg(
            clap::Arg::with_name("token")
                .short("t")
                .long("token")
                .takes_value(true)
                .value_name("TOKEN")
                .required(false)
                .help("GitHub access token"),
        )
        .arg(
            clap::Arg::with_name("all")
                .short("a")
                .long("all")
                .required(false)
                .help("show all pull-request, not filter deploy pr"),
        )
        .arg(
            clap::Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .value_name("FMT")
                .possible_values(&["html", "jira", "markdown"])
                .default_value("markdown")
                .help("result format"),
        )
        .arg(
            clap::Arg::with_name("copy")
                .short("c")
                .long("copy")
                .required(false)
                .help("also copy results to system clipboard\n(feature=clipboard builds only)"),
        )
        .arg(
            clap::Arg::with_name("base")
                .value_name("base")
                .index(1)
                .required(false)
                .default_value("master"),
        )
        .arg(
            clap::Arg::with_name("head")
                .value_name("head")
                .index(2)
                .required(false)
                .default_value("develop"),
        )
        .get_matches();

    // println!("{:?}", args);

    let prefs = config::preference::UserPreference::load().unwrap();
    let (token, should_update_token) = match (prefs.token(), args.value_of("token")) {
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

    let (base, head) = {
        let base = args.value_of("base").unwrap();
        let head = args.value_of("head").unwrap();
        (base.to_string(), head.to_string())
    };

    let cfg = config::Config {
        token: token.to_string(),
        format: value_t!(args, "format", config::OutputFormat).unwrap(),
        user: args.value_of("user").unwrap().to_string(),
        repo: args.value_of("repo").unwrap().to_string(),
        base_branch: base,
        head_branch: head,
        dont_filter: args.is_present("all"),
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
    if args.is_present("copy") {
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
