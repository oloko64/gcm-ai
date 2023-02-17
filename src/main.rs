use once_cell::sync::Lazy;
use owo_colors::{OwoColorize, Stream};
use std::error::Error;
use std::io::Write;
use std::process::exit;
use tokio::process::Command;

mod types;
use types::arg_parser::Args;
use types::completion::CompletionBody;
use types::configs::AppConfig;
use types::git::GitDiff;
use types::openai::OpenAi;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const PROMPT_MAX_LENGTH: usize = 8000;

static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    AppConfig::try_get().unwrap_or_else(|_| {
        eprintln!(
            "{}",
            format!("Configuration file not found, please run `{APP_NAME} --config`")
                .if_supports_color(Stream::Stdout, OwoColorize::bright_red)
        );
        exit(1);
    })
});

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Args = Args::parse_args();
    if args.config {
        print!("Enter your OpenAI API key: ");
        std::io::stdout().flush().unwrap();
        let api_key = rpassword::read_password().unwrap();
        AppConfig::try_set(api_key).expect("Failed to set API key");
    }

    check_config_api_key();

    assert_current_dir_is_git_repo()
        .await
        .expect("Current directory is not a git repository");

    let git_diff = match GitDiff::new().await {
        Ok(git_diff) => git_diff,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let config = CompletionBody::new("text-davinci-003")
        .temperature(0.7)
        .top_p(1.0)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .max_tokens(200)
        .stream(false)
        .n(1);

    println!(
        "\n{}",
        "Generating commit message...".if_supports_color(Stream::Stdout, OwoColorize::bright_blue)
    );

    match generate_commit_message(config, git_diff).await {
        Ok(commit_message) => {
            println!(
                "\n{}",
                commit_message.if_supports_color(Stream::Stdout, OwoColorize::bright_green)
            );
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

fn check_config_api_key() {
    if APP_CONFIG.api_key.is_empty() {
        eprintln!(
            "{}",
            format!("No API key configured, please run `{APP_NAME} --config`")
                .if_supports_color(Stream::Stdout, OwoColorize::bright_red)
        );
        exit(1);
    }
}

async fn generate_commit_message(
    config: CompletionBody,
    git_diff: GitDiff,
) -> Result<String, Box<dyn Error>> {
    let mut prompt_git_diff = format!("Write an insightful but concise Git commit message in a complete sentence in present tense for the following diff without prefacing it with anything: {}", git_diff.get_diff());

    if prompt_git_diff.len() > PROMPT_MAX_LENGTH {
        eprintln!(
            "{}",
            "Diff is too large, the generated message may be not accurate."
                .if_supports_color(Stream::Stdout, OwoColorize::bright_yellow)
        );
        prompt_git_diff = prompt_git_diff[..PROMPT_MAX_LENGTH].to_string();
    }

    let openai = OpenAi::new(&APP_CONFIG.api_key);
    let config = config.prompt(vec![prompt_git_diff]);

    let data = openai.create_completion(config).await?;

    let commit_message = data["choices"][0]["text"]
        .as_str()
        .ok_or("Failed to parse commit message")?
        .trim();

    Ok(String::from(commit_message))
}

async fn assert_current_dir_is_git_repo() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .await?;

    if !output.status.success() {
        eprintln!(
            "{}",
            "Current directory is not a git repository"
                .if_supports_color(Stream::Stdout, OwoColorize::bright_red)
        );
        std::process::exit(1);
    }
    Ok(())
}
