# [Dessin](https://docs.rs/dessin/)

**Try out the new [API](https://github.com/432-Technologies/dessin/tree/v0.8-pre)**

Generate complex drawing for PDF, SVG, and many more to come ! 

### How ?

First, let's create a drawing and give it a bunch of things.
``` rust
let mut drawing = Drawing::empty().with_canvas_size(vec2(100., 100.));

drawing.add(
        Text::new("Hello World".to_owned())
            .at(vec2(50., 50.))
    )
    .add(
        Line::from(vec2(0., 0.)).to(vec2(100., 100.))
    )
    .add(
        Circle::new()
            .at(vec2(50., 50.)).with_radius(10.)
    )
    .add(
        Image::new(ImageFormat::PNG(include_bytes!("../rustacean-flat-happy.png").to_vec()))
            .at(vec2(50., 50.))
            .with_size(vec2(10., 10.))
    );
```
We can even add sub drawings to our drawing.
``` rust
let other_drawing = Drawing::empty()
    .with_canvas_size(vec2(210., 297.))
    .add(
        EmbeddedDrawing::new(drawing)
            .at(vec2(100., 100.))
            .with_size(vec2(10., 10.))
    );
```

Then, we export our drawing to [PDF](https://docs.rs/dessin-pdf/), [SVG](https://docs.rs/dessin-svg/), PNG, etc.
``` rust
use dessin_svg::ToSVG;

let svg = drawing.to_svg().unwrap();
dbg!(svg);
```
