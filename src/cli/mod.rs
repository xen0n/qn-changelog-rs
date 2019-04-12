use docopt;

use super::config;
use super::filter;
use super::fmt;
use super::source;


const USAGE: &'static str = "
Qiniu changelog generator (Rust port).

Usage:
  qn-changelog [options] <base> <head>
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
  -h, --help                Show help
";


#[derive(Debug, Deserialize)]
struct Args {
    arg_base: String,
    arg_head: String,

    flag_user: String,
    flag_repo: String,
    flag_token: Option<String>,
    flag_all: bool,
    flag_before: Option<String>,
    flag_after: Option<String>,
    flag_format: config::OutputFormat,
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

    let cfg = config::Config {
        token: token.to_string(),
        format: args.flag_format,
        user: args.flag_user,
        repo: args.flag_repo,
        base_branch: args.arg_base,
        head_branch: args.arg_head,
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

    let stdout = ::std::io::stdout();

    // ChangelogFormatter is not object-safe, so we can't do `Box::new(sink)`
    // and reuse code
    use fmt::ChangelogFormatter;
    match cfg.format {
        config::OutputFormat::Html => {
            let mut sink = fmt::HtmlFormatter::with_writer(stdout);
            sink.format(&prefs, &entries).unwrap();
        }
        config::OutputFormat::Jira => {
            let mut sink = fmt::JiraFormatter::with_writer(stdout);
            sink.format(&prefs, &entries).unwrap();
        }
        config::OutputFormat::Markdown => {
            let mut sink = fmt::MarkdownFormatter::with_writer(stdout);
            sink.format(&prefs, &entries).unwrap();
        }
    }
}
