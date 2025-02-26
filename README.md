# [dessin](https://docs.rs/dessin/)

**Try out the new [API](https://github.com/432-Technologies/dessin/tree/v0.8-pre)**

Generate complex drawing for PDF, SVG, and many more to come !

## Getting started

First of all, add `dessin` to build beautiful drawings !

```bash
cargo add dessin
```

Then, and either/or `dessin-svg`, `dessin-pdf`, `dessin-image` or `dessin-dioxus` depending of the export you need.

### Crates

- [dessin](./dessin/README.md): the main composing logic
- [dessin-macros](./dessin-macros/README.md): the macros `dessin!()` and `#[derive(Shape)]`
- [dessin-svg](./dessin-svg/README.md): the SVG exporter
- [dessin-pdf](./dessin-pdf/README.md): the PDF exporter
- [dessin-image](./dessin-image/README.md): the image exporter
- [dessin-dioxus](./dessin-dioxus/README.md): the Dioxus exporter

### Overview

```rust
use dessin::prelude::*;
use palette::{named, Srgb};
// We also reexport palette, in case you need it
// use dessin::palette::{named, Srgb};

#[derive(Default, Shape)]
struct MyShape {
	text: String,
}
impl MyShape {
	fn say_this(&mut self, what: &str) {
		self.text = format!("{} And check this out: `{what}`", self.text);
	}
}
impl From<MyShape> for Shape {
	fn from(MyShape { text }: MyShape) -> Self {
		dessin!(*Text(fill = Srgb::<f32>::from_format(named::RED).into_linear(), { text })).into()
	}
}

fn main() {
	let dessin = dessin!(for x in 0..10 {
		let radius = x as f32 * 10.;

		dessin!([
			*Circle(
				fill = Srgb::<f32>::from_format(named::RED).into_linear(),
				{ radius },
				translate = [x as f32 * 5., 10.],
			),
			*Text(fill = Srgb::<f32>::from_format(named::BLACK).into_linear(), font_size = 10., text = "Hi !",),
		])
	});

	let dessin = dessin!([
		{ dessin }(scale = [2., 2.]),
		MyShape(say_this = "Hello world"),
	]);

	// let svg = dessin_svg::to_string(&dessin).unwrap();
	// let pdf = dessin_pdf::to_string(&dessin).unwrap();
	// Etc...
}

```
