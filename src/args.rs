use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(short, long)]
    #[clap(default_value = "orca-mini:latest")]
    pub model: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Summarize {
        #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
        path: PathBuf,
    },
    /// Give a name to the PDF file
    Name {
        #[arg(short, long, value_hint = clap::ValueHint::FilePath)]
        path: PathBuf,
    },
    Chat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_without_command_should_fail() {
        assert!(Args::try_parse_from(vec![""]).is_err());
    }

    #[test]
    fn default_model_is_orca_mini() -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::try_parse_from(vec![
            "pdf-summarizer",
            "summarize",
            "--path",
            "./dummay.pdf",
        ])?;
        assert_eq!("orca-mini:latest", args.model);
        Ok(())
    }
}
