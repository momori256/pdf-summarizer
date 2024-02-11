use anyhow::Result;
use clap::Parser;
use pdf_summarizer::{args::Args, run::run};

#[tokio::test]
async fn summarize_works() -> Result<()> {
    let args = Args::parse_from(vec!["pdf_summarizer", "summarize", "--path", "dummy.pdf"]);
    run(args).await
}

#[tokio::test]
async fn name_works() -> Result<()> {
    let args = Args::parse_from(vec![
        "pdf_summarizer",
        "--model",
        "tinyllama:latest",
        "name",
        "--path",
        "dummy.pdf",
    ]);
    run(args).await
}
