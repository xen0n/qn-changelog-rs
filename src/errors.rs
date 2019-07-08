use failure::Error;

pub(crate) type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub(crate) enum QnChangelogError {
    #[fail(display = "unexpected input")]
    UnexpectedInput,
}
