use docopt;

use super::config;
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
                            supported formats: html, markdown
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

    let cfg = config::Config {
        token: args.flag_token.unwrap(), // TODO
        format: args.flag_format,
        user: args.flag_user,
        repo: args.flag_repo,
        base_branch: args.arg_base,
        head_branch: args.arg_head,
    };

    // println!("{:?}", cfg);

    // TODO
    let src = source::GitHubSource::new(&cfg).unwrap();
    println!("{:?}", src.get_prs());
}
