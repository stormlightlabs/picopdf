use lopdf::Document;
use std::io;

/// Reorganizes pages in a PDF according to the specified order.
///
/// The new_order vector specifies the page numbers (1-indexed) in the desired order.
/// For example, [3, 1, 2] would reorder the pages so page 3 becomes first, followed by page 1, then page 2.
pub fn organize_pages(input_file: &str, new_order: &[u32]) -> io::Result<Vec<u8>> {
    let doc = Document::load(input_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to load PDF '{}': {}", input_file, e),
        )
    })?;

    let all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    let max_page = *all_pages.iter().max().unwrap_or(&0);

    if new_order.is_empty() {
        return std::fs::read(input_file);
    }

    for &page_num in new_order {
        if page_num < 1 || page_num > max_page {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Page {} is out of range (1-{})", page_num, max_page),
            ));
        }
    }

    let first_page_num = new_order[0];
    let mut output_doc = doc.clone();
    let pages_to_delete: Vec<u32> = all_pages.iter().filter(|&&p| p != first_page_num).copied().collect();
    output_doc.delete_pages(&pages_to_delete);

    for &page_num in &new_order[1..] {
        let mut page_doc = doc.clone();
        let pages_to_delete: Vec<u32> = all_pages.iter().filter(|&&p| p != page_num).copied().collect();
        page_doc.delete_pages(&pages_to_delete);

        if let Some(&page_id) = page_doc.get_pages().get(&page_num)
            && let Ok(page_obj) = page_doc.get_object(page_id)
        {
            output_doc.add_object(page_obj.clone());
        }
    }

    let mut merged = output_doc;

    let mut output = Vec::new();
    merged
        .save_to(&mut output)
        .map_err(|e| io::Error::other(format!("Failed to save PDF: {}", e)))?;

    Ok(output)
}
