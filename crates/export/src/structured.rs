//! The flowing document the writers lay out, as public data: paragraphs (with their heading
//! level and styled runs), tables, images and page breaks in reading order. A host that writes
//! another format (a word processor's own, say) starts from the same structure as Word export.

use crate::layout::{Image, Item, Page, Table, items};

/// One run of a paragraph: text in one style.
#[derive(Clone, Debug, PartialEq)]
pub struct StructuredRun {
    pub text: String,
    /// Font size in points.
    pub size: f64,
    pub bold: bool,
    pub italic: bool,
    /// RGB, 0–1.
    pub color: [f64; 3],
}

/// A piece of the flowing document, in reading order.
#[derive(Clone, Debug, PartialEq)]
pub enum Structured {
    /// A paragraph: `level` 0 is body text, 1 and up a heading (1 the largest), as Word export
    /// styles them.
    Paragraph { level: u8, runs: Vec<StructuredRun> },
    Table(Table),
    Image(Image),
    PageBreak,
}

/// The structure of `pages` (from the engine's export pages), in the order Word export writes it.
pub fn structured(pages: &[Page]) -> Vec<Structured> {
    items(pages)
        .into_iter()
        .map(|it| match it {
            Item::Para(runs, level) => Structured::Paragraph {
                level,
                runs: runs
                    .iter()
                    .map(|r| StructuredRun { text: r.text.clone(), size: r.block.size, bold: r.block.bold, italic: r.block.italic, color: r.block.color })
                    .collect(),
            },
            Item::Table(t) => Structured::Table(t),
            Item::Img(im) => Structured::Image(im.clone()),
            Item::PageBreak => Structured::PageBreak,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Block;

    /// A line at height `y` (PDF user space, y up).
    fn block(text: &str, y: f64, size: f64) -> Block {
        Block { text: text.into(), rect: [72.0, y, 72.0 + 6.0 * text.len() as f64, y + size], size, bold: false, italic: false, color: [0.0; 3], rotation: 0.0, space_around: [false; 2] }
    }

    #[test]
    fn headings_paragraphs_and_page_breaks_come_in_reading_order() {
        let first = Page { width: 612.0, height: 792.0, blocks: vec![block("Title", 700.0, 24.0), block("Body text of the page.", 650.0, 11.0)], ..Page::default() };
        let second = Page { width: 612.0, height: 792.0, blocks: vec![block("More body.", 700.0, 11.0)], ..Page::default() };
        let s = structured(&[first, second]);
        let kinds: Vec<String> = s
            .iter()
            .map(|x| match x {
                Structured::Paragraph { level, runs } => format!("p{level}:{}", runs.iter().map(|r| r.text.as_str()).collect::<String>()),
                Structured::PageBreak => "break".into(),
                Structured::Table(_) => "table".into(),
                Structured::Image(_) => "image".into(),
            })
            .collect();
        assert_eq!(kinds, ["p1:Title", "p0:Body text of the page.", "break", "p0:More body."]);
    }
}
