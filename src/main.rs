use anyhow::Context;
use clap::Parser;
use pdf_summarizer::args::{Args, Commands};
use pdf_summarizer::ollama_api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { model, command } = Args::parse();
    match command {
        Commands::Summarize { path } => ollama_api::summarize(&path, &model)
            .await
            .context("Summarize failed.")?,
        Commands::Name { path } => ollama_api::name(&path, &model)
            .await
            .context("Name failed.")?,
        Commands::Chat => ollama_api::chat().await.context("Chat failed.")?,
    };
    Ok(())
}
