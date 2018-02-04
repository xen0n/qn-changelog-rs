error_chain! {
    errors {
        UnexpectedInput {
        }
    }

    foreign_links {
        IoError(::std::io::Error);
        JsonError(::serde_json::Error);
        GitHubError(::github_rs::errors::Error);
    }
}
