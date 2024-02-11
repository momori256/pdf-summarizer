use clap::Parser;
use pdf_summarizer::{args::Args, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run::run(Args::parse()).await
}
