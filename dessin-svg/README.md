# [dessin](https://docs.rs/dessin/)

**dessin is library aimed at building complex drawings, combine them, move them and export them as PDF or SVG.**

## Getting started

Add `dessin` and `dessin-svg` to your project dependencies

```
cargo add dessin dessin-svg
```

Documentation on [docs.rs]()

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
  fn from(value: MyShape) -> Self {
    dessin!(Text: #(
      fill={Color::RED}
      text={value.text}
    )).into()
  }
}

let dessin = dessin!(for x in {0..10}: {
  let radius = x as f32 * 10.;

  dessin!(group: [
    { Circle: #(
      fill={Color::RED}
      radius={radius}
      translate={[x as f32 * 5., 10.]}
    ) }
    { Text: #(
      fill={Color::BLACK}
      font_size={10.}
      text={"Hi !"}
    ) }
  ])
});

let dessin = dessin!(group: [
  { use { dessin }: (
    scale={[2., 2.]}
  ) }
  { MyShape: (
    say_this={"Hello world"}
  ) }
]);

let svg = dessin.to_svg().unwrap();

```
