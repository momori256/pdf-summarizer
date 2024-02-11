use anyhow::Context;

use crate::{
    args::{Args, Commands},
    ollama_api,
};

pub async fn run(args: Args) -> anyhow::Result<()> {
    let Args { model, command } = args;
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
