# pdf-summarizer ![workflow status](https://github.com/momori256/pdf-summarizer/actions/workflows/general.yml/badge.svg) [![codecov](https://codecov.io/gh/momori256/pdf-summarizer/graph/badge.svg?token=SWGHMV2J2Q)](https://codecov.io/gh/momori256/pdf-summarizer)

https://github.com/momori256/pdf-summarizer/assets/90558309/3cee5b58-05f2-4275-947d-9627afb255ef

pdf-summarizer is a PDF summarization CLI app in Rust using [Ollama](https://ollama.com/), a tool similar to Docker for large language models (LLM).
This app is designed to serve as a concise example of how to leverage Ollama's functionalities from Rust.

Article:
[PDF Summarizer with Ollama in 20 Lines of Rust](https://momori-nakano.hashnode.dev/pdf-summarizer-with-ollama-in-20-lines-of-rust)

## Usage

Before running pdf-summarizer, start Ollama server with `ollama serve`.

```sh
ollama serve &
cargo run --bin=pdf-summarizer -- summarize --path sample.pdf
```

Help:

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

`--features=integration_test` should be provided when testing. The flag is used to avoid waiting for a response from LLM for a long time by using a dummy response.

```sh
cargo test --features=integration_test
```

## Binaries

- pdf-summarizer (`src/main.rs`), a main binary
- summarizer (`src/bin/summarizer.rs`), a small example for the article mentioned above
