error_chain! {
    errors {
        UnexpectedInput {
        }
    }

    foreign_links {
        GitHubError(::github_rs::errors::Error);
    }
}
