pub use fontdb;
use fontdb::{Query, ID};
use std::sync::{Arc, OnceLock, RwLock};

static FONT_HOLDER: OnceLock<RwLock<FontHolder>> = OnceLock::new();
pub fn font_holder<T, F: FnOnce(&FontHolder) -> T>(f: F) -> T {
	f(&FONT_HOLDER.get_or_init(Default::default).read().unwrap())
}
pub fn font_holder_mut<T, F: FnOnce(&mut FontHolder) -> T>(f: F) -> T {
	f(&mut FONT_HOLDER
		.get_or_init(Default::default) // RwLock is needed to have a mutable case
		.write()
		.unwrap())
}

static DEFAULT_FONT: OnceLock<FontRef> = OnceLock::new();
pub fn set_default_font(font: FontRef) {
	_ = DEFAULT_FONT.set(font);
}
pub fn default_font() -> Option<&'static FontRef> {
	DEFAULT_FONT.get()
}

#[inline]
pub fn add_font<T: AsRef<[u8]> + Sync + Send + 'static>(font_bytes: T) {
	font_holder_mut(|holder| {
		holder
			.0
			.db_mut()
			.load_font_source(fontdb::Source::Binary(Arc::new(font_bytes)));
	});
}

pub enum FontQuery<'a> {
	Family(&'a str),
}
impl<'a> From<&'a str> for FontQuery<'a> {
	fn from(value: &'a str) -> Self {
		FontQuery::Family(value)
	}
}

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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FontRef {
	pub id: ID,
	pub family: Arc<str>,
}

pub struct FontHolder(pub cosmic_text::FontSystem);
impl Default for FontHolder {
	fn default() -> Self {
		Self(cosmic_text::FontSystem::new())
	}
}
