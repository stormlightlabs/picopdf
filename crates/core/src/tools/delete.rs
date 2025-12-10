use lopdf::Document;
use std::io;

/// Deletes specific pages from a PDF document.
///
/// Page numbers are 1-indexed. Returns a new PDF with the specified pages removed.
pub fn delete_pages(input_file: &str, page_numbers: &[u32]) -> io::Result<Vec<u8>> {
    if page_numbers.is_empty() {
        return std::fs::read(input_file);
    }

    let doc = Document::load(input_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to load PDF '{}': {}", input_file, e),
        )
    })?;

    let all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    let max_page = *all_pages.iter().max().unwrap_or(&0);

    for &page_num in page_numbers {
        if page_num < 1 || page_num > max_page {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Page {} is out of range (1-{})", page_num, max_page),
            ));
        }
    }

    let mut output_doc = doc;
    output_doc.delete_pages(page_numbers);

    let mut output = Vec::new();
    output_doc
        .save_to(&mut output)
        .map_err(|e| io::Error::other(format!("Failed to save PDF: {}", e)))?;

    Ok(output)
}
