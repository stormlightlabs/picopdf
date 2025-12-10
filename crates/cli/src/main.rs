use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "picopdf")]
#[command(about = "A minimalist markdown to PDF converter", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert markdown file to PDF
    Write {
        /// Input markdown file
        #[arg(short, long)]
        input: PathBuf,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Write { input, output } => {
            if let Err(e) = write_command(&input, &output) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}

/// Executes the write command to convert markdown to PDF.
///
/// Reads the input markdown file, processes it through the four-stage pipeline, and writes the resulting PDF to the output file.
fn write_command(input: &PathBuf, output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "Reading".cyan().bold(), input.display());

    let markdown =
        fs::read_to_string(input).map_err(|e| format!("Failed to read input file '{}': {}", input.display(), e))?;

    println!("{} markdown to PDF...", "Converting".cyan().bold());

    let pdf_bytes = picopdf_core::md::markdown_to_pdf(&markdown)?;

    println!("{} PDF to {}", "Writing".cyan().bold(), output.display());

    fs::write(output, pdf_bytes).map_err(|e| format!("Failed to write output file '{}': {}", output.display(), e))?;

    println!(
        "{} Successfully converted {} to {}",
        "✓".green().bold(),
        input.display(),
        output.display()
    );

    Ok(())
}
