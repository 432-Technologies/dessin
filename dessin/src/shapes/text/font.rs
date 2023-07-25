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
pub fn fonts() -> HashMap<String, FontGroup<Font>> {
    font_holder(|f| f.fonts.clone())
}

#[inline]
pub fn font_names() -> Vec<String> {
    font_holder(|f| f.fonts.keys().cloned().collect())
}

#[inline]
pub fn add_font<S: Into<String>>(font_name: S, font: FontGroup<Font>) -> FontRef {
    font_holder_mut(move |f| {
        let font_name = font_name.into();
        f.fonts.insert(font_name.clone(), font);
        FontRef(font_name)
    })
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct FontRef(String);
impl FontRef {
    pub fn name(&self, font_weight: FontWeight) -> String {
        match font_weight {
            FontWeight::Regular => format!("{}Regular", self.0),
            FontWeight::Bold => format!("{}Bold", self.0),
            FontWeight::Italic => format!("{}Italic", self.0),
            FontWeight::BoldItalic => format!("{}BoldItalic", self.0),
        }
    }
}
impl Default for FontRef {
    fn default() -> Self {
		FontRef("Hyperlegible".to_string())
    }
}

impl<S: Into<String>> From<S> for FontRef {
    fn from(value: S) -> Self {
        FontRef(value.into())
    }
}

#[derive(Clone)]
pub enum Font {
    ByName(String),
    OTF(Vec<u8>),
    TTF(Vec<u8>),
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

	#[cfg(feature = "default-font")]
    pub fn hyperlegible() -> FontGroup<Font> {
        FontGroup {
            regular: Font::OTF(include_bytes!("../../../Atkinson-Hyperlegible-Regular-102.otf").to_vec()),
            bold: Some(Font::OTF(include_bytes!("../../../Atkinson-Hyperlegible-Bold-102.otf").to_vec())),
            italic: Some(Font::OTF(include_bytes!("../../../Atkinson-Hyperlegible-Italic-102.otf").to_vec())),
            bold_italic: Some(Font::OTF(include_bytes!("../../../Atkinson-Hyperlegible-BoldItalic-102.otf").to_vec())),
        }
    }
	#[cfg(not(feature = "default-font"))]
    pub fn hyperlegible() -> FontGroup<Font> {
        FontGroup {
            regular: Font::ByName("HyperlegibleRegular".to_string()),
            bold: Some(Font::ByName("HyperlegibleBold".to_string())),
            italic: Some(Font::ByName("HyperlegibleItalic".to_string())),
            bold_italic: Some(Font::ByName("HyperlegibleBoldItalic".to_string())),
        }
    }
}

pub struct FontHolder {
    fonts: HashMap<String, FontGroup<Font>>,
}
impl FontHolder {
    fn new() -> Self {
        let mut fonts = HashMap::new();

		fonts.insert("Hyperlegible".to_string(), FontGroup::hyperlegible());

        FontHolder { fonts }
    }
}
