use owo_colors::OwoColorize;
use std::error::Error;
use std::io::Write;
use std::process::{exit, Command};

mod types;
use types::arg_parser::Args;
use types::completion::CompletionBody;
use types::configs::AppConfig;
use types::git::GitDiff;
use types::openai::OpenAi;

/// Generate commit messages using `OpenAI`'s GPT-3 API

#[tokio::main]
async fn main() {
    let args: Args = Args::parse_args();
    if args.config {
        print!("Enter your OpenAI API key: ");
        std::io::stdout().flush().unwrap();
        let api_key = rpassword::read_password().unwrap();
        AppConfig::try_set(api_key).expect("Failed to set API key");
    }

    assert_current_dir_is_git_repo().expect("Current directory is not a git repository");

    let git_diff = match GitDiff::new() {
        Ok(git_diff) => git_diff,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };

    let config = CompletionBody::new()
        .model("text-davinci-003")
        .temperature(0.7)
        .top_p(1.0)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .max_tokens(200)
        .stream(false)
        .n(1);

    match generate_commit_message(config, git_diff).await {
        Ok(commit_message) => {
            println!("\n{}", commit_message.bright_green());
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

async fn generate_commit_message(
    config: CompletionBody,
    git_diff: GitDiff,
) -> Result<String, Box<dyn Error>> {
    if git_diff.get_diff().len() > 8000 {
        return Err("Diff is too large".into());
    }

    let api_key = match AppConfig::try_get() {
        Ok(config) => {
            if config.api_key.is_empty() {
                println!("{}", "No API key configured, please run `openai-rs --config`".bright_red());
                exit(1);
            }
            config.api_key
        }
        Err(_) => {
            println!("{}", "No API key configured, please run `openai-rs --config`".bright_red());
            exit(1);
        }
    };
    let openai = OpenAi::new(api_key);
    let config = config
        .prompt(vec![format!("Write an insightful but concise Git commit message in a complete sentence in present tense for the following diff without prefacing it with anything: {}", git_diff.get_diff())]);

    let data = openai.create_completion(config).await?;

    let commit_message = data["choices"][0]["text"]
        .as_str()
        .ok_or("Failed to parse commit message")?
        .trim();

    Ok(String::from(commit_message))
}

fn assert_current_dir_is_git_repo() -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()?;

    if !output.status.success() {
        return Err("Current directory is not a git repository".into());
    }
    Ok(())
}
