pub use fontdb;
use fontdb::{Query, ID};
use std::sync::{Arc, OnceLock, RwLock};

static FONT_HOLDER: OnceLock<RwLock<FontHolder>> = OnceLock::new();

/// Get font holder
pub fn font_holder<T, F: FnOnce(&FontHolder) -> T>(f: F) -> T {
	f(&FONT_HOLDER.get_or_init(Default::default).read().unwrap())
}

/// Get font holder mut
pub fn font_holder_mut<T, F: FnOnce(&mut FontHolder) -> T>(f: F) -> T {
	f(&mut FONT_HOLDER
		.get_or_init(Default::default) // RwLock is needed to have a mutable case
		.write()
		.unwrap())
}

static DEFAULT_FONT: OnceLock<FontRef> = OnceLock::new();

/// Set default font
pub fn set_default_font(font: FontRef) {
	_ = DEFAULT_FONT.set(font);
}

/// Get default font
pub fn default_font() -> Option<&'static FontRef> {
	DEFAULT_FONT.get()
}

/// Add a font at runtime
#[inline]
pub fn add_font<T: AsRef<[u8]> + Sync + Send + 'static>(font_bytes: T) {
	font_holder_mut(|holder| {
		holder
			.0
			.db_mut()
			.load_font_source(fontdb::Source::Binary(Arc::new(font_bytes)));
	});
}

/// Used with [`get`]
pub enum FontQuery<'a> {
	/// Get a [`FontRef`] from a [`&str`]
	Family(&'a str),
}
impl<'a> From<&'a str> for FontQuery<'a> {
	fn from(value: &'a str) -> Self {
		FontQuery::Family(value)
	}
}

/// Get a [`FontRef`] from a source
pub fn get<'a>(query: impl Into<FontQuery<'a>>) -> Option<FontRef> {
	font_holder(|holder| match query.into() {
		FontQuery::Family(name) => {
			let db = holder.0.db();
			let id = db.query(&Query {
				families: &[fontdb::Family::Name(name)],
				..Default::default()
			})?;
			let family = name.into();

			Some(FontRef { id, family })
		}
	})
}

/// A FontRef, linked to a font in the [`FontHolder`]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FontRef {
	///
	pub id: ID,
	///
	pub family: Arc<str>,
}
impl FontRef {
	/// Some or try get default
	pub fn or_default(v: Option<Self>) -> Option<Self> {
		v.or_else(|| default_font().cloned())
	}
}

/// Atlas of font
pub struct FontHolder(pub cosmic_text::FontSystem);
impl Default for FontHolder {
	fn default() -> Self {
		Self(cosmic_text::FontSystem::new())
	}
}
