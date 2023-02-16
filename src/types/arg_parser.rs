use clap::Parser;

/// Generate commit messages using `OpenAI`'s GPT-3 API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Set the initial configuration
    #[clap(long)]
    pub config: bool,
}

impl Args {
    #[must_use]
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
