use super::FontWeight;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

static FONT_HOLDER: OnceCell<Arc<RwLock<FontHolder>>> = OnceCell::new();

fn font_holder<T, F: FnOnce(&FontHolder) -> T>(f: F) -> T {
    f(&FONT_HOLDER
        .get_or_init(|| Arc::new(RwLock::new(FontHolder::new())))
        .read()
        .unwrap())
}

fn font_holder_mut<T, F: FnOnce(&mut FontHolder) -> T>(f: F) -> T {
    f(&mut FONT_HOLDER
        .get_or_init(|| Arc::new(RwLock::new(FontHolder::new())))
        .write()
        .unwrap())
}

#[inline]
pub fn get(idx: FontRef) -> FontGroup<Font> {
    font_holder(|f| f.fonts[&idx.0].clone())
}

#[inline]
pub fn fonts() -> HashMap<usize, FontGroup<Font>> {
    font_holder(|f| f.fonts.clone())
}

#[inline]
pub fn add_font(font: FontGroup<Font>) -> FontRef {
    font_holder_mut(move |f| {
        let id = f.fonts.len();
        f.fonts.insert(id, font);
        FontRef(id)
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct FontRef(usize);
impl Default for FontRef {
    fn default() -> Self {
        FontRef(0)
    }
}

impl From<FontGroup<Font>> for FontRef {
    fn from(font: FontGroup<Font>) -> Self {
        add_font(font)
    }
}

#[derive(Clone)]
pub enum Font {
    ByName(String),
    Bytes(Vec<u8>),
}

#[derive(Clone)]
pub struct FontGroup<T> {
    pub regular: T,
    pub bold: Option<T>,
    pub italic: Option<T>,
    pub bold_italic: Option<T>,
}
impl FontGroup<Font> {
    pub fn get(&self, font_weight: FontWeight) -> &Font {
        match font_weight {
            FontWeight::Regular => &self.regular,
            FontWeight::Bold => self.bold.as_ref().unwrap_or_else(|| &self.regular),
            FontWeight::BoldItalic => self.bold_italic.as_ref().unwrap_or_else(|| &self.regular),
            FontWeight::Italic => self.italic.as_ref().unwrap_or_else(|| &self.regular),
        }
    }

    pub fn helvetica() -> FontGroup<Font> {
        FontGroup {
            regular: Font::ByName("Helvetica".to_string()),
            bold: Some(Font::ByName("HelveticaBold".to_string())),
            italic: Some(Font::ByName("HelveticaOblique".to_string())),
            bold_italic: Some(Font::ByName("HelveticaBoldOblique".to_string())),
        }
    }
}

pub struct FontHolder {
    fonts: HashMap<usize, FontGroup<Font>>,
}
impl FontHolder {
    fn new() -> Self {
        let mut fonts = HashMap::new();
        fonts.insert(0, FontGroup::helvetica());
        FontHolder { fonts }
    }
}
