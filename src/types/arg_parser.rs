use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The arguments to pass to the commit command
    #[clap(short, long)]
    commit_args: Option<Vec<String>>,
}

impl Args {
    #[must_use]
    pub fn parse_args() -> Self {
        Self::parse()
    }

    #[must_use]
    pub fn get_commit_args(&self) -> &Option<Vec<String>> {
        &self.commit_args
    }
}
