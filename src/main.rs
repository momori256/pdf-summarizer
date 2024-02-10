use clap::Parser;
use pdf_summarizer::args::{Args, Commands};
use pdf_summarizer::ollama_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Args { model, command } = Args::parse();
    match command {
        Commands::Summarize { path } => ollama_api::summarize(&path, &model).await?,
        Commands::Name { path } => ollama_api::name(&path, &model).await?,
        Commands::Chat => ollama_api::chat().await?,
    }
    Ok(())
}
