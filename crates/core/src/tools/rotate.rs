use lopdf::{Document, Object};
use std::io;

/// Rotation angle in degrees (must be 0, 90, 180, or 270).
#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    None = 0,
    Clockwise90 = 90,
    Clockwise180 = 180,
    Clockwise270 = 270,
}

impl Rotation {
    pub fn from_degrees(degrees: i32) -> io::Result<Self> {
        match degrees {
            0 => Ok(Rotation::None),
            90 => Ok(Rotation::Clockwise90),
            180 => Ok(Rotation::Clockwise180),
            270 => Ok(Rotation::Clockwise270),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid rotation angle: {}. Must be 0, 90, 180, or 270", degrees),
            )),
        }
    }
}

/// Rotates specific pages in a PDF document.
///
/// Page numbers are 1-indexed. If page_numbers is empty, rotates all pages.
/// Rotation angles must be 0, 90, 180, or 270 degrees.
pub fn rotate_pages(input_file: &str, page_numbers: &[u32], rotation: Rotation) -> io::Result<Vec<u8>> {
    let mut doc = Document::load(input_file).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to load PDF '{}': {}", input_file, e),
        )
    })?;

    let all_pages: Vec<u32> = doc.get_pages().keys().copied().collect();
    let max_page = *all_pages.iter().max().unwrap_or(&0);

    let pages_to_rotate = if page_numbers.is_empty() {
        all_pages
    } else {
        for &page_num in page_numbers {
            if page_num < 1 || page_num > max_page {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Page {} is out of range (1-{})", page_num, max_page),
                ));
            }
        }
        page_numbers.to_vec()
    };

    for page_num in pages_to_rotate {
        if let Some(&page_id) = doc.get_pages().get(&page_num)
            && let Ok(page_obj) = doc.get_object_mut(page_id)
            && let Object::Dictionary(dict) = page_obj
        {
            let current_rotation = dict.get(b"Rotate").and_then(|obj| obj.as_i64()).unwrap_or(0);

            let new_rotation = (current_rotation + rotation as i64) % 360;
            dict.set("Rotate", Object::Integer(new_rotation));
        }
    }

    let mut output = Vec::new();
    doc.save_to(&mut output)
        .map_err(|e| io::Error::other(format!("Failed to save PDF: {}", e)))?;

    Ok(output)
}
