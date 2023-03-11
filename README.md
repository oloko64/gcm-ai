# GCM-AI

[![Rust](https://github.com/OLoKo64/gcm-ai/actions/workflows/rust.yml/badge.svg)](https://github.com/OLoKo64/gcm-ai/actions/workflows/rust.yml)

![gcm-ai](https://user-images.githubusercontent.com/49915167/219650248-0e7823d6-d2d9-44df-9645-4f4875b47db3.png)

`GCM-AI` is a command-line tool that generates commit messages based on the diff of staged changes using OpenAI.

This app inspired by [aicommits](https://github.com/Nutlope/aicommits) with some tweaks and written in Rust.

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

Please note that OpenAI is currently in beta, and the API is not free. But for now when you sign up, you will get $18 of free credits to use on the API, which should be enough to generate enough commit messages  as the generated commit messages are very cheap.

Once you have an API key, you can configure GCM-AI to use it by running the following command:

```bash
gcm-ai --config
```

This will prompt you for your OpenAI API key. Once you enter your API key, it will be saved to a `.toml` configuration file in your home directory.

# Contributing

Contributions are welcome! If you would like to contribute to GCM-AI, please open a pull request.

# License

GCM-AI is licensed under the MIT License. See the [LICENSE](/LICENCE) file for more information.
