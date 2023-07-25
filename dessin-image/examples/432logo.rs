mod logo_432;

use ::image::DynamicImage;
use dessin::prelude::*;
use dessin_image::ToImage;

fn main() {
    let dessin: Shape = dessin!(logo_432::Logo432: () -> ());

    let image: DynamicImage = dessin.rasterize().unwrap();

    image.into_rgba8().save("./res.png").unwrap();
}
