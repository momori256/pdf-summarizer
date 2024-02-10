use chrono::Utc;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationContext, GenerationResponse},
    Ollama,
};
use std::path::Path;
use std::{io::Write, path::PathBuf};
use tokio_stream::StreamExt;

type AppResult = Result<(), Box<dyn std::error::Error>>;

async fn ask<T: std::io::Write>(
    ollama: &Ollama,
    model: &str,
    prompt: &str,
    context: &mut Option<GenerationContext>,
    out: &mut T,
) -> AppResult {
    let mut req = GenerationRequest::new(model.to_string(), prompt.to_string());
    if let Some(context) = context {
        req = req.context(context.clone());
    }

    let res = if cfg!(test) {
        // Use dummy response for test.
        GenerationResponse {
            model: req.model_name,
            created_at: Utc::now().to_string(),
            response: "Dummy Response".to_string(),
            done: true,
            final_data: None,
        }
    } else {
        ollama.generate(req).await?
    };
    out.write_all(res.response.as_bytes())?;
    out.write_all(b"\n")?;
    out.flush()?;
    if let Some(final_data) = res.final_data {
        *context = Some(final_data.context);
    }
    Ok(())
}

async fn ask_stream<T: std::io::Write>(
    ollama: &Ollama,
    model: &str,
    prompt: &str,
    context: &mut Option<GenerationContext>,
    out: &mut T,
) -> AppResult {
    let mut req = GenerationRequest::new(model.to_string(), prompt.to_string());
    if let Some(context) = context {
        req = req.context(context.clone());
    }
    let mut stream = ollama.generate_stream(req).await?;
    while let Some(Ok(responses)) = stream.next().await {
        for res in responses {
            out.write_all(res.response.as_bytes())?;
            out.flush()?;
            if let Some(final_data) = res.final_data {
                *context = Some(final_data.context);
            }
        }
    }
    out.write_all(b"\n")?;
    Ok(())
}

async fn ask_default(model: &str, prompt: &str) -> AppResult {
    let ollama = Ollama::default();
    let mut out = std::io::stdout();
    ask(&ollama.clone(), model, prompt, &mut None, &mut out).await?;
    Ok(())
}

pub async fn summarize(pdf_path: &Path, model: &str) -> AppResult {
    let pdf = pdf_extract::extract_text(pdf_path).unwrap();
    let prompt = format!("Summarize the following text that is from a PDF.\n{pdf}");
    ask_default(model, &prompt).await
}

pub async fn name(pdf_path: &Path, model: &str) -> AppResult {
    let pdf = pdf_extract::extract_text(pdf_path).unwrap();
    let prompt =
        format!("The following text is from a PDF. Give it a suitable and concise title.\n{pdf}");
    ask_default(model, &prompt).await
}

pub async fn chat() -> AppResult {
    chat_internal(std::io::stdin()).await
}

async fn chat_internal<T: std::io::Read>(mut input: T) -> AppResult {
    let ollama = Ollama::default();
    let model = "orca-mini:latest".to_string();
    let mut out = std::io::stdout();
    let mut context = None;
    let mut pdf: Option<String> = None;

    loop {
        out.write_all("> ".as_bytes())?;
        out.flush()?;

        let mut prompt = String::new();
        input.read_to_string(&mut prompt)?;
        let prompt = prompt.trim();

        if prompt == ":exit" {
            break;
        }

        if prompt.starts_with(":use") {
            match parse_use_command(prompt) {
                Ok(content) => {
                    pdf = Some(content);
                    out.write_all("PDF was Successfully loaded. Now {pdf} will be replaced by the content of the PDF.\n".as_bytes())?;
                    out.flush()?;
                }
                Err(e) => {
                    out.write_all(format!("Failed to load PDF: {e:?}\n").as_bytes())?;
                    out.flush()?;
                }
            };
            continue;
        }

        let prompt = match substitue_pdf(prompt, &pdf) {
            Ok(prompt) => prompt,
            Err(e) => {
                out.write_all(e.as_bytes())?;
                out.flush()?;
                continue;
            }
        };
        ask_stream(&ollama, &model, &prompt, &mut context, &mut out).await?;
    }
    Ok(())
}

fn parse_use_command(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (_, path) = prompt.split_once(' ').ok_or(":use <path>.")?;
    let path = normalize_path(Path::new(path))?;
    let content = pdf_extract::extract_text(path)?;
    Ok(content)
}

fn substitue_pdf(prompt: &str, pdf: &Option<String>) -> Result<String, String> {
    const PDF: &str = "{pdf}";
    if !prompt.contains(PDF) {
        return Ok(prompt.to_string());
    }
    match pdf {
        Some(pdf) => Ok(prompt.replace(PDF, pdf)),
        None => Err("Specify a PDF with ':use <path>'\n".to_string()),
    }
}

fn normalize_path(path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(path) = path.strip_prefix("~") {
        let home = std::env::var("HOME")?;
        let home = Path::new(&home);
        return Ok(home.join(path));
    }
    Ok(path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn extract_text_from_dummy_works() {
        let out = pdf_extract::extract_text("dummy.pdf").unwrap();
        assert_eq!("\n\nDummy PDF file", out);
    }

    #[tokio::test]
    async fn model_returns_something_in_response() -> AppResult {
        let ollama = Ollama::default();
        let prompt = "Why is the sky blue?";
        let mut out = std::io::Cursor::new(vec![0; 1024]);
        ask(&ollama, "dummy", prompt, &mut None, &mut out).await?;

        let text = String::from_utf8(out.into_inner())?;
        assert!(!text.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn summarize_and_name_should_success() -> AppResult {
        summarize(Path::new("dummy.pdf"), "dummy").await?;
        name(Path::new("dummy.pdf"), "dummy").await?;
        Ok(())
    }

    #[tokio::test]
    async fn exit_from_chat() -> AppResult {
        let input = Cursor::new(":exit");
        chat_internal(input).await?;
        Ok(())
    }

    #[test]
    fn paser_use_command_can_parse_dummy_pdf() {
        let prompt = ":use dummy.pdf";
        let content = parse_use_command(prompt).unwrap();
        assert_eq!("\n\nDummy PDF file", content);
    }

    #[test]
    fn paser_use_command_fail_without_path() {
        let prompt = ":use";
        let result = parse_use_command(prompt);
        assert!(result.is_err());
    }

    #[test]
    fn substitute_pdf_succeeds_with_pdf() {
        let prompt = "Summarize the following text:\n{pdf}";
        let pdf = Some("Dummy PDF file".to_string());
        let result = substitue_pdf(prompt, &pdf).unwrap();
        assert_eq!("Summarize the following text:\nDummy PDF file", result);
    }

    #[test]
    fn substitute_pdf_fails_with_pdf() {
        let prompt = "Summarize the following text:\n{pdf}";
        let pdf = None;
        let result = substitue_pdf(prompt, &pdf);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_path_expands_tilde() {
        let path = Path::new("~/a/dummy.pdf");
        let path = normalize_path(path);
        let home = std::env::var("HOME").unwrap();
        assert_eq!(Path::new(&format!("{home}/a/dummy.pdf")), path.unwrap());
    }
}
