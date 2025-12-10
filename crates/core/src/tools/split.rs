use lopdf::Document;
use std::io;

/// Splits a PDF into separate single-page PDF files.
///
/// Returns a vector of PDF bytes, one for each page in the input document.
/// Each output PDF contains exactly one page from the original.
pub fn split_pdf(input_file: &str) -> io::Result<Vec<Vec<u8>>> {
    let doc = Document::load(input_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to load PDF '{}': {}", input_file, e),
        )
    })?;

    let page_count = doc.get_pages().len();
    let mut split_pdfs = Vec::new();

    for page_num in 1..=page_count as u32 {
        let mut page_doc = doc.clone();

        let all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
        let pages_to_delete: Vec<u32> = all_pages.iter().filter(|&&p| p != page_num).copied().collect();

        page_doc.delete_pages(&pages_to_delete);

        let mut output = Vec::new();
        page_doc
            .save_to(&mut output)
            .map_err(|e| io::Error::other(format!("Failed to save page {}: {}", page_num, e)))?;

        split_pdfs.push(output);
    }

    Ok(split_pdfs)
}

/// Splits a PDF into chunks of specified page ranges.
///
/// Each range is specified as a tuple (start_page, end_page) inclusive.
/// Page numbers are 1-indexed.
pub fn split_pdf_by_ranges(input_file: &str, ranges: &[(u32, u32)]) -> io::Result<Vec<Vec<u8>>> {
    let doc = Document::load(input_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to load PDF '{}': {}", input_file, e),
        )
    })?;

    let all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    let max_page = *all_pages.iter().max().unwrap_or(&0);

    let mut split_pdfs = Vec::new();

    for (start, end) in ranges {
        if *start < 1 || *end > max_page || start > end {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid page range: {}-{}", start, end),
            ));
        }

        let mut range_doc = doc.clone();

        let pages_to_delete: Vec<u32> = all_pages.iter().filter(|&&p| p < *start || p > *end).copied().collect();

        range_doc.delete_pages(&pages_to_delete);

        let mut output = Vec::new();
        range_doc
            .save_to(&mut output)
            .map_err(|e| io::Error::other(format!("Failed to save range {}-{}: {}", start, end, e)))?;

        split_pdfs.push(output);
    }

    Ok(split_pdfs)
}
