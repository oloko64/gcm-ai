# GCM-AI

`GCM-AI` is a command-line tool that generates commit messages based on the diff of staged changes using OpenAI. It is written in Rust.

## Installation
To install GCM-AI, you will need to have Rust installed on your system. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install GCM-AI by running the following command:

```bash
cargo install gcm-ai
```

## Usage

To generate a commit message for the staged changes, simply run the following command:

```bash
gcm-ai
```

This will generate a commit message using OpenAI's GPT-3 language model and print it to the console. You can then copy and paste the generated commit message into your git commit.

## Configuration

GCM-AI uses the OpenAI API to generate commit messages. To use the OpenAI API, you will need an API key. You can get an API key by signing up for OpenAI [here](https://beta.openai.com/signup/).

Once you have an API key, you can configure GCM-AI to use it by running the following command:

```bash
gcm-ai --config
```

This will prompt you for your OpenAI API key. Once you enter your API key, it will be saved to a `.toml` configuration file in your home directory.

# Contributing

Contributions are welcome! If you would like to contribute to GCM-AI, please open a pull request.

# License

GCM-AI is licensed under the MIT License. See the [LICENSE](/LICENCE) file for more information.