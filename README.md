# [dessin](https://docs.rs/dessin/)

**dessin is library aimed at building complex drawings, combine them, move them and export them as PDF or SVG.**

## Getting started

Add `dessin` and `dessin-svg` to your project dependencies

```
cargo add dessin dessin-svg
```

or if you need PDF:

```
cargo add dessin dessin-pdf
```

Documentation on [docs.rs](https://docs.rs/dessin/0.8.2-pre/)

### Overview

```rust
use dessin::prelude::*;
use dessin_svg::ToSVG;

#[derive(Default)]
struct MyShape {
  text: String
}
impl MyShape {
  fn say_this(&mut self, what: &str) {
    self.text = format!("{} And check this out: `{what}`", self.text);
  }
}
impl From<MyShape> for Shape {
  fn from(MyShape { text } : MyShape) -> Self {
    dessin!(Text: #(
      fill={Color::RED}
      {text}
    )).into()
  }
}

let dessin = dessin!(for x in 0..10: {
  let radius = x as f32 * 10.;

  dessin!([
    Circle: #(
      fill={Color::RED}
      {radius}
      translate={[x as f32 * 5., 10.]}
    ),
    Text: #(
      fill={Color::BLACK}
      font_size={10.}
      text={"Hi !"}
    ),
  ])
});

let dessin = dessin!([
  var(dessin): (
    scale={[2., 2.]}
  ),
  MyShape: (
    say_this={"Hello world"}
  ),
]);

let svg = dessin.to_svg().unwrap();

```
