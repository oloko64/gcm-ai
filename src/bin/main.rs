use openai_rs::configs::completion::CompletionBody;
use openai_rs::OpenAi;

#[tokio::main]
async fn main() {
    let openai = OpenAi::new("sk-1234567890");

    let config = CompletionBody::new()
        .model("davinci")
        .prompt(vec!["This is a test".to_string()]);

    dbg!(config);

    // let response = openai.create_completion(config).await;
}
