use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const USAGE: &str = "Usage: ./summarizer <pdf_path> <model>";

    let mut args = std::env::args().skip(1);
    let pdf_path = args.next().expect(USAGE);
    let model = args.next().expect(USAGE);

    let ollama = Ollama::default();
    let pdf = pdf_extract::extract_text(pdf_path)?;
    let prompt = format!("Summarize the following text from a PDF file:\n{pdf}");

    let request = GenerationRequest::new(model, prompt);
    let mut stream = ollama.generate_stream(request).await?;
    while let Some(Ok(responses)) = stream.next().await {
        for res in responses {
            print!("{}", res.response);
        }
    }
    println!();

    Ok(())
}
