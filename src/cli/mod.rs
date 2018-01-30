use docopt;


const USAGE: &'static str = "
Qiniu changelog generator (Rust port).

Usage:
  qn-changelog [options] <base> <head>
  qn-changelog (-h | --help)

<base> and <head> can be branch name or tag or commit hash

Options:
  -u USER, --user USER      GitHub user
  -r REPO, --repo REPO      GitHub repo name
  -t TOKEN, --token TOKEN   GitHub access token
  -a, --all                 show all pull-request, not filter deploy pr
  --before TIME             filter changelog before time
  --after TIME              filter changelog after time
  -f FMT, --format FMT      result format
  -h, --help                Show help
";


#[derive(Debug, Deserialize)]
struct Args {
    arg_base: String,
    arg_head: String,

    flag_user: Option<String>,
    flag_repo: Option<String>,
    flag_token: Option<String>,
    flag_all: bool,
    flag_before: Option<String>,
    flag_after: Option<String>,
    flag_format: Option<String>, // TODO: enum
}


pub(crate) fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
