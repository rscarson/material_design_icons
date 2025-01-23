//! Font parsing and glyph lookup
//!
//! This module contains the [`Font`] struct, which is a wrapper around the [`allsorts`] crate.
//!
//! It also provides a few other font-introspection utilities, such as [`FontMapper`] and [`Glyph`].
//!
use allsorts::{
    binary::read::{ReadBinary, ReadScope},
    bitmap::{BitDepth, Bitmap, BitmapGlyph, EncapsulatedFormat},
    font::MatchingPresentation,
    font_data::{DynamicFontTableProvider, FontData},
    tables::cmap::CmapSubtable,
};
use std::borrow::Cow;

/// Public re-export of the `allsorts` crate
pub use allsorts;

/// A loaded font
pub struct Font<'a>(allsorts::Font<DynamicFontTableProvider<'a>>);
impl<'a> Font<'a> {
    /// Load a font from a byte slice
    pub fn new(font_data: &'a [u8]) -> Result<Self, FontError> {
        let scope = ReadScope::new(font_data);
        let font = scope.read::<FontData<'_>>()?;
        let provider = font.table_provider(0)?;
        let font = allsorts::Font::new(provider)?;

        Ok(Self(font))
    }

    /// Load the Google Material Design Icons font in the "Outlined" style
    pub fn new_outlined() -> Result<Self, FontError> {
        Self::new(crate::outlined::ICON_FONT)
    }

    /// Load the Google Material Design Icons font in the "Rounded" style
    pub fn new_rounded() -> Result<Self, FontError> {
        Self::new(crate::rounded::ICON_FONT)
    }

    /// Load the Google Material Design Icons font in the "Sharp" style
    pub fn new_sharp() -> Result<Self, FontError> {
        Self::new(crate::sharp::ICON_FONT)
    }

    /// Return the raw font data
    pub fn font_data(&self) -> &allsorts::Font<DynamicFontTableProvider<'a>> {
        &self.0
    }

    /// Return the raw font data as mutable
    pub fn font_data_mut(&mut self) -> &mut allsorts::Font<DynamicFontTableProvider<'a>> {
        &mut self.0
    }

    /// Lookup a named glyph by it's index
    pub fn glyph_name(&self, id: u16) -> Option<Cow<'a, str>> {
        let name = self.font_data().glyph_names(&[id]).pop()?;
        Some(name)
    }

    /// Lookup a glyph ID by it's character code
    pub fn index_of(&mut self, codepoint: u32) -> Option<u16> {
        let char = std::char::from_u32(codepoint)?;
        let (id, _) =
            self.font_data_mut()
                .lookup_glyph_index(char, MatchingPresentation::NotRequired, None);
        Some(id)
    }

    /// Lookup a bitmap for a glyph by it's ID
    pub fn bitmap_for(&mut self, id: u16) -> Result<Option<BitmapGlyph>, FontError> {
        let bitmap = self
            .font_data_mut()
            .lookup_glyph_image(id, 0, BitDepth::ThirtyTwo)?;
        Ok(bitmap)
    }
}

/// A structure designed to map out the contents of a font
pub struct FontMapper<'a> {
    font: &'a Font<'a>,
    cmap: CmapSubtable<'a>,
}
impl<'a> FontMapper<'a> {
    pub fn new(font: &'a Font<'a>) -> Result<Self, FontError> {
        let cmap_data = font.font_data().cmap_subtable_data();
        let cmap = CmapSubtable::read(&mut ReadScope::new(cmap_data).ctxt())?;
        Ok(Self { font, cmap })
    }

    /// Return all glyphs in the font
    pub fn all_chars(&self) -> Result<Vec<Glyph>, FontError> {
        let chars = self.cmap.mappings()?;
        let mut glyphs = Vec::with_capacity(chars.len());
        for (_, codepoint) in chars {
            let Some(glyph) = self.find_glyph(codepoint)? else {
                continue;
            };

            glyphs.push(glyph);
        }
        Ok(glyphs)
    }

    /// Return a [`Glyph`] by character code
    pub fn find_glyph(&self, codepoint: u32) -> Result<Option<Glyph>, FontError> {
        let id = self.cmap.map_glyph(codepoint)?;
        let Some(id) = id else {
            return Ok(None);
        };

        let Some(name) = self.font.glyph_name(id) else {
            return Ok(None);
        };

        Ok(Some(Glyph {
            id,
            codepoint,
            name,
        }))
    }
}

/// Describes a single named glyph in a font  
/// Contains the glyph's ID, character code, and name
#[derive(Debug, Clone)]
pub struct Glyph<'a> {
    pub id: u16,
    pub codepoint: u32,
    pub name: Cow<'a, str>,
}
impl Glyph<'_> {
    pub fn char(&self) -> Option<char> {
        std::char::from_u32(self.codepoint)
    }
}

pub trait BitmapExt {
    /// Attempt to export the bitmap as a tuple of a file extension and raw image data  
    /// Returns None if the image format is not supported
    fn export(self) -> Option<(&'static str, Box<[u8]>)>;
}
impl BitmapExt for BitmapGlyph {
    fn export(self) -> Option<(&'static str, Box<[u8]>)> {
        match self.bitmap {
            Bitmap::Encapsulated(data) => match data.format {
                EncapsulatedFormat::Jpeg => Some(("jpg", data.data)),
                EncapsulatedFormat::Png => Some(("png", data.data)),
                EncapsulatedFormat::Tiff => Some(("tiff", data.data)),
                EncapsulatedFormat::Svg => Some(("svg", data.data)),
                _ => None,
            },

            Bitmap::Embedded(data) => Some(("bmp", data.data)),
        }
    }
}

/// Error type for font operations
#[derive(Debug)]
pub enum FontError {
    ParseError(allsorts::error::ParseError),
    ReadWriteError(allsorts::error::ReadWriteError),
    Io(std::io::Error),
}
impl From<allsorts::error::ParseError> for FontError {
    fn from(err: allsorts::error::ParseError) -> Self {
        FontError::ParseError(err)
    }
}
impl From<allsorts::error::ReadWriteError> for FontError {
    fn from(err: allsorts::error::ReadWriteError) -> Self {
        FontError::ReadWriteError(err)
    }
}
impl From<std::io::Error> for FontError {
    fn from(err: std::io::Error) -> Self {
        FontError::Io(err)
    }
}
impl std::fmt::Display for FontError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontError::ParseError(err) => write!(f, "Parse error: {}", err),
            FontError::ReadWriteError(err) => write!(f, "Read/write error: {}", err),
            FontError::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}
impl std::error::Error for FontError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font() {
        use crate::outlined::Icon;

        let mut font = Font::new_outlined().unwrap();
        let id = font.index_of(Icon::Neurology as u32).unwrap();
        let name = font.glyph_name(id).unwrap();
        assert_eq!(name, "neurology");

        let _ = font.bitmap_for(id).unwrap();
    }

    #[test]
    fn test_font_mapper() {
        let font = Font::new_outlined().unwrap();
        let mapper = FontMapper::new(&font).unwrap();
        let glyphs = mapper.all_chars().unwrap();
        assert!(!glyphs.is_empty());
    }

    #[test]
    fn test_glyph() {
        use crate::outlined::Icon;

        let font = Font::new_outlined().unwrap();
        let mapper = FontMapper::new(&font).unwrap();
        let glyph = mapper.find_glyph(Icon::Add as u32).unwrap().unwrap();
        assert_eq!(glyph.codepoint, Icon::Add as u32);
        assert_eq!(glyph.name, "add");
    }
}
