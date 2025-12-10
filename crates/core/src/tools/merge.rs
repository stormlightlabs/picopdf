use lopdf::Document;
use std::io;

/// Merges multiple PDF files into a single PDF document.
///
/// Takes a list of PDF file paths and combines all pages from each document in the order provided. Uses a simple concatenation approach.
pub fn merge_pdfs(input_files: &[&str]) -> io::Result<Vec<u8>> {
    if input_files.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No input files provided"));
    }

    if input_files.len() == 1 {
        return std::fs::read(input_files[0]);
    }

    let first_doc = Document::load(input_files[0]).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to load PDF '{}': {}", input_files[0], e),
        )
    })?;

    let mut output_doc = first_doc;

    for file_path in &input_files[1..] {
        let doc = Document::load(file_path).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to load PDF '{}': {}", file_path, e),
            )
        })?;

        let pages: Vec<u32> = doc.get_pages().keys().copied().collect();

        for page_num in pages {
            if let Some(&page_id) = doc.get_pages().get(&page_num)
                && let Ok(page_obj) = doc.get_object(page_id)
            {
                output_doc.add_object(page_obj.clone());
            }
        }
    }

    let mut output = Vec::new();
    output_doc
        .save_to(&mut output)
        .map_err(|e| io::Error::other(format!("Failed to save PDF: {}", e)))?;

    Ok(output)
}
