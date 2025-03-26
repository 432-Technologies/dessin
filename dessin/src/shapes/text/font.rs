use super::FontWeight;
use ecow::EcoString;
use std::{
	collections::HashMap,
	fmt,
	ops::Deref,
	sync::{LazyLock, OnceLock, RwLock},
};

static FONT_HOLDER: OnceLock<RwLock<FontHolder>> = OnceLock::new();

fn font_holder<T, F: FnOnce(&FontHolder) -> T>(f: F) -> T {
	f(&FONT_HOLDER
		.get_or_init(|| RwLock::new(FontHolder::new()))
		.read()
		.unwrap())
}

fn font_holder_mut<T, F: FnOnce(&mut FontHolder) -> T>(f: F) -> T {
	f(&mut FONT_HOLDER
		.get_or_init(|| RwLock::new(FontHolder::new())) // RwLock is needed to have a mutable case
		.write()
		.unwrap())
}

#[inline]
pub fn get(idx: &FontRef) -> FontGroup<Font> {
	font_holder(|f| f.fonts[&idx.0].clone())
}

#[inline]
pub fn get_or_default(idx: Option<&FontRef>) -> FontGroup<Font> {
	idx.map(get).unwrap_or_else(|| get(&*DEFAULT_FONT))
}

#[inline]
pub fn fonts() -> HashMap<EcoString, FontGroup<Font>> {
	font_holder(|f| f.fonts.clone())
}

#[inline]
pub fn font_names() -> Vec<EcoString> {
	font_holder(|f| f.fonts.keys().cloned().collect())
}

#[inline]
pub fn add_font<S: Into<EcoString>>(font_name: S, font: FontGroup<Font>) -> FontRef {
	font_holder_mut(move |f| {
		let font_name = font_name.into();
		f.fonts.insert(font_name.clone(), font);
		FontRef(font_name)
	})
}

pub const DEFAULT_FONT: LazyLock<FontRef> = LazyLock::new(|| FontRef("Hyperlegible".into()));

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[repr(transparent)]
pub struct FontRef(EcoString);
impl FontRef {}
impl Deref for FontRef {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Default for FontRef {
	fn default() -> Self {
		DEFAULT_FONT.clone()
	}
}
impl fmt::Display for FontRef {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl<S: Into<EcoString>> From<S> for FontRef {
	fn from(value: S) -> Self {
		FontRef(value.into())
	}
}

#[derive(Clone)]
pub enum Font {
	OTF(Vec<u8>),
	TTF(Vec<u8>),
}

impl Font {
	pub fn as_bytes(&self) -> &[u8] {
		match self {
			Font::OTF(b) | Font::TTF(b) => b.as_slice(),
		}
	}
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
			regular: Font::OTF(
				include_bytes!("../../../Atkinson-Hyperlegible-Regular-102.otf").to_vec(),
			),
			bold: Some(Font::OTF(
				include_bytes!("../../../Atkinson-Hyperlegible-Bold-102.otf").to_vec(),
			)),
			italic: Some(Font::OTF(
				include_bytes!("../../../Atkinson-Hyperlegible-Italic-102.otf").to_vec(),
			)),
			bold_italic: Some(Font::OTF(
				include_bytes!("../../../Atkinson-Hyperlegible-BoldItalic-102.otf").to_vec(),
			)),
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
	fonts: HashMap<EcoString, FontGroup<Font>>,
}
impl FontHolder {
	fn new() -> Self {
		let mut fonts = HashMap::new();

		fonts.insert("Hyperlegible".into(), FontGroup::hyperlegible());

		FontHolder { fonts }
	}
}
