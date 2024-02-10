use pdf_summarizer::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).expect("'path' must be provided.");
    summarize(std::path::Path::new(&path)).await?;
    Ok(())
}
