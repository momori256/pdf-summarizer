use chrono::Utc;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationContext, GenerationResponse},
    Ollama,
};
use std::io::Write;
use std::path::Path;
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

    loop {
        out.write_all("> ".as_bytes())?;
        out.flush()?;

        let mut prompt = String::new();
        input.read_to_string(&mut prompt)?;
        let prompt = prompt.trim();
        if prompt == ":exit" {
            break;
        }

        ask_stream(&ollama, &model, prompt, &mut context, &mut out).await?;
    }
    Ok(())
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
}
