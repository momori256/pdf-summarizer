# pdf-summarizer ![workflow status](https://github.com/momori256/pdf-summarizer/actions/workflows/general.yml/badge.svg)

pdf-summarizer is a PDF summarization CLI app in Rust using [Ollama](https://ollama.com/), a tool similar to Docker for large language models (LLM).
This app is designed to serve as a concise example to illustrate the way of leveraging Ollama's functionalities from Rust.

Article:
[PDF Summarizer with Ollama in 20 Lines of Rust](https://momori-nakano.hashnode.dev/pdf-summarizer-with-ollama-in-20-lines-of-rust)

## Usage

```
> cargo run --bin=pdf-summarizer

Usage: pdf-summarizer [OPTIONS] <COMMAND>

Commands:
  summarize
  name       Give a name to the PDF file
  chat
  help       Print this message or the help of the given subcommand(s)

Options:
  -m, --model <MODEL>  [default: orca-mini:latest]
  -h, --help           Print help
  -V, --version        Print version
```

## Test

`--features=integration_test` should be provided when testing. The flag is used to avoid waiting response from LLM for a long time.

```sh
cargo test --features=integration_test
```

## bin

- pdf-summarizer (`src/main.rs`), a main binary
- summarizer (`src/bin/summarizer.rs`), a small example for the article mentioned above
