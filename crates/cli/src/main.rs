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

        /// Custom regular font file (TTF/OTF)
        #[arg(long)]
        font_regular: Option<PathBuf>,

        /// Custom bold font file (TTF/OTF)
        #[arg(long)]
        font_bold: Option<PathBuf>,

        /// Custom monospace font file (TTF/OTF)
        #[arg(long)]
        font_mono: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Write { input, output, font_regular, font_bold, font_mono } => {
            if let Err(e) = write_command(&input, &output, &font_regular, &font_bold, &font_mono) {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}

/// Executes the write command to convert markdown to PDF.
///
/// Reads the input markdown file, processes it through the four-stage pipeline, and writes the resulting PDF to the output file.
fn write_command(
    input: &PathBuf, output: &PathBuf, font_regular: &Option<PathBuf>, font_bold: &Option<PathBuf>,
    font_mono: &Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "Reading".cyan().bold(), input.display());

    let markdown =
        fs::read_to_string(input).map_err(|e| format!("Failed to read input file '{}': {}", input.display(), e))?;

    let fonts = if font_regular.is_some() || font_bold.is_some() || font_mono.is_some() {
        println!("{} custom fonts...", "Loading".cyan().bold());
        match picopdf_core::md::FontConfig::from_files(
            font_regular.as_deref(),
            font_bold.as_deref(),
            font_mono.as_deref(),
        ) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{} {}", "Warning:".yellow().bold(), e);
                eprintln!("{} Using default fonts instead", "         ".yellow());
                picopdf_core::md::FontConfig::default()
            }
        }
    } else {
        picopdf_core::md::FontConfig::default()
    };

    println!("{} markdown to PDF...", "Converting".cyan().bold());

    let pdf_bytes = picopdf_core::md::markdown_to_pdf_with_fonts(&markdown, &fonts)?;

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
