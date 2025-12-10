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

    /// PDF manipulation tools
    #[command(subcommand)]
    Tool(ToolCommands),
}

#[derive(Subcommand)]
enum ToolCommands {
    /// Merge multiple PDF files into one
    Merge {
        /// Input PDF files to merge
        #[arg(required = true)]
        inputs: Vec<PathBuf>,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Split PDF into separate files
    Split {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for split PDFs
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Optional page ranges (e.g., "1-3,5,7-9")
        #[arg(short, long)]
        ranges: Option<String>,
    },

    /// Extract specific pages
    Extract {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,

        /// Pages to extract (e.g., "1,3,5-7")
        #[arg(short, long)]
        pages: String,
    },

    /// Delete specific pages
    Delete {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,

        /// Pages to delete (e.g., "2,4,6-8")
        #[arg(short, long)]
        pages: String,
    },

    /// Rotate pages
    Rotate {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,

        /// Rotation angle (90, 180, or 270 degrees)
        #[arg(short = 'a', long, default_value = "90")]
        angle: i32,

        /// Pages to rotate (empty = all pages, e.g., "1,3,5-7")
        #[arg(short, long)]
        pages: Option<String>,
    },

    /// Reorganize pages in specified order
    Organize {
        /// Input PDF file
        #[arg(short, long)]
        input: PathBuf,

        /// Output PDF file
        #[arg(short, long)]
        output: PathBuf,

        /// New page order (e.g., "3,1,2,4-6")
        #[arg(short = 'n', long)]
        new_order: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Write { input, output, font_regular, font_bold, font_mono } => {
            write_command(&input, &output, &font_regular, &font_bold, &font_mono)
        }
        Commands::Tool(tool_cmd) => handle_tool_command(tool_cmd),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
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

fn handle_tool_command(cmd: ToolCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ToolCommands::Merge { inputs, output } => {
            println!("{} {} PDF files...", "Merging".cyan().bold(), inputs.len());
            let input_paths: Vec<&str> = inputs.iter().map(|p| p.to_str().unwrap()).collect();
            let pdf_bytes = picopdf_core::tools::merge_pdfs(&input_paths)?;
            fs::write(&output, pdf_bytes)?;
            println!("{} Merged to {}", "✓".green().bold(), output.display());
        }

        ToolCommands::Split { input, output_dir, ranges } => {
            fs::create_dir_all(&output_dir)?;

            match ranges {
                Some(ranges_str) => {
                    println!("{} PDF by ranges...", "Splitting".cyan().bold());
                    let ranges = parse_page_ranges(&ranges_str)?;
                    let pdfs = picopdf_core::tools::split_pdf_by_ranges(input.to_str().unwrap(), &ranges)?;

                    for (idx, pdf_bytes) in pdfs.iter().enumerate() {
                        let output_file = output_dir.join(format!("split_{}.pdf", idx + 1));
                        fs::write(&output_file, pdf_bytes)?;
                        println!("{} Created {}", "✓".green().bold(), output_file.display());
                    }
                }
                None => {
                    println!("{} PDF into individual pages...", "Splitting".cyan().bold());
                    let pdfs = picopdf_core::tools::split_pdf(input.to_str().unwrap())?;

                    for (idx, pdf_bytes) in pdfs.iter().enumerate() {
                        let output_file = output_dir.join(format!("page_{}.pdf", idx + 1));
                        fs::write(&output_file, pdf_bytes)?;
                    }
                    println!(
                        "{} Split into {} pages in {}",
                        "✓".green().bold(),
                        pdfs.len(),
                        output_dir.display()
                    );
                }
            }
        }

        ToolCommands::Extract { input, output, pages } => {
            println!("{} pages {}...", "Extracting".cyan().bold(), pages);
            let page_numbers = parse_page_list(&pages)?;
            let pdf_bytes = picopdf_core::tools::extract_pages(input.to_str().unwrap(), &page_numbers)?;
            fs::write(&output, pdf_bytes)?;
            println!("{} Extracted to {}", "✓".green().bold(), output.display());
        }

        ToolCommands::Delete { input, output, pages } => {
            println!("{} pages {}...", "Deleting".cyan().bold(), pages);
            let page_numbers = parse_page_list(&pages)?;
            let pdf_bytes = picopdf_core::tools::delete_pages(input.to_str().unwrap(), &page_numbers)?;
            fs::write(&output, pdf_bytes)?;
            println!("{} Deleted pages, saved to {}", "✓".green().bold(), output.display());
        }

        ToolCommands::Rotate { input, output, angle, pages } => {
            let rotation = picopdf_core::tools::Rotation::from_degrees(angle)?;
            let page_numbers = if let Some(pages_str) = pages { parse_page_list(&pages_str)? } else { vec![] };

            println!("{} pages by {} degrees...", "Rotating".cyan().bold(), angle);
            let pdf_bytes = picopdf_core::tools::rotate_pages(input.to_str().unwrap(), &page_numbers, rotation)?;
            fs::write(&output, pdf_bytes)?;
            println!("{} Rotated, saved to {}", "✓".green().bold(), output.display());
        }

        ToolCommands::Organize { input, output, new_order } => {
            println!("{} pages...", "Reorganizing".cyan().bold());
            let order = parse_page_list(&new_order)?;
            let pdf_bytes = picopdf_core::tools::organize_pages(input.to_str().unwrap(), &order)?;
            fs::write(&output, pdf_bytes)?;
            println!("{} Reorganized, saved to {}", "✓".green().bold(), output.display());
        }
    }

    Ok(())
}

/// Parses a page list string like "1,3,5-7" into a vector of page numbers.
fn parse_page_list(pages_str: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut pages = Vec::new();

    for part in pages_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(format!("Invalid page range: {}", part).into());
            }
            let start: u32 = range_parts[0].parse()?;
            let end: u32 = range_parts[1].parse()?;
            for page in start..=end {
                pages.push(page);
            }
        } else {
            pages.push(part.parse()?);
        }
    }

    Ok(pages)
}

/// Parses page ranges like "1-3,5,7-9" into tuples of (start, end).
fn parse_page_ranges(ranges_str: &str) -> Result<Vec<(u32, u32)>, Box<dyn std::error::Error>> {
    let mut ranges = Vec::new();

    for part in ranges_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(format!("Invalid page range: {}", part).into());
            }
            let start: u32 = range_parts[0].parse()?;
            let end: u32 = range_parts[1].parse()?;
            ranges.push((start, end));
        } else {
            let page: u32 = part.parse()?;
            ranges.push((page, page));
        }
    }

    Ok(ranges)
}
